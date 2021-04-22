# Template Engine Example

次のコマンドで `out` ディレクトリに HTML を出力します。   
この例ではテンプレートエンジンを利用して変数の割当、ファイルからの展開をしています。  

```
$ yukicoder-md --enable-template-engine --input problem.md --package out
```

省略形

```
$ yukicoder-md -t -i problem.md -p out
```

テンプレートエンジンには
[Tera](https://tera.netlify.app/docs/)
を使っています。  

独自関数として次のようなものも使えます。 
(ローカルファイルを参照するので基本的には CLI 版のみの機能)

```
<!-- hoge.md の中身をここに展開 -->
{{import(file="hoge.md")}}
```

```
<!-- sample_0.in の中身をサンプル入力のコードブロックとして展開 -->
{{import_sample_input(file="sample_0.in")}}

<!-- sample_0.out の中身をサンプル出力のコードブロックとして展開 -->
{{import_sample_output(file="sample_0.out")}}
```

ファイルパスは `yukicoder-md` を実行したところからの相対パスまたは絶対パスを指定してください。  
今のところ展開されたファイルの中身を再帰的に展開するようにはなっていません。 
(需要があるならオプション付けるので言って)  
