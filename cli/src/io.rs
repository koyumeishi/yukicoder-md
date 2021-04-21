use std::io::{Read, Write};

pub fn read_from_file(filename: String) -> String {
    let mut buf = String::with_capacity(1024 * 16);
    let path = std::path::Path::new(&filename);
    let mut f = std::fs::File::open(&path).unwrap();
    f.read_to_string(&mut buf).unwrap();
    buf
}

pub fn read_from_stdin() -> String {
    let mut buf = String::with_capacity(1024 * 16);
    let mut stdin = std::io::stdin();
    stdin
        .read_to_string(&mut buf)
        .expect("failed to read from stdin");
    buf
}

pub fn output_to_file(filename: String, s: String) {
    let path = std::path::Path::new(&filename);
    let mut f = std::fs::File::create(path).unwrap();
    write!(&mut f, "{}", s).unwrap();
    f.flush().unwrap();
}
