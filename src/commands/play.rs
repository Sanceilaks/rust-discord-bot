use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOption, CommandDataOptionValue,
};
use serenity::prelude::Context;

use crate::utils::{self, yandex_music};

pub async fn run(
    ctx: &Context,
    msg: &ApplicationCommandInteraction,
    _options: &[CommandDataOption],
) -> Result<Option<String>, Option<String>> {
    let option = _options.get(0).unwrap().resolved.as_ref().unwrap();

    if let CommandDataOptionValue::String(arg) = option {
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

            let stream = match arg.starts_with("http") {
                true => {
                    if arg.contains("yandex") {
                        yandex_music::get_track(&arg).await.unwrap()
                    } else {
                        match songbird::input::ytdl(arg).await {
                            Ok(res) => res,
                            Err(why) => {
                                println!("Cannot cannot download url {}", arg.to_owned());
                                return Err(Some(
                                    format!("Не могу найти {}", arg.to_owned()).to_owned(),
                                ));
                            }
                        }
                    }
                }
                false => match songbird::input::ytdl_search(arg).await {
                    Ok(res) => res,
                    Err(why) => {
                        return Err(Some(format!("Не могу найти {}", arg.to_owned()).to_owned()));
                    }
                },
            };

            let metadata = stream.metadata.clone();

            let handle = handler.enqueue_source(stream.into());

            if let Some(title) = metadata.title {
                return Ok(Some(std::format!(
                    "🚀Включил {} ({:?}). Она {} в очереди",
                    title,
                    metadata.duration.unwrap(),
                    handler.queue().len()
                )));
            } else {
                return Ok(Some(std::format!("🚀Включил")));
            }
        }

        Ok(Some("🚀Включаю".to_owned()))
    } else {
        Err(Some("Не могу найти url".to_owned()))
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("play")
        .description("Play music from youtube")
        .create_option(|o| {
            o.name("url_or_name")
                .description("url to song or song name")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
