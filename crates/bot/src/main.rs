use bible::ReadingPlan;
use masterror::prelude::*;
use sqlx::postgres::PgPoolOptions;
use teloxide::{prelude::*, utils::command::BotCommands};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod scheduler;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Доступные команды:")]
enum Command {
    #[command(description = "Начать")]
    Start,
    #[command(description = "Стих на сегодня")]
    Today,
    #[command(description = "Привязать аккаунт")]
    Link,
    #[command(description = "Помощь")]
    Help
}

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into())
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let bot = Bot::from_env();

    // Start scheduler for daily verse sending
    let scheduler_pool = pool.clone();
    let scheduler_bot = bot.clone();
    tokio::spawn(async move {
        scheduler::run_daily_scheduler(scheduler_bot, scheduler_pool).await;
    });

    // Start bot
    let handler = Update::filter_message()
        .filter_command::<Command>()
        .endpoint(handle_command);

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![pool])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}

async fn handle_command(
    bot: Bot,
    msg: Message,
    cmd: Command,
    pool: sqlx::PgPool
) -> ResponseResult<()> {
    match cmd {
        Command::Start => {
            bot.send_message(
                msg.chat.id,
                "Добро пожаловать в Revelation! 🙏\n\n\
                Это приложение для изучения Библии и общения с братьями и сёстрами.\n\n\
                /today - получить стих на сегодня\n\
                /link - привязать Telegram к аккаунту\n\
                /help - помощь"
            )
            .await?;
        }
        Command::Today => {
            let reading_plan = ReadingPlan::new(pool);
            match reading_plan.get_today().await {
                Ok(Some(reading)) => {
                    let verses_text: String = reading
                        .verses
                        .iter()
                        .map(|v| format!("{}:{} {}", v.chapter, v.verse, v.text))
                        .collect::<Vec<_>>()
                        .join("\n");

                    let message = format!(
                        "📖 Чтение на сегодня (день {})\n\n{}\n\n\
                        Поделитесь своими мыслями в приложении!",
                        reading.day_of_year, verses_text
                    );

                    bot.send_message(msg.chat.id, message).await?;
                }
                Ok(None) => {
                    bot.send_message(msg.chat.id, "Чтение на сегодня не найдено.")
                        .await?;
                }
                Err(e) => {
                    tracing::error!("Error getting today's reading: {}", e);
                    bot.send_message(msg.chat.id, "Произошла ошибка. Попробуйте позже.")
                        .await?;
                }
            }
        }
        Command::Link => {
            let telegram_id = msg.from.as_ref().map(|u| u.id.0 as i64);

            if let Some(tg_id) = telegram_id {
                // Generate one-time link token
                let token = uuid::Uuid::now_v7();

                // TODO: Store token in Redis with expiration
                // For now, just show the link
                let app_url = std::env::var("APP_URL")
                    .unwrap_or_else(|_| "https://app.revelation.ru".into());

                bot.send_message(
                    msg.chat.id,
                    format!(
                        "Для привязки аккаунта перейдите по ссылке:\n\n{}/link?token={}\n\n\
                        Ссылка действительна 10 минут.",
                        app_url, token
                    )
                )
                .await?;
            }
        }
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
    }

    Ok(())
}
