//! pack output html fragment into template.html
//!
//! resource files
//! ```text
//! assets/pack/template.html
//! assets/pack/css/base.css
//! assets/pack/css/problems.show.css
//! assets/pack/css/bootstrap-sortable.css
//! ```
//!
//! output package
//! ```text
//! <DIR>/index.html
//! <DIR>/css/base.css
//! <DIR>/css/problems.show.css
//! <DIR>/css/bootstrap-sortable.css
//! ```

use std::io::Write;

use log::info;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/pack/"]
struct Asset;

#[test]
fn embed_test() {
    pack("output_package".into(), "hoge\nhoge\n".into());
}

/// output complete html contents into target_dir
pub fn pack(target_dir: String, contents: String) {
    let target_path = std::path::Path::new(&target_dir);

    for item in Asset::iter() {
        let item = item.as_ref();
        let s = Asset::get(item).unwrap();

        let item = if item == "template.html" {
            "index.html"
        } else {
            item
        };
        let pb = target_path.join(&item);
        let path = pb.as_path();
        if let Some(dir) = path.parent() {
            if !dir.is_dir() {
                info!("creating dir: {}", dir.to_str().unwrap());
                std::fs::create_dir_all(&dir).unwrap();
            }
            assert!(dir.is_dir());
        }

        let f = std::fs::File::create(&path).unwrap();
        let mut bf = std::io::BufWriter::new(f);

        if item == "index.html" {
            let s = String::from_utf8(s.to_vec()).unwrap();
            let lines = s.lines();
            for l in lines {
                if l == "<!-- content -->" {
                    bf.write_all(contents.as_bytes()).unwrap();
                } else {
                    bf.write_all(l.as_bytes()).unwrap();
                }
                bf.write_all("\n".as_bytes()).unwrap();
            }
        } else {
            bf.write_all(s.as_ref()).unwrap();
        }
        bf.flush().unwrap();
    }
}
