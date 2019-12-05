use reqwest;    // reqwest makes http request 
use scraper::{Html, Selector};      // html parsing and parsing selectors
use select;     //

fn main() -> Result<(), Box<std::error::Error>> {

      let mut website=reqwest::get("https://tribune.com.pk/cricket/")?;// make a get request
      assert!(website.status().is_success());

      let news=website.text().unwrap();
      let parse=Html::parse_document(&news);
      let sel=Selector::parse("#trending_stories > div.span-16.primary > div > h2 > a
").unwrap();
      for latest in parse.select(&sel){
        let full_news=latest.text().collect::<Vec<_>>();
        if full_news.len() == 1 {
            println!("{:?}",full_news[0]);
        }
    }
        Ok(())
}
// body > div.container.sports.category > div:nth-child(11) 
// > div.primary.span-16 > div.main.group.clearfix > div.span-6.top-news> div > div > h2 > a
