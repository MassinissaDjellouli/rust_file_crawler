use std::io;
use std::process::exit;
use crate::bin::file_crawler::FileCrawler;

pub struct CrawlerInterface{
    crawler:FileCrawler,
    ready:bool
}

impl CrawlerInterface{
    pub fn new() -> CrawlerInterface{
        CrawlerInterface{
            crawler:FileCrawler::new(),
            ready:false
        }
    }
    fn init(&mut self){
        self.crawler.start_indexing();
        self.ready = true;
    }
    pub fn run(&mut self){
        let quit_id =  vec![ &rand::random::<i128>().to_string(), &rand::random::<i128>().to_string(), &rand::random::<i128>().to_string()].join(&*rand::random::<i128>().to_string());
        self.init();
        loop{
            println!("Enter a command: (? for help)");
            let mut input = String::new();
            let result = match io::stdin().read_line(&mut input) {
                Ok(_) => self.handle_input(input.as_str(),&quit_id),
                Err(_) => {
                    println!("An error occured");
                    exit(0);
                }
            };
            if result.eq(&quit_id) {
                println!("Quitting...");
                exit(0);
            }
            println!("{}",result);
        }
    }

    fn handle_input<'a>(&mut self, input:&str,quit_id:&'a String) -> &'a str{
        println!("{}",input);
        match input.trim() {
            "?" => {
                r#"Different commands:
                    find [OPTION]... "TO_FIND"

                    TO_FIND: mandatory arg, must be between quotes

                    options:
                    -c
                        Content search. Searches the file content.
                    -f
                        Goes with -c. Full content search, including outside cached content. Will take more time.
                    -d
                        Directory only search.
                    --in [DIRECTORIES]
                        Searches only in the directories given. Must be wraped in single quotes (') and separated by commas (,)"#
            },
            "Q" | "q" => quit_id.as_str(),
            _ => "Input not recognized."
        }
    }
}