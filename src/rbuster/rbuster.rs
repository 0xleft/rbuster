use std::ops::Deref;
use std::time::Duration;
use std::{fs, sync::atomic::AtomicUsize};
use reqwest::Client;
use tokio;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

pub struct Rbuster {
    url: String,
    verbose: bool,
    wordlist: String,
    threads: usize,
    recursive: bool,
    depth: usize,
    endings: Vec<String>,
    timeout: u64,
}

impl Rbuster {
    pub fn new(url: String, verbose: bool, wordlist: String, threads: usize, recursive: bool, depth: usize, endings: Vec<String>, timeout: u64) -> Self {

        // check if wordlist exists
        if !fs::metadata(wordlist.clone()).is_ok() {
            panic!("Wordlist not found!");
        }

        Self {
            url,
            verbose,
            wordlist,
            threads,
            recursive,
            depth,
            endings,
            timeout,
        }
    }

    pub async fn run(&self) {
        // load wordlist
        let file_content = fs::read_to_string(self.wordlist.clone()).unwrap();
        // readable code 101
        // basicaly Vec<&str> -> Vec<String>
        let mut wordlist = file_content.split_whitespace().collect::<Vec<_>>().iter().map(|word| word.to_string()).collect::<Vec<_>>();

        let client = Client::builder()
            .timeout(Duration::from_secs(self.timeout))
            .build()
            .unwrap();

        for word in wordlist {
            let res = client.get(format!("{}/{}.php", self.url, word)).send().await;
            match res {
                Ok(res) => {
                    let status = res.status();
                    if status.is_success() {
                        let body = res.text().await.unwrap();
                        if !body.contains("404") && !body.to_lowercase().contains("not found") {
                            println!("{} -> {}", status, word);
                        }
                    }
                },
                Err(_) => {}
            }
        }
    }
}