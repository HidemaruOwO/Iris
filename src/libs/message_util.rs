use logger_rs::debug;
use rand::{rngs::OsRng, Rng};
use tokio::time::Duration;

use serenity::{http::Typing, model::prelude::Message, prelude::Context};

pub async fn typing(context: &Context, message: &Message) {
    let channel_id = message.channel_id.0;
    let typing = Typing::start(context.http.clone(), channel_id).expect("Failed to start typing");
    let mut rng = OsRng::default();
    let wait_time: u64 = rng.gen_range(500..2500);
    debug!("Typing for {}ms", wait_time);

    let sleep_duration = Duration::from_millis(wait_time);
    tokio::time::sleep(sleep_duration).await;

    typing.stop();
}
