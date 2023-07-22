use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

use crate::config::*;

pub fn run(_options: &[CommandDataOption]) -> String {
    "Hey, I'm alive!".to_string()
}

pub fn register(
    command: &mut CreateApplicationCommand,
    x: SlashCommand,
) -> &mut CreateApplicationCommand {
    command.name(x.name).description("A ping command")
}
