# yukicoder-md
[CommonMark](https://commonmark.org/) 形式の markdown ファイルを [yukicoder.me](https://yukicoder.me/) の問題用 HTML に変換する非公式ツールです。  
yukicoder で使えない ([ホワイトリスト](https://github.com/yuki2006/mark6/blob/master/mark6.go#L25)にない) 要素は自動で削除されます。  
`![alt text](image.png)` のような画像要素は自動で data-uri scheme に変換して埋め込みます。  

このプログラムは Rust で実装されています。

# Playground
このプロジェクトを wasm でブラウザ上から使えるようにした [Playground](https://koyumeishi.github.io/yukicoder-md/) を用意しました。  
画像の自動変換等一部機能は使えません。


# Download
Windows / Linux 向けのビルド済みの実行ファイルは [Release](https://github.com/koyumeishi/yukicoder-md/releases) から入手できます。  
Mac (動作確認できる環境がない) の方や配布している実行ファイルが動かない方は
お手数ですが下記の方法でインストールして下さい。  


# Installation
自分でコードからビルドする場合はこちら。 rust の実行環境が必要です。 ビルドには数分の時間がかかると思います。  

```
$ cargo install --git https://github.com/koyumeishi/yukicoder-md --branch main
```

# Uninstallation

```
$ cargo uninstall yukicoder-md
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
    -t, --enable-template-engine      Enables template engine preprocessing
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

    // enable template engine preprocessing
    $ yukicoder-md -t -i problem.md > output.html
    
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

1. テンプレートエンジン [tera](https://github.com/Keats/tera) による事前処理 (有効化した場合)  
   tera (jinja2 / django に近いらしい?) の標準的な機能に加えて、CLI 版は次の機能が使える  
   + `{{ import(file="path/to/file") }}`  
     "path/to/file" をここに展開する (再帰的にテンプレートエンジン適用はしないので注意 (いまのところ))
   + `{{ import_sample_input(file="path/to/file") }}`  
     "path/to/file" をサンプル入力のブロックとして展開する
   + `{{ import_sample_output(file="path/to/file") }}`  
     "path/to/file" をサンプル出力のブロックとして展開する
3. 最初の "# ..." まで削除
4. "# ..." で分割 (div.block で囲まれる) 
5. "# ..." の自動変換 (以下に該当しない場合はそのまま)  
    + "# statement" -> "# 問題文"
    + "# input" -> "# 入力"
    + "# output" -> "# 出力"
    + "# sample" -> "# サンプル"
6. "# ..." が \<h4\> に対応するように変換  
   "## ..." は \<h5\> ... 以下同様に変換
8. サンプルブロックの分割 (サンプル1,2,...)
9. HTML へ変換
10. ホワイトリストにないタグ、属性の削除 (簡易的なもので公式のと完全一致ではない)
11. 画像の data-uri scheme への変換 (無効化していない場合, CLI版のみ)
12. 出力

# License
このプロジェクトのコードのライセンスは CC0 とします。 自由に改変・再配布可能です。  
[CC0 1.0 Universal (CC0 1.0) Public Domain Dedication](https://creativecommons.org/publicdomain/zero/1.0/deed.ja)

ただしプレビュー用に同梱しているスタイルシート (base.css, problems.show.css) 
は yukicoder から引っ張ってきているものなので例外とし、
ライセンス明記しません。 (再配布の許可はいただいております)  
また、このリポジトリにはビルドされた WebAssembly ファイルや JavaScript ファイルが含まれますが、
内部で使用されているライブラリのライセンスは各ライブラリ準拠になります。
