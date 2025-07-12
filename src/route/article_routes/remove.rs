use crate::articles::articles::ArticleIndex;
use crate::error::AppError;
use crate::http::get_successful_article_add;
use crate::request::authed::AuthedRequest;
use crate::route::article_routes::get_article_or_404;
use crate::utils::get_articles_index_path;
use async_std::io::WriteExt;
use async_std::net::TcpStream;

pub async fn delete(authed: AuthedRequest<'_>, stream: &mut TcpStream) -> Result<(), AppError> {
    let article =
        get_article_or_404(&authed.req.header.get_route_uuid(), &authed.state, stream).await?;
    article.remove().await?;
    let path = get_articles_index_path();
    let mut article_index = ArticleIndex::read_articles(&path).await?;
    article_index
        .delete_articles(&path, authed.req.header.get_route_uuid())
        .await?;
    let response = get_successful_article_add();
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}
