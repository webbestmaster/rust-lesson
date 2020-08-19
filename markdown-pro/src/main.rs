pub mod markdown;

use markdown::markdownStruct::MarkdownConfigShallow;
use markdown::markdownConst::default_markdown_config;

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

    // println!("{:?}", config);

    println!("Hello, world!");
}
