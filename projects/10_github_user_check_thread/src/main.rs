use reqwest::blocking::Client as BlockingHttpClient;
use reqwest::Error;
// use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread::{spawn, JoinHandle};

enum GitHubUserSearch {
    Found(String),
    NotFound(String),
}

fn fetch_user(
    http_client: &BlockingHttpClient,
    github_username: &str,
) -> Result<GitHubUserSearch, Error> {
    let res = http_client
        .get(format!("https://github.com/{}", github_username))
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
        .send()
        .expect("Failed to send request.");

    if res.status().is_success() {
        Ok(GitHubUserSearch::Found(github_username.to_owned()))
    } else {
        Ok(GitHubUserSearch::NotFound(github_username.to_owned()))
    }
}

fn main() {
    let github_usernames = vec![
        "ericwgreene".to_owned(),
        "devops-person".to_owned(),
        "egreene-at-syntrillo".to_owned(),
        "ericwgreene2".to_owned(),
    ];

    // Arc type is used to share data between threads
    // Mutex type is used to lock the data
    // let github_user_found_count = Rc::new(Mutex::new(0));
    let github_user_found_count = Arc::new(Mutex::new(0));

    let client = BlockingHttpClient::new();

    let github_user_search_threads = github_usernames
        .into_iter()
        .map(|github_username| {
            let client = client.clone();
            // - pass a reference of the Arc to the Arc::clone method to clone it, then it will moved into the closure
            // - when spawn executes the closure on a new thread, it will be able to access the Mutex through
            //   the cloned Arc
            // - when the thread desires to modify the data protected by the Mutex, it will lock the Mutex
            //   and then modify the data
            // let github_user_found_count = Rc::clone(&github_user_found_count);
            let github_user_found_count = Arc::clone(&github_user_found_count);
            println!("fetching {}", &github_username);
            spawn(move || {
                let fetch_user_result = fetch_user(&client, &github_username);
                println!("fetched {}", &github_username);
                match fetch_user_result {
                    Ok(GitHubUserSearch::Found(_)) => {
                        // create mutable reference to the data protected by the Mutex
                        // lock the Mutex to get the mutable reference, and unwrap the result
                        let mut count = github_user_found_count.lock().unwrap();
                        // the * operator is used to dereference the MutexGuard to access the data
                        *count += 1;
                    }
                    _ => {}
                }
                fetch_user_result
            })
        })
        .collect::<Vec<JoinHandle<Result<GitHubUserSearch, Error>>>>();

    let github_user_search_results = github_user_search_threads
        .into_iter()
        .map(|thread| thread.join().expect("Unable to join thread."))
        .collect::<Vec<Result<GitHubUserSearch, Error>>>();

    let github_user_final_count = *github_user_found_count.lock().unwrap(); // Lock the Mutex to read the data
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
