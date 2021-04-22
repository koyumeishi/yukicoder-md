# Simple Problem Example

次のコマンドで `out` ディレクトリに HTML を出力します。  

```
$ yukicoder-md --input problem.md --package out
```

次のようにするとコピペする部分だけを出力します。

```
$ yukicoder-md --input problem.md --output copy_and_paste_me.txt
```

`--output` も `--package` も指定しないと標準出力へ吐き出します

```
$ yukicoder-md --input problem.md
```
