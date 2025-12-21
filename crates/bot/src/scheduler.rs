use bible::ReadingPlan;
use chrono::{Timelike, Utc};
use masterror::prelude::*;
use sqlx::PgPool;
use teloxide::prelude::*;
use tokio::time::{Duration, interval};

/// Run daily scheduler that sends verses at 7:00 AM (UTC+10 for Vladivostok)
pub async fn run_daily_scheduler(bot: Bot, pool: PgPool) {
    let mut interval = interval(Duration::from_secs(60)); // Check every minute

    loop {
        interval.tick().await;

        let now = Utc::now();
        // Vladivostok is UTC+10
        let vladivostok_hour = (now.hour() + 10) % 24;

        // Send at 7:00 AM Vladivostok time
        if vladivostok_hour == 7
            && now.minute() == 0
            && let Err(e) = send_daily_verses(&bot, &pool).await
        {
            tracing::error!("Failed to send daily verses: {}", e);
        }
    }
}

async fn send_daily_verses(bot: &Bot, pool: &PgPool) -> AppResult<()> {
    let reading_plan = ReadingPlan::new(pool.clone());

    let Some(reading) = reading_plan.get_today().await? else {
        tracing::warn!("No reading for today");
        return Ok(());
    };

    // Get all users with telegram_id who want notifications
    let users = sqlx::query!(
        r#"
        SELECT telegram_id
        FROM users
        WHERE telegram_id IS NOT NULL
        AND notification_enabled = true
        "#
    )
    .fetch_all(pool)
    .await?;

    let verses_text: String = reading
        .verses
        .iter()
        .map(|v| format!("{}:{} {}", v.chapter, v.verse, v.text))
        .collect::<Vec<_>>()
        .join("\n");

    let message = format!(
        "🌅 Доброе утро!\n\n📖 Чтение на сегодня (день {})\n\n{}\n\n\
        Что Господь говорит вам через этот отрывок?",
        reading.day_of_year, verses_text
    );

    for user in &users {
        if let Some(telegram_id) = user.telegram_id {
            let chat_id = ChatId(telegram_id);

            if let Err(e) = bot.send_message(chat_id, &message).await {
                tracing::warn!("Failed to send message to {}: {}", telegram_id, e);
            }

            // Small delay to avoid rate limiting
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }

    tracing::info!("Sent daily verses to {} users", users.len());

    Ok(())
}
