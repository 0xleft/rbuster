pub struct Rbuster {
    url: String,
    verbose: bool,
    wordlist: String,
    threads: usize,
    recursive: bool,
    depth: usize,
}

impl Rbuster {
    pub fn new(url: String, verbose: bool, wordlist: String, threads: usize, recursive: bool, depth: usize) -> Self {
        Self {
            url,
            verbose,
            wordlist,
            threads,
            recursive,
            depth,
        }
    }

    pub fn run(&self) {
        println!("Running Rbuster");
    }
}