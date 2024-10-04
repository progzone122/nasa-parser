pub mod parser;
pub mod models;
pub mod schema;

use std::fs::File;
use std::{env, io};
use diesel::{Connection, MysqlConnection, QueryResult};
use dotenvy::dotenv;
use serde_json::to_writer;
use parser::news::News;
use parser::errors::Error;
use parser::post::Post;
use crate::models::news::NewNews;
use crate::models::posts;
use crate::models::posts::{NewPost, Posts};
use crate::parser::errors::Errors;
use crate::parser::news;

pub fn db_connect() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Unknown database url");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn push_db(post_content: parser::post::PostContent, news_post: parser::news::Post) -> Result<(), Error> {
    let mut conn: MysqlConnection = db_connect();

    // Начинаем транзакцию
    conn.transaction::<_, Error, _>(|transaction_conn| {
        let post_id: i32 = match models::posts::push(transaction_conn, &NewPost {
            html: post_content.html.as_ref().map(|a| a.as_str()).unwrap_or(""),
            text: post_content.text.as_ref().map(|a| a.as_str()).unwrap_or(""),
        }) {
            Ok(id) => id,
            Err(e) => {
                eprintln!("Database push post content error: {}", e);
                return Err(Error::new(Errors::FailedRequest, format!("Database push post content error: {}", e)));
            }
        };

        // Пушим новость
        match models::news::push(transaction_conn, &NewNews {
            title: news_post.title.as_ref().map(|a| a.as_str()).unwrap_or(""),
            short_description: news_post.short_description.as_deref(),
            image: news_post.image.as_deref(),
            url: news_post.url.as_ref().map(|a| a.as_str()).unwrap_or(""),
            post_id,
        }) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Database push news content error: {}", e);
                return Err(Error::new(Errors::FailedRequest, format!("Database push news content error: {}", e)));
            }
        }

        Ok(())
    })?;

    Ok(())
}

#[tokio::main]
async fn main() {
    const ENDPOINT: &str = "https://www.nasa.gov";
    const PAGES: usize = 100;

    // let mut news_vec: Vec<News> = Vec::new();

    for page in 1..=PAGES {
        match news::News::new(ENDPOINT, page).await {
            Ok(news) => {
                // println!("News: {:?}", news);
                for news_item in news.posts {
                    if let Some(url) = news_item.url.clone() {
                        match Post::new(url.clone()).await {
                            Ok(post_detail) => {
                                match push_db(post_detail.content, news_item) {
                                    Ok(_) => {}
                                    Err(_) => {}
                                }
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
                eprintln!("Error fetching news: {}", e.info());
            }
        };
    }

    //
    // // Create news and posts vectors
    // let mut news_vec: Vec<news> = Vec::new();
    // let mut posts: Vec<Post> = Vec::new();
    //
    // for page in 1..=PAGES {
    //     match news::new(ENDPOINT, page).await {
    //         Ok(news) => {
    //             news_vec.push(news.clone()); // Store news for writing later
    //
    //             // For each post in the news, create a Post instance using its URL
    //             for post in news.posts {
    //                 if let Some(url) = post.url {
    //                     match Post::new(url.clone()).await {
    //                         Ok(post_detail) => {
    //                             posts.push(post_detail);
    //                         }
    //                         Err(e) => {
    //                             eprintln!("Error fetching post detail for URL {}: {:#?}", url, e);
    //                         }
    //                     }
    //                 } else {
    //                     eprintln!("Post has no URL, skipping.");
    //                 }
    //             }
    //         }
    //         Err(e) => {
    //             eprintln!("Error fetching news from page {}: {:#?}", page, e);
    //         }
    //     };
    // }
    //
    // // Write all news to news.json
    // let mut news_file: File = File::create("./result/news.json")?;
    // if let Err(e) = to_writer(&mut news_file, &news_vec) {
    //     eprintln!("Error writing news to JSON: {}", e);
    // }
    //
    // // Write all posts to posts.json
    // let mut posts_file: File = File::create("./result/posts.json")?;
    // if let Err(e) = to_writer(&mut posts_file, &posts) {
    //     eprintln!("Error writing posts to JSON: {}", e);
    // }

    // Ok(())


}