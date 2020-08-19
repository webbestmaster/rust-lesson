pub mod markdown;

use markdown::markdown_struct::MarkdownConfigShallow;
// use markdown::markdown_const::DEFAULT_MARKDOWN_CONFIG;
use markdown::markdown_pro;

fn main() {
    let _key_fn = | _lang_name: &str, _code: &str | -> String {
        "my str".to_string()
    };

    let config = MarkdownConfigShallow {
        use_line_break: Some(false),
        wrapper_class_name: Some("my_class"),
        parse_link: None,
        use_wrapper: Some(true),
    };

    let html = markdown_pro();

    println!("html is here ---> {}", html);

    println!("{:?}", config);
}
