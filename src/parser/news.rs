use std::borrow::Borrow;
use reqwest::Response;
use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};
use super::errors::{Error, Errors};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Post {
    pub(crate) title: Option<String>,
    pub(crate) short_description: Option<String>,
    pub(crate) url: Option<String>,
    pub(crate) image: Option<String>
}

impl Post {
    fn new(title: Option<String>, short_description: Option<String>, image: Option<String>, url: Option<String>) -> Self {
        Self {
            title,
            short_description,
            url,
            image
        }
    }

    fn get_title(element: &scraper::ElementRef<'_>) -> Result<Option<String>, Error> {
        let selector: Selector = Selector::parse(".hds-a11y-heading-22")
            .map_err(|_e| Error::new(Errors::FailedParseSelector, "failed parse selector title post"))?;

        let name: Option<String> = element
            .select(&selector)
            .next()
            .map(|element| element.text().collect());

        Ok(name)
    }
    fn get_description(element: &scraper::ElementRef<'_>) -> Result<Option<String>, Error> {
        let selector: Selector = Selector::parse(".margin-top-0.margin-bottom-1")
            .map_err(|_e| Error::new(Errors::FailedParseSelector, "failed parse selector description post"))?;

        let description: Option<String> = element
            .select(&selector)
            .next()
            .map(|element| element.text().collect());

        Ok(description)
    }
    fn get_url(element: &scraper::ElementRef<'_>) -> Result<Option<String>, Error> {
        let selector: Selector = Selector::parse("a")
            .map_err(|_e| Error::new(Errors::FailedParseSelector, "failed parse selector url post"))?;

        let url: Option<String> = element
            .select(&selector)
            .next()
            .and_then(|element| element.attr("href").map(|href| href.to_string()));

        Ok(url)
    }
    fn get_image(element: &scraper::ElementRef<'_>) -> Result<Option<String>, Error> {
        let selector: Selector = Selector::parse(".hds-media-background img")
            .map_err(|_e| Error::new(Errors::FailedParseSelector, "failed parse selector url post"))?;

        let image: Option<String> = element
            .select(&selector)
            .next()
            .and_then(|element| element.attr("src").map(|src| src.to_string()));

        Ok(image)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct News {
    pub posts: Vec<Post>,
    pub page: usize
}
impl News {
    pub async fn new<S: Into<String>>(endpoint: S, page: usize) -> Result<News, Error> {
        let endpoint: &str = &endpoint.into();

        let resp: Response = reqwest::get(format!("{}/wp-json/nasa-hds/v1/content-lists?postType=post&layout=list&showThumbnails=yes&showReadTime=yes&showExcerpts=yes&showContentTypeTags=yes&pageClicked={}", endpoint, page))
            .await
            .map_err(|e| Error::new(Errors::FailedResponseBody, e.to_string()))?;

        let body: String = resp
            .text()
            .await
            .map_err(|e| Error::new(Errors::FailedResponseBody, e.to_string()))?;

        let json_data: serde_json::Value = serde_json::from_str(&body)
            .map_err(|e| Error::new(Errors::FailedParseJson, e.to_string()))?;

        let document: Html = if let Some(html) = json_data["html"].as_str() {
            Html::parse_document(html)
        } else {
            return Err(Error::new(Errors::FailedParseJson, "html not found"))
        };

        let mut posts: Vec<Post> = Vec::new();
        let posts_selector: Selector = Selector::parse(".hds-content-item")
            .map_err(|e| Error::new(Errors::FailedParseSelector, e.to_string()))?;
        for post_element in document.select(&posts_selector) {

            posts.push(Post::new(Post::get_title(&post_element)?,
                                 Post::get_description(&post_element)?,
                                 Post::get_image(&post_element)?,
                                 Post::get_url(&post_element)?,
            ));
        };

        Ok(News { posts, page })

    }

    pub fn to_json(&self) -> Result<String, Error> {
        serde_json::to_string(self).map_err(|e| Error::new(Errors::FailedSerializeJson, e.to_string()))
    }
}