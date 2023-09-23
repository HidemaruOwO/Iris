use logger_rs::info;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn main(context: &Context, message: &Message, _args: &Vec<&str>) {
    message.reply(context, "Hello world!!").await.unwrap();
    info!("✅ Success");
}
