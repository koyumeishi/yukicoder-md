最初の "# ..." の行まで無視します。
    "# statement" => "# 問題文"
    "# input" => "# 入力"
    "# output" => "# 出力"
    "# sample" => "# サンプル"
の様に自動で変換します。
"# 見出し" の様にしてもかまいません。

`Enable Template Engine` にチェックを入れると
[Tera](https://tera.netlify.app/docs/#getting-started)
が使えるようになります。
構文に誤りがあると変換は中断されます。  
(エラーメッセージが役に立たないので頑張って見つけて…)
{\{ ←のバックスラッシュを外すとエラーになります }}

{%set A="アリス"%}
{%set B="ボブ"%}

*問題文ここから*

# statement
マークダウン形式で問題文を記述できます。  
[CommonMark](https://commonmark.org/) 準拠 + {テーブル, ~~打消し線~~}です。  
HTMLタグも使えますが [yukicoder](https://yukicoder.me/) で使えない ([ホワイトリスト](https://github.com/yuki2006/mark6/blob/master/mark6.go#L25)にない) 要素は自動で削除されます。
出力された HTML を yukicoder にコピペしてください。

1. <font color="red">red</font> **bold**
1. <marquee>{{A}} vs {{B}}</marquee>
1. <blink> blink はホワイトリストにないので消されます </blink>

---

TeX 記法も使えます。  
(**Playground では MathJax ではなく
KaTeX で表示しているため表示が異なる部分があります**)  

$A_i \leq 10^5$

<details>
    <summary> 🎍 </summary>
    <font size="100">Hello!</font>
</details>

# input
<!-- この下に入力フォーマットを記述してください。MathJaxが使えます。 -->
```
$N\ K$
$M$
```

ここに入力の説明、制約等を記述してください。


# output
ここに出力内容を記述してください。特殊な形式でなければなるべく問題文に記載してください  
最後に改行してください。


# sample
"sample-input" コードブロックの位置で
サンプルを自動で区切ります。

<!-- sample 1 -->
```sample-input
ここにサンプルケース1の入力を記述してください。
1
```

```sample-output
ここにサンプルケース1に対する解を記述してください。
0
```

ここにサンプルケース1に対する説明を記述してください。なければ空欄で構いません。

<!-- サンプルケース2ここから。不要であれば、「サンプルケース2ここまで」の行まで削除してください。 -->
```sample-input
1
```

```sample-output
0
```

<!-- サンプルケース2ここまで -->

<!-- サンプルケース3ここから。不要であれば、「サンプルケース3ここまで」の行まで削除してください。 -->
```sample-input
1
```

```sample-output
0
```
<!-- サンプルケース3ここまで -->
