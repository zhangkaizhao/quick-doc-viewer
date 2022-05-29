use async_std::prelude::*;
use tide;

use quick_doc_viewer::make_app;

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let app = make_app();
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
