use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "db", derive(sqlx::Type))]
#[cfg_attr(feature = "db", sqlx(type_name = "gender", rename_all = "snake_case"))]
pub enum Gender {
    Male,
    Female
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id:            Uuid,
    pub name:          Option<String>,
    pub gender:        Option<Gender>,
    pub birth_date:    Option<NaiveDate>,
    pub confession_id: Option<Uuid>,

    // Auth bindings (progressive auth)
    pub email:       Option<String>,
    pub phone:       Option<String>,
    pub telegram_id: Option<i64>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateUser {
    pub id: Uuid // generated on client
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateUserProfile {
    #[validate(length(min = 2, max = 100))]
    pub name:          Option<String>,
    pub gender:        Option<Gender>,
    pub birth_date:    Option<NaiveDate>,
    pub confession_id: Option<Uuid>
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct BindTelegram {
    pub telegram_id: i64
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct BindEmail {
    #[validate(email)]
    pub email: String
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct BindPhone {
    #[validate(length(min = 10, max = 15))]
    pub phone: String
}
