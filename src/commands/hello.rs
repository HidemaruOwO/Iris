use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description("Hello world command")]
async fn hello(context: &Context, message: &Message) -> CommandResult {
    message.reply(context, "Hello world!!").await?;
    Ok(())
}
