use std::process::Command;

use logger_rs::info;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::libs::config;

pub async fn main(context: &Context, message: &Message, _args: &Vec<&str>) {
    let github_url = "https://github.com/HidemaruOwO/Iris";

    let mut branch = String::from("develop");

    let out = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .expect("Failed to execute command: uname");

    if out.status.code().unwrap() == 0 {
        branch = String::from_utf8(out.stdout).unwrap().trim().to_string();
    }

    if _args[0].is_empty() {
        message
            .reply(
                context,
                format!(
                    "⚠️ Missing argument.\nUsage: `{}src <CommandName>`",
                    &config::get_config().get_prefix()
                ),
            )
            .await
            .unwrap();
        return;
    }

    message
        .reply(
            context,
            format!(
                "💼 **Source Code: commands/{command_name}.rs**\n[commands/{command_name}.rs]({}/blob/{}/src/commands/{command_name}.rs)",
                github_url, branch, command_name=_args[0]
            ),
        )
        .await
        .unwrap();
    info!("✅ Success");
}
