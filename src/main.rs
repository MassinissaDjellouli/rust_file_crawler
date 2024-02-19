use crate::bin::crawler_interface::CrawlerInterface;

mod bin;
fn main() {
    let mut crawler = CrawlerInterface::new();
    crawler.run()
}
