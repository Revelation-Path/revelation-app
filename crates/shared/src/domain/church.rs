use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Church {
    pub id:            Uuid,
    pub name:          String,
    pub city:          String,
    pub address:       Option<String>,
    pub confession_id: Uuid,
    pub admin_id:      Uuid,
    pub latitude:      Option<f64>,
    pub longitude:     Option<f64>,
    pub created_at:    DateTime<Utc>
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateChurch {
    #[validate(length(min = 2, max = 200))]
    pub name: String,

    #[validate(length(min = 2, max = 100))]
    pub city: String,

    #[validate(length(max = 500))]
    pub address: Option<String>,

    pub confession_id: Uuid,

    pub latitude:  Option<f64>,
    pub longitude: Option<f64>
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateChurch {
    #[validate(length(min = 2, max = 200))]
    pub name: Option<String>,

    #[validate(length(max = 500))]
    pub address: Option<String>,

    pub latitude:  Option<f64>,
    pub longitude: Option<f64>
}
