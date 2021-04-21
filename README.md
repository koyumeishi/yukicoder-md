# yukicoder-md
[CommonMark](https://commonmark.org/) 形式の markdown ファイルを [yukicoder.me](https://yukicoder.me/) の問題用 HTML に変換する非公式ツールです。  
yukicoder で使えない ([ホワイトリスト](https://github.com/yuki2006/mark6/blob/master/mark6.go#L25)にない) 要素は自動で削除されます。  
`![alt text](image.png)` のような画像要素は自動で data-uri scheme に変換して埋め込みます。  

このプログラムは Rust で実装されています。

# Playground
// TODO
このプロジェクトを wasm でブラウザ上から使えるようにした [playground]() を用意しました。  
画像の自動変換等一部機能は使えません。


# Download
// TODO  
ビルド済みバイナリ


# Installation
自分でコードからビルドする場合はこちら。 rust の実行環境が必要です。 ビルドには数分の時間がかかると思います。  

// TODO
```
$ cargo install ...
```

# Uninstallation
// TODO
```
$ cargo uninstall ...
```

# Usage
```bash
$ yukicoder-md --help

yukicoder-md 0.1.0
koyumeishi <koyumeishi+github@gmail.com>


USAGE:
    yukicoder-md.exe [FLAGS] [OPTIONS]

FLAGS:
    -d, --disable-image-conversion    Disables image to data-uri conversion
    -f, --disable-template-engine     Disables template engine preprocessing
    -e, --example                     Prints markdown example file to stdout
    -h, --help                        Prints help information
    -V, --version                     Prints version information

OPTIONS:
    -i, --input <FILE>     Specifies input markdown file path. if not set, read from STDIN.
    -o, --output <FILE>    Specifies output html fragment file path. if not set, output to STDOUT.
    -p, --package <DIR>    Generates complete html package in <DIR>/ .

example:
    // read from file, output to file
    $ yukicoder-md -i problem.md -o output.html

    // read from stdin, output to stdout. image conversion disabled
    $ yukicoder-md -d < problem.md > output.html
    
    // generate complete html package files into output/
    $ yukicoder-md -i problem.md -p output
    
    // generate example markdown file
    $ yukicoder-md -e > problem.md

    // disable template engine preprocessing
    $ yukicoder-md -f -i problem.md > output.html
    
```

デフォルトでは標準入力で markdown 形式 (CommonMark) の文字列を受け取り、
標準出力に変換後の HTML を出力します。
出力された HTML を yukicoder の問題ページにコピペしてください。 
オプションで入出力ファイルを指定することもできます。  

出力される HTML はコピペする部分だけで不完全なものなので
ブラウザで開いてもちゃんと表示されません。  

`--package <DIR>` オプションを指定すると指定したディレクトリに
完全な HTML + CSS ファイルを出力します。
ブラウザで `<DIR>/index.html` を開くと yukicode っぽいページが表示されます。


# Pipeline
変換処理は次のような順に行われています。  

1. テンプレートエンジン [tera](https://github.com/Keats/tera) による事前処理  
2. 最初の "# ..." まで削除
3. "# ..." で分割 (div.block で囲まれる)
4. サンプルブロックの分割 (サンプル1,2,...)
5. HTML へ変換
6. ホワイトリストにないタグ、属性の削除
7. 画像の data-uri scheme への変換 (CLI版のみ)
8. 出力
