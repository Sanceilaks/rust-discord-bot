use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::Message;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{CommandDataOption, CommandDataOptionValue, ApplicationCommandInteraction};
use serenity::prelude::Context;
use serenity::utils::{MessageBuilder, Content};

pub async fn run(ctx: &Context, msg: &ApplicationCommandInteraction, _options: &[CommandDataOption]) -> Result<Option<String>, Option<String>> {
    let manager = songbird::get(ctx)
        .await
        .unwrap()
        .clone();

        if let Some(mutex) = manager.get(msg.guild_id.unwrap()) {
            let handler = mutex.lock().await;

            if handler.queue().is_empty() {
                return Ok(Some("🚀Очередь пуста!".to_owned()));
            }

            let mut output: Vec<String> = Vec::new();
            for track in handler.queue().current_queue() {
                let track_metadata = track.metadata().clone();
                output.push(format!("{}) {} ({:?})", output.len() + 1, track_metadata.title.unwrap(),
                    track_metadata.duration.unwrap()));
            }

            Ok(Some(std::format!("🚀{}", output.join("\n"))))
        } else {
            Err(Some("Ты не в голосовом канале!".to_owned()))
        }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("queue").description("Show queue")
}