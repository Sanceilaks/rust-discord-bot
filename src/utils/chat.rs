use serenity::{prelude::Context, model::prelude::interaction::{application_command::ApplicationCommandInteraction}, utils::MessageBuilder};

pub async fn send_response(ctx: &Context, interaction: &ApplicationCommandInteraction, message: &str)
{
    interaction.create_interaction_response(&ctx.http, |resp| {
        resp.kind(serenity::model::prelude::interaction::InteractionResponseType::DeferredChannelMessageWithSource)
        .interaction_response_data(|data| data.content(message))
    }).await.expect("Cannot create response");
}

pub async fn edit_response(ctx: &Context, interaction: &ApplicationCommandInteraction, message: &str)
{
    interaction.edit_original_interaction_response(&ctx.http, |data| {
        data.content(message)
    }).await.unwrap();
}

pub async fn wait_response(ctx: &Context, interaction: &ApplicationCommandInteraction)
{
    interaction.create_interaction_response(&ctx.http, |resp| {
        resp.kind(serenity::model::prelude::interaction::InteractionResponseType::DeferredChannelMessageWithSource)
    }).await.expect("Cannot create response");
}
