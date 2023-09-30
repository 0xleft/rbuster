use std::{fs, sync::atomic::AtomicUsize};
use reqwest::Client;
use tokio;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

pub struct Rbuster {
    url: String,
    verbose: bool,
    wordlist: String,
    // max request per second
    threads: usize,
    recursive: bool,
    depth: usize,
    endings: Vec<String>,
}

impl Rbuster {
    pub fn new(url: String, verbose: bool, wordlist: String, threads: usize, recursive: bool, depth: usize, endings: Vec<String>) -> Self {

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
        }
    }

    pub async fn run(&self) {
        // starting the bruteforce
        let word_deque: Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(VecDeque::new()));

        // load wordlist
        let wordlist = fs::read_to_string(self.wordlist.clone()).unwrap();
        let mut wordlist: Vec<&str> = wordlist.split("\n").collect();
        wordlist.reverse();

        // load wordlist into shared deque
        for word in wordlist.clone() {
            word_deque.lock().unwrap().push_back(word.to_string());
        }

        println!("{} words loaded", word_deque.lock().unwrap().len());
        
        // create a client
        let client = Client::new();

        // create a counter
        let counter = Arc::new(AtomicUsize::new(0));

        // start
        for _ in 0..self.threads {
            let client = client.clone();
            let counter = counter.clone();
            let word_deque = word_deque.clone();
            let url = self.url.clone();
            let verbose = self.verbose.clone();
            let recursive = self.recursive.clone();
            let depth = self.depth.clone();
            let endings = self.endings.clone();

            tokio::spawn(async move {
                loop {
                    let word = word_deque.lock().unwrap().pop_front();
                    if word.is_none() {
                        break;
                    }

                    let word = word.unwrap();

                    let mut url = url.clone();
                    url.push_str(&word);

                    let res = client.get(&url).send().await;
                    if res.is_ok() {
                        let res = res.unwrap();

                        if res.status().is_success() {
                            println!("{} - {}", res.status(), url);
                        }
                    }
                }
            });
        }
    }
}