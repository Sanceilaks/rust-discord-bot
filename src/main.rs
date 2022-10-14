use serenity::{prelude::{GatewayIntents, Context, EventHandler}, Client, model::prelude::{interaction::Interaction, Ready, command::Command}};
use serenity::async_trait;
use songbird::SerenityInit;
use utils::chat;

mod commands;
mod utils;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            chat::wait_response(&ctx, &command).await;

            let content = match command.data.name.as_str() {
                "test_command" => commands::test_command::run(&ctx, &command, &command.data.options).await,
                "play" => commands::play::run(&ctx, &command, &command.data.options).await,
                "join" => commands::join::run(&ctx, &command, &command.data.options).await,
                "skip" => commands::skip::run(&ctx, &command, &command.data.options).await,
                "pause" => commands::pause::run(&ctx, &command, &command.data.options).await,
                "resume" => commands::resume::run(&ctx, &command, &command.data.options).await,
                _ => Err(Some("Такой комманды нет!".to_owned()))
            };

            if let Err(why) = content {
                if let Some(message) = why {
                    chat::edit_response(&ctx, &command, std::format!("Ошибка: {}", message.to_owned()).as_str()).await;
                }
            } else {
                if let Some(message) = content.unwrap() {
                    chat::edit_response(&ctx, &command, message.as_str()).await;
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is online now!", ready.user.name);

        Command::create_global_application_command(&ctx.http, |c| {
            commands::test_command::register(c)
        }).await.expect("Cannot create command!");

        Command::create_global_application_command(&ctx.http, |c| {
            commands::play::register(c)
        }).await.expect("Cannot create command!");

        Command::create_global_application_command(&ctx.http, |c| {
            commands::join::register(c)
        }).await.expect("Cannot create command!");

        Command::create_global_application_command(&ctx.http, |c| {
            commands::skip::register(c)
        }).await.expect("Cannot create command!");

        Command::create_global_application_command(&ctx.http, |c| {
            commands::resume::register(c)
        }).await.expect("Cannot create command!");

        Command::create_global_application_command(&ctx.http, |c| {
            commands::pause::register(c)
        }).await.expect("Cannot create command!");
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("RUST_DISCORD_TOKEN").expect("Token must be set");

    let mut client = Client::builder(token, GatewayIntents::GUILDS | GatewayIntents::GUILD_VOICE_STATES)
        .event_handler(Handler)
        .register_songbird()
        .await
        .expect("Error while creating client");
    
    if let Err(why) = client.start().await {
        println!("Client error {:#?}", why);
    }
}
