use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::{ChannelId};
use serenity::model::prelude::interaction::application_command::{CommandDataOption, CommandDataOptionValue, ApplicationCommandInteraction};
use serenity::prelude::Context;

use crate::utils::chat::{wait_response, edit_response, send_response};
use crate::utils::voice::{get_voice_channel_for_user, join_voice_channel};

pub async fn run(ctx: &Context, msg: &ApplicationCommandInteraction, _options: &[CommandDataOption]) -> Result<Option<String>, Option<String>> {
    let guild_id = msg.guild_id.unwrap();

    let user_channel_id = get_voice_channel_for_user(&ctx, &msg);
    let connect_target = match user_channel_id {
        Some(chl) => chl,
        None => {
            return Err(None);
        }
    };

    join_voice_channel(&ctx, connect_target, guild_id).await;

    edit_response(&ctx, &msg, "Готово!").await;

    Ok(None)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("join").description("Join to your voice channel")
}