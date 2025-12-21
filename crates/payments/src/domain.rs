use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "payment_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Refunded
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "payment_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum PaymentType {
    Donation,     // Пожертвование церкви
    Subscription, // Подписка на приложение
    OneTime       // Разовый платеж
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id:                  Uuid,
    pub user_id:             Uuid,
    pub church_id:           Option<Uuid>,
    pub payment_type:        PaymentType,
    pub amount:              i64, // in kopeks/cents
    pub currency:            String,
    pub status:              PaymentStatus,
    pub provider:            String,
    pub provider_payment_id: Option<String>,
    pub created_at:          DateTime<Utc>,
    pub completed_at:        Option<DateTime<Utc>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePayment {
    pub user_id:      Uuid,
    pub church_id:    Option<Uuid>,
    pub payment_type: PaymentType,
    pub amount:       i64,
    pub currency:     String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentCard {
    pub id:               Uuid,
    pub user_id:          Uuid,
    pub last_four:        String,
    pub brand:            String,
    pub exp_month:        i16,
    pub exp_year:         i16,
    pub is_default:       bool,
    pub provider_card_id: String,
    pub created_at:       DateTime<Utc>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddCardRequest {
    pub user_id:        Uuid,
    pub provider_token: String // Token from payment provider SDK
}
