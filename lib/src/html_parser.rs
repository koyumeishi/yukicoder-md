//! apply whitelist to html (allowed tags and attributes)
//! from https://github.com/yuki2006/mark6/blob/44604378f649ed21cc6e60daadbd7c4147433317/mark6.go#L25
extern crate html5ever;
extern crate markup5ever;
extern crate markup5ever_rcdom;

use html5ever::driver::ParseOpts;
use html5ever::tree_builder::{TreeBuilderOpts, TreeSink};
use html5ever::{parse_document, serialize};

use markup5ever::interface::tree_builder::NodeOrText::{AppendNode, AppendText};
use markup5ever::tendril::TendrilSink;
use markup5ever::Attribute;
use markup5ever_rcdom::{Handle, NodeData, RcDom, SerializableHandle};

use log::{info, warn};

use std::default::Default;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::whitelist::WHITELIST;

/// convert image source path to data uri scheme  
/// if failed to read file, just return original src
fn convert_img_to_base64(src: String) -> String {
    // already data uri scheme
    if src.starts_with("data:") {
        return src;
    }
    let path = Path::new(&src);
    let mime = mime_guess::from_path(&path);
    if let Some(m) = mime.first_raw() {
        let f = File::open(&path);
        if f.is_err() {
            warn!(r#"img source error: src="{}", {}"#, &src, &f.err().unwrap());
            return src;
        }

        let mut f = f.ok().unwrap();
        let mut buf = Vec::new();
        let _ = f.read_to_end(&mut buf).unwrap();

        let buf = base64::encode(buf);

        let res = format!("data:{mime_type};base64,{data}", mime_type = m, data = buf);
        return res;
    }
    src
}

/// walk on dom tree and sanitize elements
fn sanitize(doc: &mut RcDom, node: &Handle, par_node: &Handle, convert_image_flag: bool) {
    let next_par = match &node.data {
        NodeData::Text { contents } => {
            let contents = contents.borrow().clone();
            doc.append(&par_node, AppendText(contents));
            None
        }
        NodeData::Comment { contents } => {
            let contents = contents.clone();
            let e = doc.create_comment(contents);
            doc.append(&par_node, AppendNode(e));
            None
        }
        NodeData::Element { name, attrs, .. } => {
            let tag_name: String = name.local.to_string();
            if WHITELIST.contains_key(&tag_name) {
                let attr_list = &WHITELIST[&tag_name];
                let v: Vec<Attribute> = attrs
                    .borrow()
                    .iter()
                    .filter(|a| {
                        let attr_name = a.name.local.to_string();
                        attr_list.contains(&attr_name)
                    })
                    .cloned()
                    .map(|mut a| {
                        if convert_image_flag {
                            let attr_name = a.name.local.to_string();
                            if &tag_name == "img" && &attr_name == "src" {
                                let src = a.value.to_string();
                                let data = convert_img_to_base64(src);
                                a.value = data.into();
                            }
                        }
                        a
                    })
                    .collect();
                let e = doc.create_element(name.clone(), v, Default::default());
                doc.append(&par_node, AppendNode(e.clone()));
                Some(e)
            } else {
                info!("invalid element discarded: [{}]", &tag_name);
                None
            }
        }
        _ => {
            info!("node skipped : {:?}", &node.data);
            None
        }
    };

    let next_par = next_par.as_ref().unwrap_or(par_node);
    for ch in node.children.borrow().iter() {
        sanitize(doc, ch, &next_par, convert_image_flag);
    }
}

/// accept tags and attributes in WHITELIST
///
/// # e.g.
/// `<div style="...">text</div>` => `<div>text</div>`  
/// `<blink><p>text</p></blink>` => `<p>text</p>`
pub fn apply_whitelist(html_str: String, convert_image_flag: bool) -> String {
    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };

    // DOM like
    // <html><head></head><body>{contents}</body></html>
    let parsed_dom = parse_document(RcDom::default(), opts).one(html_str);

    let parsed_dom = &parsed_dom.document; // document
    let parsed_dom = &parsed_dom.children.borrow()[0]; // html
    let parsed_dom = &parsed_dom.children.borrow()[1]; // body

    let mut doc = RcDom::default();
    let root = doc.document.clone();

    sanitize(&mut doc, parsed_dom, &root, convert_image_flag);

    let mut bytes = vec![];
    let ser: SerializableHandle = doc.document.into();
    serialize(&mut bytes, &ser, Default::default()).expect("Error occured in serializing");
    String::from_utf8(bytes).unwrap()
}

#[test]
fn parse_and_sanitize_test() {
    let s = r#"
<div class="hoge" id="block">
    <span style="color:red">
        &text
    </span>
    <blink>blink_text</blink>
    <br />
    <span ok>
        text2 "ok"
    </span>
    <img src="/img/hoge.png">
</div>"#;

    let res = apply_whitelist(s.to_string(), true);
    eprintln!("{}", res);
}
