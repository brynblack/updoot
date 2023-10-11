use crate::commands::age;
use std::env;

use log::info;
use poise::{
    serenity_prelude::{self as serenity, ReactionType},
    Event,
};

mod commands;

pub struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub async fn run() {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age()],
            event_handler: |_ctx, event, _framework, _data| {
                Box::pin(event_handler(_ctx, event, _framework, _data))
            },
            ..Default::default()
        })
        .token(env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN value"))
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}
async fn event_handler(
    ctx: &serenity::Context,
    event: &Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        Event::Ready { data_about_bot } => {
            info!("logged in as {}", data_about_bot.user.name);
            info!("listening for events");
        }
        Event::Message { new_message } => {
            if new_message.embeds.is_empty() {
                return Ok(());
            }

            new_message
                .react(ctx, ReactionType::Unicode(String::from("❤️")))
                .await?;

            new_message
                .react(ctx, ReactionType::Unicode(String::from("💔")))
                .await?;
        }
        _ => {}
    }
    Ok(())
}
