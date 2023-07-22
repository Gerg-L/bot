use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use clap::Parser;
use regex::Regex;
use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::prelude::command::CommandOptionType;

use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::{Client, Context, EventHandler, GatewayIntents};

use bot::commands;
use bot::config::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
}
fn find_config() -> PathBuf {
    let cli = Cli::parse();
    return match cli.config {
        Some(x) => x,
        None => match env::current_dir() {
            Ok(p) => p.join("config.yml"),
            Err(_) => panic!("Couldn't get CWD and a config wasn't passed via CLI flags"),
        },
    };
}

fn read_config(p: &Path) -> String {
    return match fs::read_to_string(p) {
        Ok(x) => x,
        Err(_) => panic!("File \"{}\" does not exist!", p.to_string_lossy()),
    };
}

fn parse_config(s: String) -> Config {
    match serde_yaml::from_str(&s) {
        Ok(x) => x,
        Err(_) => panic!("Failed to deserialize config"),
    }
}

fn get_token() -> String {
    match env::var("DISCORD_TOKEN") {
        Ok(x) => x,
        Err(_) => {
            panic!("discord_token is set to an empty string and $DISCORD_TOKEN doesn't exist")
        }
    }
}

fn get_guild_id() -> GuildId {
    GuildId(match env::var("GUILD_ID") {
        Ok(x) => x.parse().expect("yeet"),
        Err(_) => {
            panic!("$GUILD_ID must be set!")
        }
    })
}

struct Handler {
    config: Config,
}
#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {}", command.data.name.as_str());

            let mut content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                "id" => commands::id::run(&command.data.options),
                "attachmentinput" => commands::attachmentinput::run(&command.data.options),
                _ => "not implemented :(".to_string(),
            };
            for x in self.config.commands.iter() {
                if x.name == command.data.name.as_str() {
                    for i in command.data.options.iter() {
                        for why in x.sub_commands.iter() {
                            if why.name == i.name{
                                content = why.response.clone();
                            }
                        };
                        

                        
                    }
                }
            }

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = get_guild_id();
        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::ping::register(command))
                .create_application_command(|command| commands::id::register(command))
                .create_application_command(|command| commands::attachmentinput::register(command));

            for x in self.config.commands.iter() {
                commands.create_application_command(|command| {
                    command.name(&x.name).description("Generated command");
                    for i in x.sub_commands.iter() {
                        command.create_option(|option| {
                            option
                                .name(&i.name)
                                .kind(CommandOptionType::SubCommand)
                                .description("required")
                                .required(false)
                        });
                    }
                    return command;
                });
            }
            return commands;
        })
        .await;

        println!(
            "I now have the following guild slash commands: {:#?}",
            commands
        );
    }
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        };
        let check = check_regex(&self.config, &msg).await;
        if check.is_some() {
            msg.reply_ping(&ctx.http, check.unwrap()).await.unwrap();
        }

        if msg.content == "!hello" {
            let msg = msg
                .channel_id
                .send_message(&ctx.http, |m| m.content("Hello, World!"))
                .await;

            if let Err(why) = msg {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}
async fn check_regex(config: &Config, message: &Message) -> Option<String> {
    for x in config.regex_pairs.iter() {
        let regex = Regex::new(&x.regex).unwrap();
        if regex.is_match(&message.content) {
            return Some(x.response.clone());
        };
    }
    return None;
}
#[tokio::main]
async fn main() {
    let found_config = find_config();
    println!("Found config");
    let read_config = read_config(&found_config);
    println!("Read config");
    let parsed_config = parse_config(read_config);
    println!("Parsed config");
    let token = get_token();
    println!("Found token");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler {
            config: parsed_config,
        })
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
