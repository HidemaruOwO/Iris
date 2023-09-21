mod commands;

use std::env;
use std::collections::HashSet;

use log::LevelFilter;
use serenity::async_trait;
use serenity::framework::standard::{
    help_commands,
    macros::{group, help},
    Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::framework::StandardFramework;
use serenity::model::{channel::Message, gateway::Ready, id::UserId };
use serenity::prelude::{Client, Context, EventHandler};
use logger_rs::*;

use commands::hello::*;
// ------------------------------
#[group]
#[description("General commands")]
#[summary("General")]
#[commands(hello)]
struct General;
// ------------------------------
#[tokio::main]
async fn main() {
    let filters: [(Option<&str>, LevelFilter); 2] = [
        (None, LevelFilter::Warn),
        (Some("iris"), LevelFilter::Info),
    ];
    init_logger(&filters);
    info!("🚀 Starting...\n");
    let config = get_config();
    info!("🔎 Checking config...");
    // let owner_name = 
    info!("Bot owner ID: {}", config.get_owner_id());
    info!("Bot prefix: {}\n", config.get_prefix());
    info!("🔑 Connecting to Discord...");
    let framework = StandardFramework::new()
        .configure(|c| c.prefix(config.get_token()))
        .help(&MY_HELP)
        .group(&GENERAL_GROUP);
    let mut client = Client::builder(config.get_token())
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("creating client");
    if let Err(err) = client.start().await {
        error!("An error occurred while running the client: {:?}", err);
    }
}

pub struct Config {
    token: String,
    owner_id: UserId,
    prefix: String,
}

impl Config {
    pub fn get_token(&self) -> &String {
        &self.token
    }

    pub fn get_owner_id(&self) -> &UserId {
        &self.owner_id
    }

    pub fn get_prefix(&self) -> &String {
        &self.prefix
    }
}

pub fn get_config() -> Config  {
    Config {
        token: env::var("BOT_TOKEN").expect("Expected a BOT_TOKEN in the environment"),
        owner_id: env::var("BOT_OWNER_ID").expect("Expected a BOT_OWNER_ID id in the environment").parse::<UserId>().expect("The owner id is not a valid integer"),
        prefix: env::var("BOT_PREFIX").expect("Expected a BOT_PREFIX in the environment"),
    }
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[help]
#[individual_command_tip = "Help command"]
#[strikethrough_commands_tip_in_guild = ""]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    msg.channel_id.say(&context.http, "Help command").await?;
        let _ = help_commands::plain(context, msg, args, &help_options, groups, owners).await?;
    Ok(())
}


