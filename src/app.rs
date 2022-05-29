use std::io;
use std::path::Path;
// for `Mime::from_str`
use std::str::FromStr;

use http_types::Mime;
use tide::{Body, Request, Response, Result, StatusCode};
use urlencoding::decode as url_decode;

use crate::{
    Files,
    MimeType,
    PageQuery,
    Templates,
    guess_mime_type,
    is_markdown,
    is_rst,
    read_text_file_to_string,
    markdown,
    rst_to_html,
};

#[derive(Clone)]
pub struct State {
    storage_root: String,
    file_paths: Vec<String>,
    templates: Templates,
}

async fn files(req: Request<State>) -> Result<impl Into<Response>> {
    // read all files and generate index
    let state = req.state();
    let file_paths = &state.file_paths;
    let templates = &state.templates;

    let html_content = templates.files(file_paths);
    Ok(html_content)
}

async fn page(req: Request<State>) -> Result<impl Into<Response>> {
    // render preview or raw
    let p: PageQuery = req.query()?;
    if p.raw {
        raw(req).await
    } else {
        preview(req).await
    }
}

async fn preview(req: Request<State>) -> Result<Response> {
    let state = req.state();
    let storage_root = &state.storage_root;
    let file_paths = &state.file_paths;
    let templates = &state.templates;

    let path = req.url().path();

    let path_string = path.to_string();

    // file path may be encoded
    let path_string = url_decode(&path_string).unwrap().into_owned();

    if !file_paths.contains(&path_string) {
        return Ok(Response::new(StatusCode::NotFound));
    }

    let file_path = Path::new(&storage_root.to_owned()).join(&path_string.strip_prefix('/').unwrap());
    let content = read_text_file_to_string(&file_path).await?;

    // prewiew as selected format, currently support Markdown

    let p: PageQuery = req.query()?;
    match p.format.as_str() {
        "markdown" => {
            let content = markdown(&content);
            let html_content = templates.page(&path_string, &content).into_string();
            return Ok(html_response(html_content));
        },
        "restructuredtext" => {
            let content = rst_to_html(&content);
            let html_content = templates.page(&path_string, &content).into_string();
            return Ok(html_response(html_content));
        },
        "plain_text" => {
            let html_content = templates.text(&path_string, &content).into_string();
            return Ok(html_response(html_content));
        },
        _ => {},
    }

    // guess mime type of the file to determine preview format
    // if mime type of the file is other, send raw of file

    let mime_type = guess_mime_type(&path_string);
    match mime_type {
        MimeType::Text => {
            let html_content = if is_markdown(&path_string) {
                let content = markdown(&content);
                templates.page(&path_string, &content).into_string()
            } else if is_rst(&path_string) {
                let content = rst_to_html(&content);
                templates.page(&path_string, &content).into_string()
            } else {
                templates.text(&path_string, &content).into_string()
            };
            Ok(html_response(html_content))
        },
        _ => {
            raw(req).await
        },
    }
}

fn html_response(html_content: String) -> Response {
    let mime = Mime::from_str("text/html;charset=utf-8").unwrap();
    Response::builder(200)
        .body(html_content)
        .content_type(mime)
        .build()
}

async fn raw(req: Request<State>) -> Result<Response> {
    let state = req.state();
    let storage_root = &state.storage_root;
    let file_paths = &state.file_paths;

    let path = req.url().path();

    let path_string = path.to_string();

    // file path may be encoded
    let path_string = url_decode(&path_string).unwrap().into_owned();

    if !file_paths.contains(&path_string) {
        return Ok(Response::new(StatusCode::NotFound));
    }

    let file_path = Path::new(&storage_root.to_owned()).join(&path_string.strip_prefix('/').unwrap());

    // copied from tide 0.16.0 src/fs/serve_file.rs

    match Body::from_file(&file_path).await {
        Ok(body) => Ok(Response::builder(StatusCode::Ok).body(body).build()),
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            Ok(Response::new(StatusCode::NotFound))
        }
        Err(e) => Err(e.into()),
    }
}

pub fn make_app() -> tide::Server<State> {
    let files_ = Files::load(".");
    let file_paths = files_.paths;

    let templates = Templates::build(&files_.special_file_paths);

    let state = State {
        storage_root: ".".to_string(),
        file_paths: file_paths,
        templates: templates,
    };

    let mut app = tide::with_state(state);
    app.at("/").get(files);
    app.at("/*").get(page);
    app
}
