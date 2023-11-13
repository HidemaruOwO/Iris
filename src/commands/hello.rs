use logger_rs::info;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn main(context: &Context, message: &Message, _args: &Vec<&str>) {
    crate::libs::message_util::typing(context, message).await;
    message
        .reply(context, "Hello world!!")
        .await
        .unwrap();
    info!("✅ Success");
}
