//! markdown to htmlElement converter for https://yukicoder.me/
extern crate clap;

use log::info;
use std::time::Instant;

use clap::*;
use io::{output_to_file, read_from_file, read_from_stdin};
use pack::pack;

use yukicoder_md_lib::convert;

mod example;
mod io;
mod pack;

fn parse_args<'a>() -> clap::ArgMatches<'a> {
    app_from_crate!()
        .arg(
            Arg::with_name("input")
                .long("input")
                .short("i")
                .takes_value(true)
                .value_name("FILE")
                .help("Specifies input markdown file path. if not set, read from STDIN."),
        )
        .arg(
            Arg::with_name("output")
                .long("output")
                .short("o")
                .takes_value(true)
                .value_name("FILE")
                .help("Specifies output html fragment file path. if not set, output to STDOUT."),
        )
        .arg(
            Arg::with_name("package")
                .long("package")
                .short("p")
                .takes_value(true)
                .value_name("DIR")
                .help("Generates complete html package in <DIR>/ ."),
        )
        .arg(
            Arg::with_name("disable-image-conversion")
                .long("disable-image-conversion")
                .short("d")
                .help("Disables image to data-uri conversion"),
        )
        .arg(
            Arg::with_name("enable-template-engine")
                .long("enable-template-engine")
                .short("t")
                .help("Enables template engine preprocessing"),
        )
        .arg(
            Arg::with_name("example")
                .long("example")
                .short("e")
                .help("Prints markdown example file to stdout"),
        )
        .after_help(
            r#"example:
    // read from file, output to file
    $ yukicoder-md -i problem.md -o output.html

    // read from stdin, output to stdout. image conversion disabled
    $ yukicoder-md -d < problem.md > output.html
    
    // generate complete html package files into output/
    $ yukicoder-md -i problem.md -p output
    
    // generate example markdown file
    $ yukicoder-md -e > problem.md

    // enable template engine preprocessing
    $ yukicoder-md -t -i problem.md > output.html
    "#,
        )
        .get_matches()
}

fn main() {
    env_logger::Builder::new()
        .filter(Some(module_path!()), log::LevelFilter::Debug)
        .init();

    let args = parse_args();

    if args.is_present("example") {
        println!("{}", example::get_example());
        return;
    }

    let input = if let Some(input_file) = args.value_of("input") {
        read_from_file(input_file.to_string())
    } else {
        read_from_stdin()
    };

    let start_time = Instant::now();
    let result = convert(
        input,
        !args.is_present("disable-image-conversion"),
        !args.is_present("disable-template-engine"),
    );
    let end_time = Instant::now();
    info!(
        "conversion completed in {:?}",
        end_time.duration_since(start_time)
    );

    if let Some(package_dir) = args.value_of("package") {
        pack(package_dir.into(), result);
    } else if let Some(output_file) = args.value_of("output") {
        output_to_file(output_file.to_string(), result);
    } else {
        println!("{}", &result);
    }
}
