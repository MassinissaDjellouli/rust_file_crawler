use crate::bin::crawler_interface::CrawlerInterface;
use crate::bin::file_crawler::FileCrawler;

mod bin;
fn main() {
    let mut crawler = CrawlerInterface::new();
    crawler.run()
}
