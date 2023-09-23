use std::env::consts;
use std::process::Command;
use std::time::Instant;
use std::{collections::HashMap, time::Duration};

use chrono::{NaiveDateTime, Utc};
use logger_rs::{error, info};
use serenity::model::prelude::*;
use serenity::prelude::*;
use sysinfo::{CpuExt, System, SystemExt};

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
    let os = &consts::OS;
    let arch = &consts::ARCH;
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
    // Get CPU
    #[derive(Debug)]
    struct CpuInfo {
        brand: String,
        usage: f32,
        frequency: u64,
    }

    #[derive(Debug)]
    struct CpuList {
        brand: String,
        usage: f32,
        frequency: u64,
        cores: u32,
    }

    let mut sys = System::new_all();
    let mut brand_map: HashMap<String, Vec<CpuInfo>> = HashMap::new();

    let sampling_count = 2;

    for _ in 0..sampling_count {
        sys.refresh_cpu();
        for cpu in sys.cpus() {
            let brand = cpu.brand().to_string();
            brand_map
                .entry(brand.clone())
                .or_insert_with(Vec::new)
                .push(CpuInfo {
                    brand: brand.clone(),
                    usage: cpu.cpu_usage(),
                    frequency: cpu.frequency(),
                });
        }
        std::thread::sleep(Duration::from_millis(500));
    }
    let cpu_list: Vec<CpuList> = brand_map
        .iter()
        .map(|(brand, cpus)| {
            let (avg_usage, avg_frequency) = cpus.iter().fold((0.0, 0), |acc, cpu| {
                (acc.0 + cpu.usage, acc.1 + cpu.frequency)
            });
            let len = cpus.len() as f32;
            CpuList {
                brand: brand.clone(),
                usage: avg_usage / len,
                frequency: avg_frequency / len as u64,
                cores: len as u32 / sampling_count,
            }
        })
        .collect();

    info!("CPU List: {:?}", cpu_list);

    let cpu_info: String = cpu_list
        .iter()
        .enumerate()
        .map(|(index, cpu)| {
            format!(
                "⚙️ **CPU {}**```js\nBRAND: {}\nLOGIC CORE: {}\nUSAGE: {:.1}%\nFREQUENCY: {:.2}GHz\n```",
                index + 1,
                cpu.brand,
                cpu.cores,
                cpu.usage,
                cpu.frequency as f64 / 1000.0
            )
        })
        .collect::<Vec<String>>()
        .join("");

    let msg = message
        .channel_id
        .send_message(context, |m| {
            m.content(format!(
                "{}\n{}\n{}",
                format!(
                    "🚀 **API Latency**\n```js\nREAD: {}ms\nWRITE: {}ms\n```",
                    &ping, &write_ping
                ),
                format!(
                    "🖥 **System Info**\n```js\nOS: {} {}\nARCH: {}\nHOST: {}\n```",
                    os, &release, arch, &host_name
                ),
                cpu_info
            ))
        })
        .await
        .expect("Failed to send message");

    msg.react(context, ReactionType::Unicode("✅".to_string()))
        .await
        .expect("Failed to react");

    info!("✅ Successful");
}
