use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOption, CommandDataOptionValue,
};
use serenity::model::prelude::ChannelId;
use serenity::prelude::Context;

use crate::utils;

pub async fn run(
    ctx: &Context,
    msg: &ApplicationCommandInteraction,
    _options: &[CommandDataOption],
) -> Result<Option<String>, Option<String>> {
    let option = _options.get(0).unwrap().resolved.as_ref().unwrap();

    if let CommandDataOptionValue::String(arg) = option {
        if !arg.starts_with("http") {
            return Err(Some("Это не ссылка".to_owned()));
        }

        let guild_id = msg.guild_id.unwrap();

        let target_voice = match utils::voice::get_voice_channel_for_user(&ctx, &msg) {
            None => {
                return Err(Some("Ты не в голосовом".to_owned()));
            }
            Some(id) => id,
        };

        let manager = songbird::get(ctx).await.unwrap().clone();

        match utils::voice::join_voice_channel(&ctx, target_voice, guild_id).await {
            Err(why) => {
                return Err(Some(why));
            }
            _ => {}
        }

        if let Some(handler_mutex) = manager.get(guild_id) {
            let mut handler = handler_mutex.lock().await;

            let stream = match songbird::ytdl(&arg).await {
                Ok(s) => s,
                Err(why) => {
                    println!("Cannot open stream: {:?}", why);
                    return Err(Some("Не могу получить контент".to_owned()));
                }
            };

            let metadata = stream.metadata.clone();

            let handle = handler.enqueue_source(stream.into());

            return Ok(Some(std::format!(
                "Включил {} ({:?}). Она {} в очереди",
                metadata.title.unwrap(),
                metadata.duration.unwrap(),
                handler.queue().len()
            )));
        }

        Ok(Some("Включаю".to_owned()))
    } else {
        Err(Some("Не могу найти url".to_owned()))
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("play")
        .description("Play music from youtube")
        .create_option(|o| {
            o.name("url")
                .description("description")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
