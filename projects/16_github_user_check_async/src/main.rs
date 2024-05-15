use reqwest::Client as HttpClient;
use reqwest::Error;
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;

enum GitHubUserSearch {
    Found(String),
    NotFound(String),
}

async fn fetch_user(
    http_client: &HttpClient,
    github_username: &str,
) -> Result<GitHubUserSearch, Error> {
    let res = http_client
        .get(format!("https://github.com/{}", github_username))
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
        .send()
        .await?;

    if res.status().is_success() {
        Ok(GitHubUserSearch::Found(github_username.to_owned()))
    } else {
        Ok(GitHubUserSearch::NotFound(github_username.to_owned()))
    }
}

#[tokio::main]
async fn main() {
    let github_usernames = vec![
        "ericwgreene".to_owned(),
        "devops-person".to_owned(),
        "egreene-at-syntrillo".to_owned(),
        "ericwgreene2".to_owned(),
    ];

    let github_user_found_count = Arc::new(Mutex::new(0));
    let client = HttpClient::new();

    let mut github_user_search_tasks: Vec<JoinHandle<Result<GitHubUserSearch, Error>>> = vec![];

    for github_username in github_usernames {
        let client = client.clone();
        let github_user_found_count = Arc::clone(&github_user_found_count);
        println!("fetching {}", &github_username);
        let task: JoinHandle<Result<GitHubUserSearch, Error>> = tokio::spawn(async move {
            let fetch_user_result = fetch_user(&client, &github_username).await;
            println!("fetched {}", &github_username);
            if let Ok(GitHubUserSearch::Found(_)) = fetch_user_result {
                let mut count = github_user_found_count.lock().unwrap();
                *count += 1;
            }
            fetch_user_result
        });
        github_user_search_tasks.push(task);
    }

    let mut github_user_search_results = Vec::new();
    for task in github_user_search_tasks {
        github_user_search_results.push(task.await.expect("Task failed."));
    }

    let github_user_final_count = *github_user_found_count.lock().unwrap();
    println!("Number of GitHub users found: {}", github_user_final_count);

    for result in github_user_search_results {
        match result {
            Ok(GitHubUserSearch::Found(username)) => {
                println!("Found GitHub user: {}", username);
            }
            Ok(GitHubUserSearch::NotFound(username)) => {
                println!("GitHub user not found: {}", username);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}
