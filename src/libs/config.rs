use serenity::model::prelude::UserId;
use std::env;

pub struct Config {
    token: String,
    prefix: String,
    id: UserId,
    owner_id: UserId,
}

impl Config {
    pub fn get_token(&self) -> &String {
        &self.token
    }
    pub fn get_prefix(&self) -> &String {
        &self.prefix
    }
    pub fn get_id(&self) -> &UserId {
        &self.id
    }
    pub fn get_owner_id(&self) -> &UserId {
        &self.owner_id
    }
}

pub fn get_config() -> Config {
    Config {
        token: env::var("BOT_TOKEN").expect("Expected a BOT_TOKEN in the environment"),
        prefix: env::var("BOT_PREFIX").expect("Expected a BOT_PREFIX in the environment"),
        id: env::var("BOT_ID")
            .expect("Expected a BOT_ID id in the environment")
            .parse::<UserId>()
            .expect("The id is not a valid integer"),
        owner_id: env::var("BOT_OWNER_ID")
            .expect("Expected a BOT_OWNER_ID id in the environment")
            .parse::<UserId>()
            .expect("The owner id is not a valid integer"),
    }
}
