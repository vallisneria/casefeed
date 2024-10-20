mod db;
mod rss;
mod util;

use rss::RssItem;
use worker::{
    console_error, console_log, event, Context, Env, Headers, Request, Response,
    Result as WorkerResult, RouteContext, Router,
};

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> WorkerResult<Response> {
    Router::new()
        .get_async(
            //  /대법원/판례공보
            "/%EB%8C%80%EB%B2%95%EC%9B%90/%ED%8C%90%EB%A1%80%EA%B3%B5%EB%B3%B4",
            scourt_bulletin,
        )
        .run(req, env)
        .await
}

async fn scourt_bulletin(req: Request, ctx: RouteContext<()>) -> WorkerResult<Response> {
    let db = ctx.d1("DB")?;
    let if_modified_since = req.headers().get("If-Modified-Since")?;

    let channel_config = rss::RssChannelConfig {
        title: "대법원 판례공보",
        link: "asdf",
        description: "대법원 판례공보",
        language: Some("ko-KR"),
    };

    let items = match if_modified_since.as_ref() {
        Some(since) => db::get_scourt_bulletin_list_since(&db, &since).await,
        _ => db::get_scourt_bulletin_list(&db, 40).await,
    }
    .unwrap();

    if if_modified_since.is_some() && items.len() == 0 {
        return Ok(Response::empty()?.with_status(304));
    }

    let last_modified = items[0].get_pubdate();
    let body = rss::generate_rss(&channel_config, &items);
    let header = {
        let mut headers = Headers::new();
        headers.append("Last-Modified", &last_modified)?;
        headers
    };

    Ok(Response::ok(body)?.with_headers(header))
}
