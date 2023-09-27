use logger_rs::info;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn main(context: &Context, message: &Message, _args: &Vec<&str>) {
    message
        .reply(context, "まだないよ！！ここでコマンドを確認してね！！\nhttps://github.com/HidemaruOwO/Iris/tree/develop/src/commands")
        .await
        .unwrap()
        .react(context, ReactionType::Unicode("✅".to_string()))
        .await
        .expect("Failed to react");
    info!("✅ Success");
}
