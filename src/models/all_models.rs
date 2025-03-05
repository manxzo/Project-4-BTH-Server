use chrono::{NaiveDate, NaiveDateTime};
use serde::{Serialize, Deserialize};
use sqlx::{Decode, FromRow};
use uuid::Uuid;
use serde_json::Value;
use strum_macros::{EnumString,Display};



#[derive(Debug, Serialize, Deserialize, sqlx::Type,Display,EnumString,PartialEq)]
//  USER & AUTHENTICATION STRUCTS
#[sqlx(type_name = "user_role", rename_all = "lowercase")] 
pub enum UserRole {
    Member,
    Sponsor,
    Admin,
}
#[derive(Debug, Serialize, Deserialize, FromRow,Decode)]
pub struct User {
    pub user_id: Uuid, 
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: UserRole, 
    pub banned_until: Option<NaiveDateTime>, 
    pub avatar_url: String, 
    pub created_at: NaiveDateTime, 
    pub dob: NaiveDate, 
    pub user_profile: String, 
    pub bio: Option<String>, 
    pub email_verified: bool, 
    pub email_verification_token: Option<Uuid>, 
    pub forgot_password_token: Option<Uuid>, 
    pub forgot_password_expires_at: Option<NaiveDateTime>, 
    pub location: Option<Value>, 
    pub interests: Option<Vec<String>>,
    pub experience: Option<Vec<String>>,
    pub available_days: Option<Vec<String>>,
    pub languages: Option<Vec<String>>,
    pub privacy:bool,
}


//  SPONSOR APPLICATION

#[derive(Debug, Serialize, Deserialize, sqlx::Type ,PartialEq,Display,EnumString)]
#[sqlx(type_name = "application_status", rename_all = "lowercase")] 
pub enum ApplicationStatus {
    Pending,
    Approved,
    Rejected,
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SponsorApplication {
    pub application_id: Uuid, 
    pub user_id: Uuid, 
    pub status: ApplicationStatus, 
    pub application_info: String, 
    pub reviewed_by: Option<Uuid>, 
    pub admin_comments: Option<String>, 
    pub created_at: NaiveDateTime, 
}
//  LOCATION STRUCT (For Matching & Users)

#[derive(Debug, Serialize, Deserialize, Clone,sqlx::Type)]
#[sqlx(type_name="jsonb")]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub city: Option<String>,
    pub country: Option<String>,
}

//  MATCHING REQUESTS

#[derive(Debug, Serialize, Deserialize,sqlx::Type,Display,EnumString,PartialEq)]
#[sqlx(type_name = "matching_status", rename_all = "lowercase")] 
pub enum MatchingStatus {
    Pending,
    Accepted,
    Declined,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MatchingRequest {
    pub matching_request_id: Uuid, 
    pub member_id: Uuid, 
    pub sponsor_id: Option<Uuid>,
    pub status: MatchingStatus, 
    pub match_score: Option<f32>,
    pub created_at: NaiveDateTime,
}


#[derive(Debug, Clone,Serialize,Deserialize,Decode,FromRow)]
pub struct MatchUser {
    pub id: Uuid,
    pub dob: NaiveDate,
    pub location: Option<Location>, 
    pub interests: Option<Vec<String>>,
    pub experience: Option<Vec<String>>,
    pub available_days: Option<Vec<String>>,
    pub languages: Option<Vec<String>>,
}


//  1-1 MESSAGES & GROUP CHATS

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub message_id: Uuid, 
    pub sender_id: Uuid, 
    pub receiver_id: Uuid, 
    pub content: String, 
    pub timestamp: NaiveDateTime, 
    pub flagged: bool, 
    pub deleted: bool, 
    pub edited: bool, 
    pub seen_at: Option<NaiveDateTime>
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct GroupChatMessage {
    pub group_chat_message_id: Uuid, 
    pub group_chat_id: Uuid, 
    pub sender_id: Uuid, 
    pub content: String, 
    pub timestamp: NaiveDateTime, 
    pub flagged: bool, 
    pub deleted: bool, 
    pub edited: bool,
}


//  GROUP CHATS & MEMBERS

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct GroupChat {
    pub group_chat_id: Uuid, 
    pub created_at: NaiveDateTime, 
    pub flagged:bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct GroupChatMember {
    pub group_chat_id: Uuid, 
    pub user_id: Uuid, 
}


//  GROUP MEETINGS & PARTICIPANTS

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct GroupMeeting {
    pub meeting_id: Uuid,
    pub group_chat_id:Option<Uuid>, 
    pub host_id: Uuid, 
    pub title: String, 
    pub description: Option<String>, 
    pub scheduled_time: NaiveDateTime, 
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MeetingParticipant {
    pub meeting_id: Uuid, 
    pub user_id: Uuid, 
}


//  RESOURCE LIBRARY

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Resource {
    pub resource_id: Uuid, 
    pub contributor_id: Uuid, 
    pub title: String, 
    pub content: String, 
    pub approved: bool, 
    pub created_at: NaiveDateTime, 
}


//  REPORTS & FLAGGED CONTENT

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "reported_type", rename_all = "lowercase")] 
pub enum ReportedType {
    Message,
    GroupChatMessage,
    GroupChat,
    User,
    Post,
    Comment
}
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "report_status", rename_all = "lowercase")] 
pub enum ReportStatus {
    Pending,
    Resolved,
    Reviewed,
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Report {
    pub report_id: Uuid, 
    pub reporter_id: Uuid, 
    pub reported_user_id: Uuid, 
    pub reason: String, 
    pub reported_type: ReportedType,
    pub reported_item_id: Uuid, 
    pub status: ReportStatus, 
    pub reviewed_by: Option<Uuid>, 
    pub resolved_at: Option<NaiveDateTime>, 
    pub created_at: NaiveDateTime, 
}


//  POSTS & COMMENTS
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Post {
    pub post_id: Uuid, 
    pub author_id: Uuid, 
    pub content: String, 
    pub flagged: bool, 
    pub created_at: NaiveDateTime,  
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PostLike {
    pub post_id: Uuid,  
    pub user_id: Uuid,  
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub comment_id: Uuid, 
    pub post_id: Uuid, 
    pub author_id: Uuid, 
    pub content: String, 
    pub created_at: NaiveDateTime, 
    pub parent_comment_id: Option<Uuid>, 
}

//NOTIFICATIONS/ANNOUNCEMENTS
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "announcement_type", rename_all = "lowercase")] 
pub enum AnnouncementType {
    General,
    Recommended,
    MeetingReminder,
    Invitation
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Announcement {
    pub announcement_type:AnnouncementType,
    pub announcement_target_id:Option<Uuid>,
    pub message:String,
    pub created_at:NaiveDateTime,
}