//! convert markdown to html
use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag};

use log::info;
use std::iter;

/// process iterator and return subtree nodes with itr fowarding
fn get_subtree<'a, I>(itr: &mut I) -> Vec<Event<'a>>
where
    I: Iterator<Item = Event<'a>>,
{
    let mut res = vec![];
    let mut depth = 0;

    #[allow(clippy::clippy::while_let_on_iterator)]
    while let Some(e) = itr.next() {
        match &e {
            Event::Start(_) => {
                depth += 1;
            }
            Event::End(_) => {
                depth -= 1;
            }
            _ => {}
        }
        res.push(e);

        if depth == 0 {
            break;
        }
    }
    res
}

/// add heading level  
/// `#  => ####`  
/// `## => #####`  
fn increase_heading_level<'a, I>(itr: I) -> Vec<Event<'a>>
where
    I: Iterator<Item = Event<'a>>,
{
    let offset = 3;
    itr.map(|e| match &e {
        Event::Start(Tag::Heading(level)) => Event::Start(Tag::Heading(level + offset)),
        Event::End(Tag::Heading(level)) => Event::End(Tag::Heading(level + offset)),
        _ => e,
    })
    .collect()
}

/// split sapmles into blocks by 'sample-input' code blocks
fn split_samples<'a, I>(parser: I) -> Vec<Event<'a>>
where
    I: Iterator<Item = Event<'a>>,
{
    let mut res: Vec<Event> = vec![];
    let mut count = 0;

    let mut pending: Vec<Event> = vec![];

    // make pending into one block
    let resolve = |pending: &mut Vec<Event<'a>>, res: &mut Vec<Event<'a>>, count: &mut i32| {
        if pending.is_empty() {
            return;
        }
        *count += 1;
        res.push(Event::Html(format!(
            "<div class=\"sample\"> <h5 class=\"underline\">サンプル{}</h5> <div class=\"paragraph\"> ",
            count).into()));
        res.append(pending); // pending.len() == 0
        res.push(Event::Html("</div></div>\n".into()));
    };

    let mut parser = parser.peekable();
    while let Some(e) = parser.peek() {
        match e {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => match lang.as_ref() {
                "sample-input" => {
                    resolve(&mut pending, &mut res, &mut count);
                    let before = Event::Html("<h6>入力</h6><pre>\n".into());
                    let after = Event::Html("</pre>".into());
                    let mut g = get_subtree(&mut parser);
                    pending.append(
                        &mut iter::once(before)
                            .chain(g.drain(1..g.len() - 1))
                            .chain(iter::once(after))
                            .collect(),
                    );
                }
                "sample-output" => {
                    let before = Event::Html("<h6>出力</h6><pre>".into());
                    let after = Event::Html("</pre>".into());
                    let mut g = get_subtree(&mut parser);
                    pending.append(
                        &mut iter::once(before)
                            .chain(g.drain(1..g.len() - 1))
                            .chain(iter::once(after))
                            .collect(),
                    );
                    // pending.append(&mut wrap_subtree(&mut parser, iter::once(before), iter::once(after)));
                }
                _ => {
                    if !pending.is_empty() {
                        pending.push(parser.next().unwrap());
                    } else {
                        res.push(parser.next().unwrap());
                    }
                }
            },
            _ => {
                if !pending.is_empty() {
                    pending.push(parser.next().unwrap());
                } else {
                    res.push(parser.next().unwrap());
                }
            }
        }
    }
    resolve(&mut pending, &mut res, &mut count);

    res
}

/// h4 => h4.shadow
fn decorate_h4<'a, I>(parser: I) -> Vec<Event<'a>>
where
    I: Iterator<Item = Event<'a>>,
{
    parser
        .map(|e| match &e {
            Event::Start(Tag::Heading(4)) => Event::Html(r#"<h4 class="shadow">"#.into()),
            Event::End(Tag::Heading(4)) => Event::Html("</h4>\n".into()),
            _ => e,
        })
        .collect()
}

/// wrap segment by div.block
fn wrap_by_block<'a, I: Iterator<Item = Event<'a>>>(parser: I) -> Vec<Event<'a>> {
    let before = Event::Html("<div class=\"block\">\n".into());
    let after = Event::Html("\n</div>\n".into());
    iter::once(before)
        .chain(parser)
        .chain(iter::once(after))
        .collect()
}

