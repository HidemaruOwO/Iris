use std::time::Instant;

use chrono::{NaiveDateTime, Utc};
use logger_rs::info;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn main(context: &Context, message: &Message, _args: &Vec<&str>) {
    let timestamp = Utc::now();
    let message_timestamp = message.timestamp.to_string();

    let local_time = timestamp.naive_utc();
    let message_time =
        NaiveDateTime::parse_from_str(&message_timestamp, "%Y-%m-%dT%H:%M:%S%.fZ").unwrap();

    let ping = local_time
        .signed_duration_since(message_time)
        .num_milliseconds();

    let send_start = Instant::now();
    let benchmark_msg = message.reply(context, "Benchmarking...").await.unwrap();
    let write_ping = send_start.elapsed().as_millis();
    benchmark_msg.delete(context).await.unwrap();

    let msg = message
        .channel_id
        .send_message(context, |m| {
            m.content(format!(
                "🚀 **API Latency**\n```js\nREAD: {}ms\nWRITE: {}ms\n```",
                ping, write_ping
            ))
        })
        .await
        .expect("Failed to send message");

    msg.react(context, ReactionType::Unicode("✅".to_string()))
        .await
        .expect("Failed to react");

    info!("✅ Successful");
}
