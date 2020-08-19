use super::markdown_struct;

pub const DEFAULT_MARKDOWN_CONFIG: markdown_struct::MarkdownConfig = markdown_struct::MarkdownConfig {
    use_line_break: false,
    wrapper_class_name: "md-pro",
    parse_link: true,
    use_wrapper: true,
};
