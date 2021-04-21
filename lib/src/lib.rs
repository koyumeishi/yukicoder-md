use template_engine::apply_template_engine;

#[macro_use]
extern crate lazy_static;

extern crate tera;

mod html_parser;
mod md_parser;
mod template_engine;
mod whitelist;

pub fn convert(input: String, convert_image: bool, use_template_engine: bool) -> String {
    let input = if use_template_engine {
        apply_template_engine(input)
    } else {
        input
    };
    let input = md_parser::convert(input);
    html_parser::apply_whitelist(input, convert_image)
}
