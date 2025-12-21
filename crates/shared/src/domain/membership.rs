use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "db", derive(sqlx::Type))]
#[cfg_attr(
    feature = "db",
    sqlx(type_name = "church_role", rename_all = "snake_case")
)]
pub enum ChurchRole {
    Guest,  // Гость
    Member, // Член церкви
    Deacon, // Дьякон
    Elder,  // Пресвитер
    Pastor, // Пастор
    Admin   // Администратор в приложении
}

impl ChurchRole {
    pub fn can_manage_content(&self) -> bool {
        matches!(self, Self::Pastor | Self::Admin)
    }

    pub fn can_manage_members(&self) -> bool {
        matches!(self, Self::Pastor | Self::Admin | Self::Elder)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Membership {
    pub id:        Uuid,
    pub user_id:   Uuid,
    pub church_id: Uuid,
    pub role:      ChurchRole,
    pub joined_at: DateTime<Utc>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinChurch {
    pub church_id: Uuid,
    pub role:      ChurchRole
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMemberRole {
    pub user_id: Uuid,
    pub role:    ChurchRole
}
