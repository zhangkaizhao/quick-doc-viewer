use maud::{DOCTYPE, html, Markup, PreEscaped};

#[derive(Clone)]
pub struct Templates {
    nav: Markup,
}

impl Templates {
    pub fn build(special_file_paths: &Vec<String>) -> Self {
        Templates {
            nav: Templates::nav(special_file_paths),
        }
    }

    fn nav(special_file_paths: &Vec<String>) -> Markup {
        html! {
            nav {
                a href="/" { "Files" }
                " |"
                @for file_path in special_file_paths {
                    " "
                    a href=(file_path) { (file_path) }
                    " |"
                }
            }
        }
    }

    fn style(&self) -> Markup {
        html! {
            "pre,code {background: #fef;}"
            ".actions a {margin: 2px;}"
        }
    }

    fn header(&self) -> Markup {
        html! {
            header {
                (self.nav)
            }
        }
    }

    fn footer(&self) -> Markup {
        html! {
            footer {
                "Powered by "
                a href="https://github.com/zhangkaizhao/quick-doc-viewer" { "Quick doc viewer" }
            }
        }
    }

    fn layout(&self, title: &str, content: Markup) -> Markup {
        html! {
            (DOCTYPE)
            html {
                head {
                    meta charset="utf-8";
                    title { (title) }
                    style {
                        (self.style())
                    }
                }
                body {
                    (self.header())
                    h1 { (title) }
                    (content)
                    (self.footer())
                }
            }
        }
    }

    fn actions(&self, path: &str) -> Markup {
        let raw_url = path.to_string() + "?raw=true";
        let markdown_url = path.to_string() + "?format=markdown";
        let rst_url = path.to_string() + "?format=restructuredtext";
        let plain_text_url = path.to_string() + "?format=plain_text";
        html! {
            div class="actions" {
                a href=(raw_url) { "raw" }
                a href=(path) { "preview" }
                a href=(markdown_url) { "preview as Markdown" }
                a href=(rst_url) { "preview as reStructuredText" }
                a href=(plain_text_url) { "preview as plain text" }
            }
        }
    }

    pub fn files(&self, file_paths: &Vec<String>) -> Markup {
        self.layout("/", html! {
            ul {
                @for file_path in file_paths {
                    li {
                        a href=(file_path) { (file_path) }
                    }
                }
            }
        })
    }

    pub fn page(&self, file_path: &str, html_content: &str) -> Markup {
        self.layout(file_path, html! {
            div {
                (self.actions(file_path))
                div { (PreEscaped(html_content)) }
            }
        })
    }

    pub fn text(&self, file_path: &str, file_content: &str) -> Markup {
        self.layout(file_path, html! {
            div {
                (self.actions(file_path))
                div {
                    pre { (file_content) }
                }
            }
        })
    }
}
