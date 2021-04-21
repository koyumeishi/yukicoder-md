use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/example/"]
struct Asset;

/// get "assets/sample/sample.md" as String
pub fn get_example() -> String {
    let s = Asset::get("example.md").unwrap();
    String::from_utf8(s.to_vec()).unwrap()
}
