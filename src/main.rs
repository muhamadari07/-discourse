extern crate maman;

use std::env;
use std::process;

use maman::Spider;

#[cfg(not(test))]
fn main() {
    let url = match env::args().nth(1) {
        Some(url) => url,
        None => {
            println!("Usage: maman URL");
            process::exit(1);
        }
    };

    let mut spider = Spider::new(url);
    spider.crawl()
}
