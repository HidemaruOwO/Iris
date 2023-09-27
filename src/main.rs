mod commands;
mod libs;

use log::LevelFilter;
use logger_rs::*;
use serenity::async_trait;
use serenity::model::{channel::Message, gateway::Ready};
use serenity::prelude::{Client, Context, EventHandler};

use libs::api;
use libs::config;
// ---------- Event Handler ----------
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("🌸 {} is connected!", ready.user.name);
    }
    async fn message(&self, context: Context, message: Message) {
        let cfg = &config::get_config();
        let prefix = cfg.get_prefix();
        let token = cfg.get_token();
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
                "ping" => commands::ping::main(&context, &message, &args).await,
                "src" => commands::src::main(&context, &message, &args).await,
                "help" => commands::help::main(&context, &message, &args).await,
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
    let cfg = &config::get_config();
    info!("🔎 Checking config...");
    // let owner_name =
    info!("Bot owner ID: {}", cfg.get_owner_id());
    info!("Bot prefix: {}\n", cfg.get_prefix());
    info!("🔑 Connecting to Discord...");
    let mut client = Client::builder(cfg.get_token())
        .event_handler(Handler)
        .await
        .expect("creating client");
    if let Err(err) = client.start().await {
        error!("An error occurred while running the client: {:?}", err);
    }
}
