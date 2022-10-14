use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::Message;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{CommandDataOption, CommandDataOptionValue, ApplicationCommandInteraction};
use serenity::prelude::Context;
use serenity::utils::{MessageBuilder, Content};
use songbird::tracks::TrackResult;

pub async fn run(ctx: &Context, msg: &ApplicationCommandInteraction, _options: &[CommandDataOption]) -> Result<Option<String>, Option<String>> {
    let manager = songbird::get(ctx)
        .await
        .unwrap()
        .clone();
    
        if let Some(mutex) = manager.get(msg.guild_id.unwrap()) {
            let mut handler = mutex.lock().await;
            if handler.queue().pause().is_err() {
                return Err(Some("Нет треков в очереди".to_owned()));
            }
            Ok(Some("Готово".to_owned()))
        } else {
            Err(Some("Ты не в голосовом канале!".to_owned()))
        }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("pause").description("Pause bot")
}