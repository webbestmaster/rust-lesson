pub mod markdown;

use markdown::markdownStruct::MarkdownConfigShallow;
use markdown::markdownConst::default_markdown_config;
use markdown::markdownPro;

fn main() {
    let key_fn = | lang_name: &str, code: &str | -> String {
        "my str".to_string()
    };

    let config = MarkdownConfigShallow {
        use_line_break: Some(false),
        wrapper_class_name: Some("my_class"),
        parse_link: None,
        use_wrapper: Some(true),
    };

    let html = markdownPro();

    println!("html is here ---> {}", html);

    println!("Hello, world!");
}