/// convert fenced code blocks to pre block
///
/// pulldown-cmark converts code blocks to "pre > code" blocks
/// but mathjax is disabled on <code> blocks for yukicoder
/// indented code blocks are unchanged
fn convert_fenced_codeblocks<'a, I>(parser: I) -> Vec<Event<'a>>
where
    I: Iterator<Item = Event<'a>>,
{
    parser
        .map(|e| match &e {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(_))) => Event::Html("<pre>".into()),
            Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(_))) => Event::Html("</pre>".into()),
            _ => e,
        })
        .collect()
}

/// split statements into blocks by h1 line
/// this method ignores texts before first h1 line
///
/// auto conversion applied
/// `# statement` => `# 問題文`
/// `# input`     => `# 入力`
/// `# output`    => `# 出力`
/// `# sample`    => `# サンプル`
fn preprocess(s: &str) -> Vec<String> {
    let mut blocks: Vec<String> = vec![];
    let mut pending: Vec<String> = vec![];
    let resolve = |a: &mut Vec<String>, b: &mut Vec<String>| {
        if !a.is_empty() {
            b.push(a.join("\n"));
        }
    };

    for line in s.lines() {
        match line {
            "# statement" => {
                resolve(&mut pending, &mut blocks);
                pending = vec![String::from("# 問題文")];
            }
            "# input" => {
                resolve(&mut pending, &mut blocks);
                pending = vec![String::from("# 入力")];
            }
            "# output" => {
                resolve(&mut pending, &mut blocks);
                pending = vec![String::from("# 出力")];
            }
            "# sample" => {
                resolve(&mut pending, &mut blocks);
                pending = vec![String::from("# サンプル")];
            }
            _ => {
                if line.starts_with("# ") {
                    // another h1 level
                    resolve(&mut pending, &mut blocks);
                    pending = vec![line.into()];
                } else if !pending.is_empty() {
                    pending.push(line.into());
                } else {
                    // skip
                    info!("line skipped before the first heading: '{}'", &line);
                }
            }
        }
    }
    resolve(&mut pending, &mut blocks);
    blocks
}

/// convert markdown to html string
pub fn convert(input: String) -> String {
    let mut option = Options::empty();
    option.insert(Options::ENABLE_STRIKETHROUGH);
    option.insert(Options::ENABLE_TABLES);
    let s = input.as_str();

    let v = preprocess(s);
    let b: Vec<String> = v
        .iter()
        .map(|block| {
            let parser = Parser::new_ext(block.as_str(), option);
            let parser = increase_heading_level(parser).into_iter();
            let parser = if block.starts_with("# サンプル") {
                split_samples(parser).into_iter()
            } else {
                parser
            };
            let parser = decorate_h4(parser).into_iter();
            let parser = convert_fenced_codeblocks(parser).into_iter();
            let parser = wrap_by_block(parser).into_iter();
            let mut html_out = String::with_capacity(s.len() * 3 / 2);
            html::push_html(&mut html_out, parser);

            html_out
        })
        .collect();

    b.join("\n")
}

#[test]
fn md_test() {
    let s: String = r###"
igonred message begin

hogehoge

end
# statement
"this is a pen" << wow!  
a & b&c  <>


end of the statement block
# sample
message before sample 1
```sample-input
a b c
```
sample 1 input description

```sample-output
10000
```
sample 1 output description

```sample-input
this is sample 2
```

```sample-output
10000
```

end of the sample block
"###
    .to_string();
    let res = convert(s);
    eprintln!("{}", res);
}
