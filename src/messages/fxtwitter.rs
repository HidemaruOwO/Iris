use logger_rs::{error, info};
use regex::Regex;
use serenity::model::prelude::*;
use serenity::prelude::*;
use url::Url;

pub async fn main(context: &Context, message: &Message) {
    let content = &message.content;
    let reg = Regex::new(r"https?://\S+").unwrap();
    let mut is_changed = false;

    let text = "**🔎 Tweet Preview**";
    let mut urls = String::from(text);

    for reg in reg.find_iter(content) {
        if !is_changed {
            is_changed = true;
        }
        if urls == text {
            urls.push_str("\n")
        }
        match Url::parse(reg.as_str()) {
            Ok(url) => {
                let domain = url.domain().unwrap();
                if domain == "twitter.com" || domain == "x.com" {
                    if let Some(mut pathes) = url.path_segments() {
                        if pathes.clone().count() == 3 as usize {
                            urls.push_str(&format!(
                                "・[Tweet by @{}](https://fxtwitter.com{})\n",
                                pathes.next().unwrap(),
                                url.path()
                            ));
                        }
                    }
                }
            }
            Err(err) => {
                error!("Invalid URL ({}): {}", reg.as_str(), err);
            }
        }
    }

    if is_changed {
        crate::libs::message_util::typing(context, message).await;
        message
            .reply(context, urls.trim_end_matches("\n"))
            .await
            .unwrap();
        info!("Success");
    }
}
