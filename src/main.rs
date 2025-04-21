use colored::*;
use newsapi::{get_articles,Articles};
use::std::error::Error;
use dotenv::dotenv; 

fn render_articles(articles: &Articles){
    for i in &articles.articles{
        println!("{}", format!("> {}", i.title).green());  
        println!("{}", format!("- {}", i.url).yellow()); 
    }
}

fn main()-> Result<(),Box<dyn Error>> {
    dotenv()?;
    let api_key = std::env::var("API_KEY")?;
    let url = "https://newsapi.org/v2/top-headlines?country=us&apiKey=";
    let url = format!("{}{}",url,api_key);

    let articles = get_articles(&url)?;

    render_articles(&articles);
    Ok(())

}
