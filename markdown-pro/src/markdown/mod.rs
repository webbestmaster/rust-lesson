pub mod markdown_struct;
pub mod markdown_const;

use markdown_struct::MarkdownConfig;

pub fn markdown_pro(_markdown_text: &str, _config: MarkdownConfig) -> String {
    "me string".to_string()
}
