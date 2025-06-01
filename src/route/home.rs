use crate::articles;
use crate::articles::articles::ArticleMeta;
use crate::error::HttpError;
use crate::html::{get_html, get_html_template};
use crate::http::get_response;
use async_std::io::WriteExt;
use async_std::net::TcpStream;

pub async fn home(stream: &mut TcpStream) -> Result<(), HttpError> {
    let html_template = get_html_template("index.html").await?;
    let articles_meta = articles::articles::ArticleIndex::read_articles(
        async_std::path::Path::new("Articles/index.json"),
    )
    .await?;
    let articles = get_articles_html(&articles_meta.articles);
    let html = get_html(html_template, "{{articles}}", &articles);
    let response = get_response(&html);
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}

fn get_articles_html(article_list: &[ArticleMeta]) -> String {
    article_list
        .iter()
        .map(|article| {
            format!(
                "<li><a href=\"{}\">{} {}</a></li>\n",
                article.uuid, article.title, article.date
            )
        })
        .collect::<Vec<_>>()
        .join("")
}
