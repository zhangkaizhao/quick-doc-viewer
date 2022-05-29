use std::io;
use std::path::Path;

use async_std::fs;
use chardetng::EncodingDetector;
use comrak::{ComrakOptions, markdown_to_html};
use rst_parser::parse as parse_rst;
use rst_renderer::render_html as render_rst_to_html;
use walkdir::{DirEntry, WalkDir};

use crate::constants::{
    COMMON_TEXT_FILE_EXTENSIONS,
    COMMON_IMAGE_FILE_EXTENSIONS,
    COMMON_AUDIO_FILE_EXTENSIONS,
    COMMON_VIDEO_FILE_EXTENSIONS,
    MARKDOWN_FILE_EXTENSIONS,
    SPECIAL_FILE_NAMES_LOWERCASE,
    IGNORED_DIRECTORIES,
};

pub fn is_markdown(file_name: &str) -> bool {
    let v: Vec<&str> = file_name.rsplitn(2, ".").collect();
    let ext = v[0];
    MARKDOWN_FILE_EXTENSIONS.contains(&ext.to_lowercase().as_str())
}

pub fn is_rst(file_name: &str) -> bool {
    let v: Vec<&str> = file_name.rsplitn(2, ".").collect();
    let ext = v[0];
    ext.to_lowercase().as_str() == "rst"
}

pub fn is_special_file(path: &str) -> bool {
    let file_name = path.strip_prefix("/").unwrap();
    let v: Vec<&str> = file_name.rsplitn(2, ".").collect();
    let base_name = if v.len() == 2 { v[1] } else { file_name };
    SPECIAL_FILE_NAMES_LOWERCASE.contains(&base_name.to_lowercase().as_str())
}

pub fn is_ignored_dir(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| IGNORED_DIRECTORIES.contains(&s))
        .unwrap_or(false)
}

pub struct Files {
    pub root: String,
    pub paths: Vec<String>,
    pub special_file_paths: Vec<String>,
}

impl Files {
    pub fn load(root: &str) -> Self {
        let mut file_paths = Vec::new();
        let mut special_file_paths = Vec::new();
        let walker = WalkDir::new(root).into_iter();
        for entry in walker.filter_entry(|e| !is_ignored_dir(e)) {
            let entry = entry.unwrap();
            if entry.file_type().is_file() {
                let mut path = entry.path().to_str().unwrap().to_owned();
                if path.starts_with("./") {
                    path = path[1..].to_string();
                }
                if is_special_file(&path) {
                    special_file_paths.push(path.clone());
                }
                file_paths.push(path);
            }
        }
        file_paths.sort_unstable();
        special_file_paths.sort_unstable();
        Files {
            root: root.to_owned(),
            paths: file_paths,
            special_file_paths: special_file_paths,
        }
    }
}

pub enum MimeType {
    Text,
    Image,
    Audio,
    Video,
    Other,
}

pub fn guess_mime_type(path: &str) -> MimeType {
    let v: Vec<&str> = path.rsplitn(2, ".").collect();
    let ext = v[0];
    let ext_lower = ext.to_lowercase();
    if COMMON_TEXT_FILE_EXTENSIONS.contains(&ext_lower.as_str()) {
        MimeType::Text
    } else if COMMON_IMAGE_FILE_EXTENSIONS.contains(&ext_lower.as_str()) {
        MimeType::Image
    } else if COMMON_AUDIO_FILE_EXTENSIONS.contains(&ext_lower.as_str()) {
        MimeType::Audio
    } else if COMMON_VIDEO_FILE_EXTENSIONS.contains(&ext_lower.as_str()) {
        MimeType::Video
    } else {
        // more text types
        let guess = mime_guess::from_path(path);
        if let Some(mime_type) = guess.first() {
            if mime_type.type_() == mime::TEXT {
                MimeType::Text
            } else {
                MimeType::Other
            }
        } else {
            MimeType::Other
        }
    }
}

pub async fn read_text_file_to_string(file_path: &Path) -> Result<String, io::Error> {
    // std::fs::read_to_string and async_std::fs::read_to_string only support utf-8 bytes.
    match fs::read_to_string(&file_path).await {
        Ok(content) => Ok(content),
        Err(e) => {
            if e.kind() != io::ErrorKind::InvalidData {
                return Err(e.into());
            }
            let bytes = fs::read(&file_path).await?;

            // decode bytes to utf-8 string
            let mut det = EncodingDetector::new();
            det.feed(&bytes[..], true);
            let enc = det.guess(None, false);
            let (content, _) = enc.decode_without_bom_handling(&bytes[..]);
            Ok(content.to_string())
        },
    }
}

pub fn markdown(content: &str) -> String {
    let mut options = ComrakOptions::default();
    // Enables strikethrough/tagfilter/table/autolink/tasklist extensions fro the GFM spec.
    options.extension.strikethrough = true;
    options.extension.tagfilter = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    // Enables comrak's own extensions.
    options.extension.superscript = true;
    // options.extension.header_ids = Some("qdv-".to_string());
    options.extension.header_ids = Some("".to_string());
    options.extension.footnotes = true;
    options.extension.description_lists = true;
    // options.extension.front_matter_delimiter = Some("---".to_owned());
    options.render.unsafe_ = true;
    // options.render.escape = true;
    markdown_to_html(content, &options)
}

pub fn rst_to_html(content: &str) -> String {
    // Note: there are a lot of `unimplemented!` in the rst_parser crate which cause panics here.
    match parse_rst(content) {
        Ok(document) => {
            let mut out: Vec<u8> = vec![];
            let standalone = false;
            match render_rst_to_html(&document, &mut out, standalone) {
                Ok(()) => String::from_utf8(out).unwrap(),
                Err(err) => {
                    format!(
                        "Failed to render file content as reStructuredText to html: {}",
                        err.to_string()
                    ).to_string()
                },
            }
        },
        Err(err) => {
            format!(
                "Failed to parse file content as reStructuredText: {}",
                err.to_string()
            ).to_string()
        }
    }
}
