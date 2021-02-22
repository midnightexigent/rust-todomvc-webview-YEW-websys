use std::error::Error;

use rust_embed::RustEmbed;
use warp::{http::header::HeaderValue, path::Tail, reply::Response, Filter, Rejection, Reply};

#[derive(RustEmbed)]
#[folder = "static/"]
struct Asset;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (addr, fut) = warp::serve(
        warp::path::end()
            .and_then(|| async { get_asset("index.html") }) //retrun the index.html contents if nothing is requested
            .or(warp::get()
                .and(warp::path::tail())
                .and_then(|v: Tail| async move { get_asset(v.as_str()) })), //return the other files' contents when they're requested
    )
    .bind_ephemeral(([127, 0, 0, 1], 0)); // this could be a .run().await and go in the tokio::spawn if the port was known in advance. but this allows us to let the os decide which port to bind, then get it back
    tokio::spawn(async {
        fut.await; // run the server on a different thread
    });

    // run the webview
    web_view::builder()
        .title("webview yew todomvc")
        .content(web_view::Content::Url(format!(
            "http://127.0.0.1:{}",
            addr.port() // get the content from the server created above. the port is automatically assigned by the os
        )))
        .size(600, 400)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .map_err(|err| err.into())
}

fn get_asset(path: &str) -> Result<impl Reply, Rejection> {
    let asset = Asset::get(path).ok_or_else(warp::reject::not_found)?; //load the embedded asset thanks to the rust-embed lib
    let mime = mime_guess::from_path(path).first_or_octet_stream(); // guess the content type from path

    let mut res = Response::new(asset.into());
    res.headers_mut().insert(
        warp::hyper::header::CONTENT_TYPE,
        HeaderValue::from_str(mime.as_ref()).unwrap(),
    );
    Ok(res)
}
