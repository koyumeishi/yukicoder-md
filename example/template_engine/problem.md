template engine を有効化して試してね

{% set A="Alice" %}
{% set B="Bob" %}

# statement
{{A}} と {{B}} がじゃんけんをします。  
それはそれとして、 $A+B$ を出力してください。

# input

```
$A$
$B$
```

$A,B$ は次の制約を満たします。

$0\leq A+B \leq 100$  
$|A|, |B| \leq 10^9$



# output
$A+B$ を一行に出力してください。 


# sample
<!-- サンプル入出力をファイルから -->
{{import_sample_input(file="sample_0.in")}}

{{import_sample_output(file="sample_0.out")}}

{{A}}, {{B}} は関係ありません。

