use crate::articles::article::Article;
use crate::articles::articles::{ArticleIndex, ArticleMeta};
use crate::error::AppError;
use crate::http::get_response;
use crate::http::get_successful_article_add;
use crate::request::authed::AuthedRequest;
use async_std::io::WriteExt;
use async_std::net::TcpStream;
use async_std::path::Path;

pub async fn get_article_html(
    authed: AuthedRequest<'_>,
    stream: &mut TcpStream,
) -> Result<(), AppError> {
    let context = tera::Context::new();
    let render = authed
        .state
        .tempelates
        .render("add_article.html", &context)?;
    let response = get_response(&render);
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}

pub async fn add_article(
    authed: AuthedRequest<'_>,
    stream: &mut TcpStream,
) -> Result<(), AppError> {
    let path = Path::new("Articles").join("index.json");
    let mut article_index = ArticleIndex::read_articles(&path).await?;
    let article = Article::new(authed.req.get_body()?, authed.session.user_id.clone())?;
    article.save()?;
    let article_meta = ArticleMeta::new(article.title, article.uuid, article.date);
    article_index.add_articles(article_meta, &path).await?;
    let response = get_successful_article_add();
    stream.write(response.as_bytes()).await?;
    Ok(())
}
