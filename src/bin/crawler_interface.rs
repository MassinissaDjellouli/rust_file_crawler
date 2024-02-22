use std::collections::VecDeque;
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
        let _ = &self.crawler.start_indexing();
        self.ready = true;
    }
    pub fn run(&mut self){
        let quit_id =  vec![ rand::random::<i128>().to_string(), rand::random::<i128>().to_string(), rand::random::<i128>().to_string()].join(&*rand::random::<i128>().to_string());
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

    fn handle_input<'a>(&mut self, input:&'a str,quit_id:&'a String) -> String{
        match input.trim() {
            "?" => {
                String::from(r#"Commands:
                    find <TO_FIND>
                    find_content <CONTENT>
                        Content search. Searches the file content in cached files.
                    find_content_full <CONTENT>
                        Full content search, including outside cached content. Will take more time.
                    find_directory <TO_FIND>
                        Directory only search.
                    find_in [DIRECTORIES] <TO_FIND>
                        Searches only in the directories given. Must be wraped in single quotes (') and separated by commas (,)
                            example -> 'dir1,"dir 2",dir3'"#)
            },
            "Q" | "q" => quit_id.to_string(),
            _ => self.default_handle(input.trim())
        }
    }
    fn default_handle(&mut self, input: &str) -> String{
        if input.starts_with("find_content_full"){
            return self.handle_find_content(input,true)
        }
        if input.starts_with("find_content"){
            return self.handle_find_content(input,false)
        }
        if input.starts_with("find_directory"){
            return self.handle_find_dir(input)
        }
        if input.starts_with("find_in"){
            return self.handle_find_in(input)
        }
        if input.starts_with("find"){
            return self.handle_find(input)
        }
        String::from("Input not recognized.")
    }

    fn handle_find(&mut self, input: &str) -> String{
        let mut split = input.split(" ").collect::<VecDeque<&str>>();
        if match split.get(0) {
            Some(s) => s,
            None => return String::from("Missing function.")
        } != &"find" {
           return String::from("Invalid command.")
        }

        if split.len() < 2 {
            return String::from("Missing arguments")
        }
        split.pop_front();
        let split = split.into_iter().collect::<Vec<&str>>().join(" ");
        let found = self.crawler.find(split.clone(),None);
        format!("Found {} files with \"{}\" : \n\n{}",found.len(),split,found.join("\n"))
    }
    fn handle_find_dir(&mut self, input: &str) -> String{
        let mut split = input.split(" ").collect::<VecDeque<&str>>();
        if match split.get(0) {
            Some(s) => s,
            None => return String::from("Missing function.")
        } != &"find_directory" {
            return String::from("Invalid command.")
        }

        if split.len() < 2 {
            return String::from("Missing arguments")
        }
        split.pop_front();
        let split = split.into_iter().collect::<Vec<&str>>().join(" ");
        let found = self.crawler.find_dir(split.clone(),None);
        format!("Found {} directory with \"{}\" : \n\n{}",found.len(),split,found.join("\n"))
    }
    fn handle_find_in(&mut self, input: &str) -> String{
        String::from("not impl")
    }
    fn handle_find_content(&mut self, input: &str, full:bool) -> String{
        String::from("not impl")
    }

}