use std::env;
use std::path::Path;
use crate::bin::crawler_interface::CrawlerInterface;

mod bin;
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut start_from:Option<&Path> = None;
    let mut flags = 0;
    if args.len() > 1{
        let path = Path::new(&args[1]);
        if path.exists() && path.is_dir(){
            start_from = Some(path)
        }
        if args.contains(&"-c".to_string()){
            flags += 1
        }
    }
    let mut crawler = CrawlerInterface::new(start_from,flags);
    crawler.run()
}
