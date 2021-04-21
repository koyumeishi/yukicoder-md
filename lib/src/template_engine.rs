#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::hash_map::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use tera::{Context, Function, Tera, Value, to_value};

use log::warn;

pub fn import_from_file(path: &Path) -> String {
    match File::open(&path) {
        Ok(mut f) => {
            let mut s = String::new();
            f.read_to_string(&mut s).unwrap();
            s
        }
        _ => {
            let s = format!("faild to import file: {}", path.to_str().unwrap());
            warn!("{}", &s);
            s
        }
    }
}

pub fn apply_template_engine(input: String) -> String {
    let context = Context::new();
    let mut tera = Tera::default();
    tera.register_function(
        "import",
        |args: &HashMap<String, Value>| -> tera::Result<Value> {
            match args.get("file") {
                Some(Value::String(filename)) => {
                    let res = import_from_file(Path::new(filename));
                    Ok(to_value(res).unwrap())
                }
                _ => {
                    let res = "invalid function usage : import(file=\"path/to/file\")".to_string();
                    Ok(to_value(res).unwrap())
                }
            }
        },
    );

    let wrapped_import = |input_type: &'static str| {
        move |args: &HashMap<String, Value>| -> tera::Result<Value> {
            match args.get("file") {
                Some(Value::String(filename)) => {
                    let res = import_from_file(Path::new(filename));
                    let res = res.trim_end().to_string();
                    let res = format!("```{}\n{}\n```\n", input_type, res);
                    Ok(to_value(res).unwrap())
                }
                _ => {
                    let res = "invalid function usage : import_sample_input(file=\"path/to/file\")".to_string();
                    Ok(to_value(res).unwrap())
                }
            }
        }
    };

    tera.register_function(
        "import_sample_input",
        wrapped_import("sample-input")
    );
    tera.register_function(
        "import_sample_output",
        wrapped_import("sample-output")
    );

    tera.render_str(input.as_str(), &context).unwrap()
}

#[test]
fn template_test() {
    let s = r##"
{%set person_0 = "Takahashi" %}
{%set person_1 = "Aoki" %}

# problem
<p>
    {{person_0}} vs {{person_1}}  
</p>

{{ import_sample_input(file="Cargo.toml") }}

{{ import_sample_output(files="Cargo.toml") }}

"##
    .to_string();

    eprintln!("{}", apply_template_engine(s));
}
