pub mod parser;

use std::fs::File;
use std::io;
use serde_json::to_writer;
use parser::news::News;
use parser::errors::Error;
use parser::post::Post;

#[tokio::main]
async fn main() -> io::Result<()> {
    const ENDPOINT: &str = "https://www.nasa.gov";
    const PAGES: usize = 2;

    // Create news and posts vectors
    let mut news_vec: Vec<News> = Vec::new();
    let mut posts: Vec<Post> = Vec::new();

    for page in 1..=PAGES {
        match News::new(ENDPOINT, page).await {
            Ok(news) => {
                news_vec.push(news.clone()); // Store news for writing later

                // For each post in the news, create a Post instance using its URL
                for post in news.posts {
                    if let Some(url) = post.url {
                        match Post::new(url.clone()).await {
                            Ok(post_detail) => {
                                posts.push(post_detail);
                            }
                            Err(e) => {
                                eprintln!("Error fetching post detail for URL {}: {:#?}", url, e);
                            }
                        }
                    } else {
                        eprintln!("Post has no URL, skipping.");
                    }
                }
            }
            Err(e) => {
                eprintln!("Error fetching news from page {}: {:#?}", page, e);
            }
        };
    }

    // Write all news to news.json
    let mut news_file: File = File::create("./result/news.json")?;
    if let Err(e) = to_writer(&mut news_file, &news_vec) {
        eprintln!("Error writing news to JSON: {}", e);
    }

    // Write all posts to posts.json
    let mut posts_file: File = File::create("./result/posts.json")?;
    if let Err(e) = to_writer(&mut posts_file, &posts) {
        eprintln!("Error writing posts to JSON: {}", e);
    }

    Ok(())
}