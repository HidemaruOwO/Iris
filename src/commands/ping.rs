use std::env::consts;
use std::process::Command;
use std::time::Instant;

use chrono::{NaiveDateTime, Utc};
use logger_rs::{error, info};
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

    // Get OS
    let os = consts::OS;
    let arch = consts::ARCH;
    let host_name = match hostname::get() {
        Ok(host) => host.to_string_lossy().to_string(),
        Err(_) => "unknown host".to_string(),
    };

    let mut release = "unknown release".to_string();

    #[cfg(target_os = "linux")]
    {
        let out = Command::new("uname")
            .arg("-r")
            .output()
            .expect("Failed to execute command: uname");

        if let Ok(stdout) = String::from_utf8(out.stdout) {
            release = stdout.trim().to_string();
        } else {
            error!("Failed to get os release");
        }
    }

    #[cfg(target_os = "macos")]
    {
        let out = Command::new("sw_vers")
            .arg("-productVersion")
            .output()
            .expect("Failed to execute command: sw_vers");

        if let Ok(stdout) = String::from_utf8(out.stdout) {
            release = stdout.trim().to_string();
        } else {
            error!("Failed to get os release");
        }
    }

    #[cfg(target_os = "windows")]
    {
        let out = Command::new("wmic")
            .arg("OS")
            .arg("get")
            .arg("Version")
            .output()
            .expect("Failed to execute command: wmic");

        if let Ok(stdout) = String::from_utf8(out.stdout) {
            release = stdout.trim().to_string();
        } else {
            error!("Failed to get os release");
        }
    }
    let msg = message
        .channel_id
        .send_message(context, |m| {
            m.content(format!(
                "🚀 **API Latency**\n```js\nREAD: {}ms\nWRITE: {}ms\n```\n🖥 **System Info**\n```js\nOS: {} {}\nARCH: {}\nHOST: {}\n```",
                ping, write_ping, os, release, arch, host_name
            ))
        })
        .await
        .expect("Failed to send message");

    msg.react(context, ReactionType::Unicode("✅".to_string()))
        .await
        .expect("Failed to react");

    info!("✅ Successful");
}
