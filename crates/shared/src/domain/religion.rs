use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Вероисповедание (Christianity, Islam, Judaism, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Religion {
    pub id:      Uuid,
    pub name:    String,
    pub name_ru: String
}

/// Конфессия внутри вероисповедания (Baptist, Orthodox, Catholic, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Confession {
    pub id:          Uuid,
    pub religion_id: Uuid,
    pub name:        String,
    pub name_ru:     String
}
