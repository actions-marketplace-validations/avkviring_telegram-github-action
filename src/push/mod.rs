use std::fs;

use crate::github::PushEvent;

pub fn process_push_event(event: String) -> String {
    let result: PushEvent = serde_json::from_str(event.as_str()).unwrap();

    let message = format_commit_message(
        result.pusher.name,
        result.repository.name,
        result
            .commits
            .iter()
            .map(|r| Commit {
                href: r.url.clone(),
                comment: r.message.clone(),
            })
            .collect(),
    );
    return message;
}

struct Commit {
    href: String,
    comment: String,
}

fn format_commit_message(author: String, repo: String, commits: Vec<Commit>) -> String {
    let concat = commits
        .iter()
        .map(|c| format!("[➞]({}) {}\n", c.href, c.comment))
        .fold("".to_string(), |mut acc, g| {
            acc.push_str(&g.to_string());
            acc
        });
    return format!("*{} push to {}*\n {}", author, repo, concat);
}