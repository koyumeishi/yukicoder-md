# With Image Example

画像は data-uri scheme (base64埋め込み) に自動変換します。  

```
$ yukicoder-md --input problem.md --package out
```

ローカルファイルを参照 (実行ディレクトリからの相対パス) 
しているだけなのでウェブ上のアドレスを参照しても読み込めません。  
MIME type も自動で設定するようにしています。

もしこの機能を無効化したい場合は `--disable-image-conversion` もしくは `-d`
フラグを渡してください。  
画像は \<img src="..."\> のような形で残します。 (ウェブ上の画像を参照したい場合とか？)

