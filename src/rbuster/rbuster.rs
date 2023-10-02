use std::time::Duration;
use std::fs;
use reqwest::Client;

pub struct Rbuster {
    url: String,
    wordlist: String,
    threads: usize,
    endings: String,
    timeout: u64,
    not_found_string: String,
}

impl Rbuster {
    pub fn new(url: String, wordlist: String, threads: usize, endings: String, timeout: u64, not_found_string: String) -> Self {

        if !fs::metadata(wordlist.clone()).is_ok() {
            panic!("Wordlist not found!");
        }

        Self {
            url,
            wordlist,
            threads,
            endings,
            timeout,
            not_found_string,
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
                let not_found_string = self.not_found_string.clone();

                let task = tokio::spawn(async move {
                    let client = Client::new();
                    let res = client.get(url).timeout(timeout).send().await;
                    match res {
                        Ok(res) => {
                            let url = res.url().clone();
                            let status = res.status().as_u16();
                            if res.status().is_success() && !res.text().await.unwrap().contains(&not_found_string) {
                                println!("{} - {}", status, url);
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