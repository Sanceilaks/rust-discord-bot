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
            let mut handler = mutex.lock().await;
            let _result = handler.queue().skip();

            if let Some(track) = handler.queue().current_queue().get(1) {
                let metadata = track.metadata().clone();
                let title = metadata.title.unwrap();
                let lenght = metadata.duration.unwrap();
                return Ok(Some(std::format!("🚀Теперь играет: {} ({:?})", title, lenght)));
            }

            Ok(Some("🚀Очередь пуста!".to_owned()))
        } else {
            Err(Some("Ты не в голосовом канале!".to_owned()))
        }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("skip").description("Skip current track")
}