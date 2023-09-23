mod commands;
mod libs;

use std::env;

use log::LevelFilter;
use logger_rs::*;
use serenity::async_trait;
use serenity::model::{channel::Message, gateway::Ready, id::UserId};
use serenity::prelude::{Client, Context, EventHandler};

use libs::api;
// ---------- Event Handler ----------
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
    async fn message(&self, context: Context, message: Message) {
        let config = &get_config();
        let prefix = config.get_prefix();
        let token = config.get_token();
        if !message.content.starts_with(prefix) {
            return;
        }
        if let Some(content) = message.content.strip_prefix(prefix) {
            let mut parts = content.split_whitespace();
            let cmd = parts.next().unwrap_or("");
            let args = parts.collect::<Vec<_>>();

            if cmd.is_empty() {
                return;
            }

            let guild = api::guild(token.to_string(), message.guild_id.unwrap().to_string())
                .await
                .unwrap();

            info!("💨 Running command: {}", cmd);
            info!("args: {:?}", args);
            info!("server: ");
            info!(" name: {}", guild.get_name());
            info!(" id: {}", message.guild_id.unwrap().to_string());

            match cmd {
                "puyo" => commands::puyo::main(&context, &message, &args).await,
                "hello" => commands::hello::main(&context, &message, &args).await,
                _ => info!("Command not found: {}", cmd),
            }
        }
    }
}

// -----------------------------------
#[tokio::main]
async fn main() {
    let filters: [(Option<&str>, LevelFilter); 2] =
        [(None, LevelFilter::Warn), (Some("iris"), LevelFilter::Info)];
    init_logger(&filters);
    info!("🚀 Starting...\n");
    let config = get_config();
    info!("🔎 Checking config...");
    // let owner_name =
    info!("Bot owner ID: {}", config.get_owner_id());
    info!("Bot prefix: {}\n", config.get_prefix());
    info!("🔑 Connecting to Discord...");
    let mut client = Client::builder(config.get_token())
        .event_handler(Handler)
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

pub fn get_config() -> Config {
    Config {
        token: env::var("BOT_TOKEN").expect("Expected a BOT_TOKEN in the environment"),
        owner_id: env::var("BOT_OWNER_ID")
            .expect("Expected a BOT_OWNER_ID id in the environment")
            .parse::<UserId>()
            .expect("The owner id is not a valid integer"),
        prefix: env::var("BOT_PREFIX").expect("Expected a BOT_PREFIX in the environment"),
    }
}
