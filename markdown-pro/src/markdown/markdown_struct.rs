// TODO: impl code_highlight

#[derive(Debug)]
pub struct MarkdownConfigShallow<'a> {
    pub use_line_break: Option<bool>,
    pub wrapper_class_name: Option<&'a str>,
    pub parse_link: Option<bool>,
    // pub code_highlight: Option<fn(lang_name: &str, code: &str) -> String>,
    pub use_wrapper: Option<bool>,
}

#[derive(Debug)]
pub struct MarkdownConfig<'a> {
    pub use_line_break: bool,
    pub wrapper_class_name: &'a str,
    pub parse_link: bool,
    // pub code_highlight: fn(lang_name: &str, code: &str) -> String,
    pub use_wrapper: bool,
}

// impl MarkdownConfigShallow {
//     fn code_highlight(self, lang_name: &str, code: &str) -> String {
//         String::default()
//     }
// }
