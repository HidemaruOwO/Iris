use logger_rs::info;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn main(context: &Context, message: &Message, _args: &Vec<&str>) {
    crate::libs::message_util::typing(context, message).await;
    message
        .reply(
            context,
            "https://obj.04.si/files/b3029ce7-35ef-4e0d-a6ea-7f05ffb3cb71.gif",
        )
        .await
        .unwrap()
        .react(context, ReactionType::Unicode("✅".to_string()))
        .await
        .expect("Failed to react");
    1;
    info!("✅ Success");
}
