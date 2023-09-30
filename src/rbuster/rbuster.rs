use std::time::Duration;
use std::fs;
use reqwest::Client;

pub struct Rbuster {
    url: String,
    verbose: bool,
    wordlist: String,
    threads: usize,
    recursive: bool,
    depth: usize,
    endings: String,
    timeout: u64,
}

impl Rbuster {
    pub fn new(url: String, verbose: bool, wordlist: String, threads: usize, recursive: bool, depth: usize, endings: String, timeout: u64) -> Self {

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
        let wordlist = file_content.split_whitespace().collect::<Vec<_>>().iter().map(|word| word.to_string()).collect::<Vec<_>>();

        let mut threads: Vec<tokio::task::JoinHandle<()>> = Vec::new();
        let endings = self.endings.split(",").collect::<Vec<_>>();
        // make sure they all are unique
        let endings = endings.iter().map(|ending| ending.to_string()).collect::<std::collections::HashSet<_>>();

        for word in wordlist {
            for ending in endings.clone()  {
                
                // max threads
                if threads.len() >= self.threads {
                    let _ = threads.pop().unwrap().await;
                }

                let url = format!("{}/{}{}", self.url, word, ending);
                let timeout = Duration::from_secs(self.timeout).clone();
                let task = tokio::spawn(async move {
                    let client = Client::new();
                    let res = client.get(url).timeout(timeout).send().await;
                    match res {
                        Ok(res) => {
                            if res.status().is_success() {
                                println!("{} - {}", res.status(), res.url());
                            }
                        },
                        Err(err) => {
                            println!("{}", err);
                        }
                    }
                });
                threads.push(task);
            }
        }
    }
}