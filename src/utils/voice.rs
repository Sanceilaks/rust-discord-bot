use std::sync::Arc;

use serenity::{prelude::Context, model::prelude::{ChannelId, interaction::application_command::ApplicationCommandInteraction, GuildId}, http::Http, async_trait};
use songbird::{
    input::{
        self,
        restartable::Restartable,
    },
    Event,
    EventContext,
    EventHandler as VoiceEventHandler,
    SerenityInit,
    TrackEvent,
};


pub fn get_voice_channel_for_user(ctx: &Context, msg: &ApplicationCommandInteraction) -> Option<ChannelId> {
    let guild_id = msg.guild_id.unwrap();
    let guild = ctx.cache.guild(guild_id).unwrap();

    guild.voice_states.get(&msg.user.id)
        .and_then(|state| state.channel_id)
}

pub async fn join_voice_channel(ctx: &Context, target: ChannelId, guild: GuildId) -> Result<(), String> {
    let manager = songbird::get(ctx).await.unwrap().clone();
    let (mutex, result) = manager.join(guild, target).await;

    Ok(())
}