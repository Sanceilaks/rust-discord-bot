use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::Message;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{CommandDataOption, CommandDataOptionValue, ApplicationCommandInteraction};
use serenity::prelude::Context;
use serenity::utils::{MessageBuilder, Content};

pub async fn run(ctx: &Context, msg: &ApplicationCommandInteraction, _options: &[CommandDataOption]) -> Result<Option<String>, Option<String>> {
    let option = _options.get(0).unwrap().resolved.as_ref().unwrap();
    match option {
        CommandDataOptionValue::User(user, _member) => Ok(Some(format!("Вы пинганули {}", user.name.to_owned()))),
        _ => Err(Some("Вы никого не пинганули".to_owned())),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("test_command").description("just a test command").
    create_option(|o| {
        o.name("test_option").description("just a test option").kind(CommandOptionType::User).required(false)
    })
}