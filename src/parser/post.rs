use reqwest::Response;
use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};
use super::errors::{Error, Errors};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PostContent {
    pub(crate) text: Option<String>,
    pub(crate) html: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Post {
    title: Option<String>,
    pub(crate) content: PostContent
}

impl Post {
    pub async fn new<S: Into<String>>(url: S) -> Result<Post, Error> {
        let url: &str = &url.into();

        let resp: Response = reqwest::get(url)
            .await
            .map_err(|e| Error::new(Errors::FailedResponseBody, e.to_string()))?;

        let body: String = resp
            .text()
            .await
            .map_err(|e| Error::new(Errors::FailedResponseBody, e.to_string()))?;

        let document: Html = Html::parse_document(&body);

        Ok(Post {
            title: Self::get_title(&document)?,
            content: Self::get_content(&document)?,
        })
    }

    pub fn to_json(&self) -> Result<String, Error> {
        serde_json::to_string(self).map_err(|e| Error::new(Errors::FailedSerializeJson, e.to_string()))
    }

    fn get_title(document: &Html) -> Result<Option<String>, Error> {
        let selector: Selector = Selector::parse(r#"meta[property="og:title"]"#)
            .map_err(|e| Error::new(Errors::FailedParseSelector, e.to_string()))?;

        let title: Option<String> = document
            .select(&selector)
            .next()
            .and_then(|element| element.attr("content").map(|content| content.to_string()));

        Ok(title)
    }
    fn get_content(document: &Html) -> Result<PostContent, Error> {
        let selector_html: Selector = Selector::parse(".entry-content")
            .map_err(|e| Error::new(Errors::FailedParseSelector, e.to_string()))?;

        let html: Option<String> = document
            .select(&selector_html)
            .map(|element| element.html())
            .collect::<Vec<String>>()
            .join("")
            .into();

        let text: Option<String> = Some(
            document
                .select(&selector_html)
                .flat_map(|element| element.text()) // Извлекаем текстовые узлы
                .collect::<Vec<&str>>() // Собираем их в вектор строк
                .join("") // Объединяем в одну строку
        );

        Ok(PostContent { text, html })
    }
}

