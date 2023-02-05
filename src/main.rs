use serenity::{prelude::{GatewayIntents, Context, EventHandler}, Client, model::prelude::{interaction::Interaction, Ready, command::Command, ResumedEvent}};
use serenity::async_trait;
use songbird::SerenityInit;
use utils::chat;

mod commands;
mod utils;

struct Handler;

macro_rules! create_command{
    ($register_function:expr, $ctx:expr) => {
        Command::create_global_application_command(&$ctx.http, |c| {
            $register_function(c)
        }).await.expect(&std::format!("Cannot create command: {}", stringify!($register_function)));
        println!("{} created!", stringify!($register_function));
    }
}

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
                "queue" => commands::queue::run(&ctx, &command, &command.data.options).await,
                _ => Err(Some("Такой комманды нет!".to_owned()))
            };

            if let Err(why) = content {
                if let Some(message) = why {
                    chat::edit_response(&ctx, &command, std::format!("🚀Ошибка: {}", message.to_owned()).as_str()).await;
                }
            } else {
                if let Some(message) = content.unwrap() {
                    chat::edit_response(&ctx, &command, message.as_str()).await;
                }
            }
        }
    }

	async fn resume(&self, ctx: Context, event: ResumedEvent) {
		println!("Resumed.\n{:?}", event.trace);
	}

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("🚀{}🚀 is online now!", ready.user.name);
        
        println!("--- Registring commands ---");

        create_command!(commands::test_command::register, ctx);
        create_command!(commands::play::register, ctx);
        create_command!(commands::join::register, ctx);
        create_command!(commands::skip::register, ctx);
        create_command!(commands::resume::register, ctx);
        create_command!(commands::pause::register, ctx);
        create_command!(commands::queue::register, ctx);

        println!("---------------------------");
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
