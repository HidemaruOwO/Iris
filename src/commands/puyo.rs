use logger_rs::info;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn main(context: &Context, message: &Message, _args: &Vec<&str>) {
    message
        .reply(
            context,
            "https://obj.04.si/files/b3029ce7-35ef-4e0d-a6ea-7f05ffb3cb71.gif",
        )
        .await
        .unwrap();
    info!("✅ Success");
}
