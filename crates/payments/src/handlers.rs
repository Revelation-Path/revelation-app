use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post}
};
use masterror::prelude::*;
use uuid::Uuid;

use crate::{
    AppState,
    domain::{AddCardRequest, CreatePayment, Payment, PaymentCard, PaymentStatus}
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_payment))
        .route("/{payment_id}", get(get_payment))
        .route("/user/{user_id}", get(get_user_payments))
        .route("/cards", post(add_card))
        .route("/cards/user/{user_id}", get(get_user_cards))
        .route("/cards/{card_id}", axum::routing::delete(delete_card))
        .route("/webhook", post(payment_webhook))
}

async fn create_payment(
    State(state): State<AppState>,
    Json(payload): Json<CreatePayment>
) -> AppResult<Json<Payment>> {
    let id = Uuid::now_v7();

    let payment = sqlx::query_as!(
        Payment,
        r#"
        INSERT INTO payments (id, user_id, church_id, payment_type, amount, currency, status, provider)
        VALUES ($1, $2, $3, $4, $5, $6, 'pending', 'stripe')
        RETURNING
            id,
            user_id,
            church_id,
            payment_type as "payment_type: _",
            amount,
            currency,
            status as "status: _",
            provider,
            provider_payment_id,
            created_at,
            completed_at
        "#,
        id,
        payload.user_id,
        payload.church_id,
        payload.payment_type as _,
        payload.amount,
        payload.currency
    )
    .fetch_one(&state.pool)
    .await?;

    // TODO: Create payment intent with provider (Stripe, YooKassa, etc.)

    Ok(Json(payment))
}

async fn get_payment(
    State(state): State<AppState>,
    Path(payment_id): Path<Uuid>
) -> AppResult<Json<Payment>> {
    let payment = sqlx::query_as!(
        Payment,
        r#"
        SELECT
            id,
            user_id,
            church_id,
            payment_type as "payment_type: _",
            amount,
            currency,
            status as "status: _",
            provider,
            provider_payment_id,
            created_at,
            completed_at
        FROM payments
        WHERE id = $1
        "#,
        payment_id
    )
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(payment))
}

async fn get_user_payments(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>
) -> AppResult<Json<Vec<Payment>>> {
    let payments = sqlx::query_as!(
        Payment,
        r#"
        SELECT
            id,
            user_id,
            church_id,
            payment_type as "payment_type: _",
            amount,
            currency,
            status as "status: _",
            provider,
            provider_payment_id,
            created_at,
            completed_at
        FROM payments
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
        user_id
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(payments))
}

async fn add_card(
    State(_state): State<AppState>,
    Json(_payload): Json<AddCardRequest>
) -> AppResult<Json<PaymentCard>> {
    Err(AppError::internal("Card tokenization not implemented yet"))
}

async fn get_user_cards(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>
) -> AppResult<Json<Vec<PaymentCard>>> {
    let cards = sqlx::query_as!(
        PaymentCard,
        r#"
        SELECT
            id,
            user_id,
            last_four,
            brand,
            exp_month,
            exp_year,
            is_default,
            provider_card_id,
            created_at
        FROM payment_cards
        WHERE user_id = $1
        ORDER BY is_default DESC, created_at DESC
        "#,
        user_id
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(cards))
}

async fn delete_card(State(state): State<AppState>, Path(card_id): Path<Uuid>) -> AppResult<()> {
    sqlx::query!("DELETE FROM payment_cards WHERE id = $1", card_id)
        .execute(&state.pool)
        .await?;

    Ok(())
}

#[derive(serde::Deserialize)]
pub struct WebhookPayload {
    event_type: String,
    payment_id: String,
    status:     String
}

async fn payment_webhook(
    State(state): State<AppState>,
    Json(payload): Json<WebhookPayload>
) -> AppResult<()> {
    // TODO: Verify webhook signature

    let status = match payload.status.as_str() {
        "succeeded" => PaymentStatus::Completed,
        "failed" => PaymentStatus::Failed,
        _ => return Ok(())
    };

    let completed_at = if status == PaymentStatus::Completed {
        Some(chrono::Utc::now())
    } else {
        None
    };

    sqlx::query!(
        r#"
        UPDATE payments SET
            status = $2,
            completed_at = $3
        WHERE provider_payment_id = $1
        "#,
        payload.payment_id,
        status as _,
        completed_at
    )
    .execute(&state.pool)
    .await?;

    Ok(())
}
