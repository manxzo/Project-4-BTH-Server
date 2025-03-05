use actix_web::{web, HttpRequest, HttpResponse, Responder, HttpMessage};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use uuid::Uuid;
use crate::auth::Claims;
use crate::models::all_models::ApplicationStatus;

#[derive(Debug, Deserialize,Serialize)]
pub struct SponsorApplicationRequest {
    pub application_info: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct SponsorApplication {
    pub application_id: Uuid,
    pub user_id: Uuid,
    pub status: ApplicationStatus,
    pub application_info: String,
    pub reviewed_by: Option<Uuid>,
    pub admin_comments: Option<String>,
    pub created_at: NaiveDateTime,
}

pub async fn submit_sponsor_application(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    payload: web::Json<SponsorApplicationRequest>,
) -> impl Responder {
    if let Some(claims) = req.extensions().get::<Claims>() {
        // Check if the user has already submitted an application
        let check_query = "SELECT COUNT(*) FROM sponsor_applications WHERE user_id = $1";

        let existing_count: i64 = sqlx::query_scalar(check_query)
            .bind(&claims.id)
            .fetch_one(pool.get_ref())
            .await
            .unwrap_or(0);

        if existing_count > 0 {
            return HttpResponse::Conflict().body("You have already submitted an application.");
        }

        // Insert the application into the database
        let insert_query = "
            INSERT INTO sponsor_applications (user_id, status, application_info, created_at)
            VALUES ($1, $2, $3, NOW())
            RETURNING application_id, user_id, status, application_info, reviewed_by, admin_comments, created_at";
        
        let application_result = sqlx::query_as::<_, SponsorApplication>(insert_query)
            .bind(&claims.id)
            .bind(ApplicationStatus::Pending) 
            .bind(&payload.application_info)
            .fetch_one(pool.get_ref())
            .await;

        match application_result {
            Ok(application) => HttpResponse::Ok().json(application),
            Err(_) => HttpResponse::InternalServerError().body("Failed to submit application"),
        }
    } else {
        HttpResponse::Unauthorized().body("Authentication required")
    }
}
pub async fn check_sponsor_application_status(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> impl Responder {
    if let Some(claims) = req.extensions().get::<Claims>() {
        let query = "SELECT * FROM sponsor_applications WHERE user_id = $1";

        let result = sqlx::query_as::<_, SponsorApplication>(query)
            .bind(&claims.id)
            .fetch_one(pool.get_ref())
            .await;

        match result {
            Ok(application) => HttpResponse::Ok().json(application),
            Err(_) => HttpResponse::NotFound().body("No sponsor application found."),
        }
    } else {
        HttpResponse::Unauthorized().body("Authentication required")
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateSponsorApplicationRequest {
    pub application_info: String,
}

pub async fn update_sponsor_application(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    payload: web::Json<UpdateSponsorApplicationRequest>,
) -> impl Responder {
    if let Some(claims) = req.extensions().get::<Claims>() {
        // Check if application exists
        let check_query = "SELECT status FROM sponsor_applications WHERE user_id = $1";
        
        let result: Result<ApplicationStatus, sqlx::Error> = sqlx::query_scalar(check_query)
            .bind(&claims.id)
            .fetch_one(pool.get_ref())
            .await;

        match result {
            Ok(status) => {
                if status == ApplicationStatus::Approved {
                    return HttpResponse::Forbidden().body("You cannot update an approved application.");
                }

                let update_query = "
                    UPDATE sponsor_applications 
                    SET application_info = $1, status = CASE WHEN status = 'rejected' THEN 'pending' ELSE status END 
                    WHERE user_id = $2
                    RETURNING application_id, user_id, status, application_info, reviewed_by, admin_comments, created_at";

                let updated_result = sqlx::query_as::<_, SponsorApplication>(update_query)
                    .bind(&payload.application_info)
                    .bind(&claims.id)
                    .fetch_one(pool.get_ref())
                    .await;

                match updated_result {
                    Ok(updated_application) => HttpResponse::Ok().json(updated_application),
                    Err(_) => HttpResponse::InternalServerError().body("Failed to update application."),
                }
            }
            Err(_) => HttpResponse::NotFound().body("No sponsor application found."),
        }
    } else {
        HttpResponse::Unauthorized().body("Authentication required")
    }
}

pub async fn delete_sponsor_application(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> impl Responder {
    if let Some(claims) = req.extensions().get::<Claims>() {
        let delete_query = "DELETE FROM sponsor_applications WHERE user_id = $1";

        let result = sqlx::query(delete_query)
            .bind(&claims.id)
            .execute(pool.get_ref())
            .await;

        match result {
            Ok(_) => HttpResponse::Ok().body("Sponsor application deleted successfully."),
            Err(_) => HttpResponse::InternalServerError().body("Failed to delete sponsor application."),
        }
    } else {
        HttpResponse::Unauthorized().body("Authentication required")
    }
}

pub fn config_sponsor_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/sponsor") 
            .route("/apply", web::post().to(submit_sponsor_application)) 
            .route("/check", web::get().to(check_sponsor_application_status))
            .route("/update",web::patch().to(update_sponsor_application))
            .route("/delete",web::delete().to(delete_sponsor_application)) 
    );
}
