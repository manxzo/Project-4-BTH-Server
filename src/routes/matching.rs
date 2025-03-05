use crate::auth::Claims;
use crate::handlers::match_algo::calculate_match_score;
use crate::models::all_models::{MatchUser, MatchingRequest, MatchingStatus};
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use serde::Deserialize;
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;
pub async fn recommend_sponsors(pool: web::Data<PgPool>, req: HttpRequest) -> impl Responder {
    if let Some(claims) = req.extensions().get::<Claims>() {
        let user_query = "
            SELECT user_id as id, dob, location, interests, experience, available_days, languages
            FROM users WHERE user_id = $1";

        let user_result = sqlx::query_as::<_, MatchUser>(user_query)
            .bind(&claims.id)
            .fetch_one(pool.get_ref())
            .await;

        if let Ok(member) = user_result {
            if member.location.is_none()
                || member.interests.is_none()
                || member.experience.is_none()
                || member.available_days.is_none()
                || member.languages.is_none()
            {
                return HttpResponse::BadRequest()
                    .body("Complete your profile before requesting a sponsor.");
            }

            let sponsor_query = "
                SELECT user_id as id, dob, location, interests, experience, available_days, languages
                FROM users WHERE role = 'sponsor'";

            let sponsors_result = sqlx::query_as::<_, MatchUser>(sponsor_query)
                .fetch_all(pool.get_ref())
                .await;

            match sponsors_result {
                Ok(sponsors) => {
                    let mut sponsor_scores: Vec<(MatchUser, f32)> = sponsors
                        .into_iter()
                        .map(|sponsor| {
                            let score = calculate_match_score(&member, &sponsor);
                            (sponsor, score)
                        })
                        .collect();

                    sponsor_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

                    HttpResponse::Ok().json(sponsor_scores)
                }
                Err(_) => HttpResponse::InternalServerError().body("Failed to fetch sponsors."),
            }
        } else {
            HttpResponse::InternalServerError().body("Failed to fetch user data.")
        }
    } else {
        HttpResponse::Unauthorized().body("Authentication required")
    }
}

#[derive(Debug, Deserialize)]
pub struct SponsorRequest {
    pub sponsor_id: Uuid,
}

pub async fn request_sponsor(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    payload: web::Json<SponsorRequest>,
) -> impl Responder {
    if let Some(claims) = req.extensions().get::<Claims>() {
        // Check if the user has already sent a request to the same sponsor
        let check_request_query =
            "SELECT COUNT(*) FROM matching_requests WHERE member_id = $1 AND sponsor_id = $2";
        let existing_count: i64 = sqlx::query_scalar(check_request_query)
            .bind(&claims.id)
            .bind(&payload.sponsor_id)
            .fetch_one(pool.get_ref())
            .await
            .unwrap_or(0);

        if existing_count > 0 {
            return HttpResponse::Conflict().body("You have already requested this sponsor.");
        }

        // Ensure user has filled required fields before requesting
        let user_query = "
            SELECT location, interests, experience, available_days, languages
            FROM users WHERE user_id = $1";

        let user_result: Result<
            (
                Option<Value>,
                Option<Vec<String>>,
                Option<Vec<String>>,
                Option<Vec<String>>,
                Option<Vec<String>>,
            ),
            sqlx::Error,
        > = sqlx::query_as(user_query)
            .bind(&claims.id)
            .fetch_one(pool.get_ref())
            .await;

        match user_result {
            Ok((location, interests, experience, available_days, languages)) => {
                if location.is_none()
                    || interests.is_none()
                    || experience.is_none()
                    || available_days.is_none()
                    || languages.is_none()
                {
                    return HttpResponse::BadRequest()
                        .body("Complete your profile before requesting a sponsor.");
                }

                // Insert the matching request
                let insert_query = "
                    INSERT INTO matching_requests (member_id, sponsor_id, status, created_at)
                    VALUES ($1, $2, 'pending', NOW())
                    RETURNING matching_request_id, member_id, sponsor_id, status, created_at";

                let request_result = sqlx::query_as::<_, MatchingRequest>(insert_query)
                    .bind(&claims.id)
                    .bind(&payload.sponsor_id)
                    .fetch_one(pool.get_ref())
                    .await;

                match request_result {
                    Ok(request) => HttpResponse::Ok().json(request),
                    Err(_) => {
                        HttpResponse::InternalServerError().body("Failed to request sponsor.")
                    }
                }
            }
            Err(_) => HttpResponse::InternalServerError().body("Failed to fetch user data."),
        }
    } else {
        HttpResponse::Unauthorized().body("Authentication required")
    }
}

pub async fn check_matching_status(pool: web::Data<PgPool>, req: HttpRequest) -> impl Responder {
    if let Some(claims) = req.extensions().get::<Claims>() {
        let query = "SELECT * FROM matching_requests WHERE member_id = $1 AND status = 'pending'";

        let result = sqlx::query_as::<_, MatchingRequest>(query)
            .bind(&claims.id)
            .fetch_one(pool.get_ref())
            .await;

        match result {
            Ok(request) => HttpResponse::Ok().json(request),
            Err(_) => HttpResponse::NotFound().body("No pending matching requests."),
        }
    } else {
        HttpResponse::Unauthorized().body("Authentication required")
    }
}

#[derive(Debug, Deserialize)]
pub struct SponsorResponse {
    pub matching_request_id: Uuid,
    pub accept: bool, // true = accept, false = decline
}

pub async fn respond_to_matching_request(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    payload: web::Json<SponsorResponse>,
) -> impl Responder {
    if let Some(claims) = req.extensions().get::<Claims>() {
        let update_query = "
            UPDATE matching_requests 
            SET status = $1 
            WHERE matching_request_id = $2 AND sponsor_id = $3
            RETURNING matching_request_id, member_id, sponsor_id, status, created_at";

        let new_status = if payload.accept {
            MatchingStatus::Accepted
        } else {
            MatchingStatus::Declined
        };

        let result = sqlx::query_as::<_, MatchingRequest>(update_query)
            .bind(new_status.to_string())
            .bind(&payload.matching_request_id)
            .bind(&claims.id)
            .fetch_one(pool.get_ref())
            .await;

        match result {
            Ok(updated_request) => HttpResponse::Ok().json(updated_request),
            Err(_) => HttpResponse::InternalServerError().body("Failed to update request."),
        }
    } else {
        HttpResponse::Unauthorized().body("Authentication required")
    }
}

pub fn config_matching_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/matching")
            .route("/recommend", web::get().to(recommend_sponsors))
            .route("/request", web::post().to(request_sponsor))
            .route("/status", web::get().to(check_matching_status))
            .route("/respond", web::patch().to(respond_to_matching_request)),
    );
}
