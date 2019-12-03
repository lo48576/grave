# rich-quote-toml

没: 2019-12-04

AsciiDoc の blockquote 記法が意味・見た目ともにあまりに貧弱なのが不満で書いた、 asciidoctor 用の拡張。
ブログで使う予定だったが破棄。

## 概要

```
[richquote]
====
# Quoted text.
# String.
text = '''

# Display name of the creator of the quoted content (optional).
# AsciiDoc string.
creator = "@user"

Quoted text in AsciiDoc format.
'''
# Document or event where the quoted content (firstly) appeared.
# AsciiDoc string.
source = 'Quote source in inline AsciiDoc format.'

# URI of the quote source, if available.
# Array of strings or string.
uri = 'https://example.com/'

# Published date of the quoted content (optional).
published = '2000-01-01T00:00:00+09:00'
# Updated date of the quoted content (optional).
updated = '2000-01-01T00:00:00+09:00'
# Datetime when the content is referred and quoted.
referred = '2000-01-01T00:00:00+09:00'

# Note about the quote (optional).
# AsciiDoc string.
note = '*emphasis* added'
====
```

コメントそのまま。
これがたとえば以下のような HTML に変換されてほしかった。

```html
<figure>
  <blockquote cite="https://example.com/">text</blockquote>
  <figcaption>@user, link:https://example.com/[source], *emphasis* added</figcaption>
</figure>
```

## 死因

### やることの割に複雑

まずインラインの AsciiDoc テキストを HTML に変換する方法が面倒すぎるというのも含め、全体的にノードを弄る処理が多い。
やっていることは単に断片を普通に変換したうえで並び換えて HTML テンプレートに嵌めるだけなのだが、コードが十分に煩雑だったのでメンテしたくなかった。

### あまりに AsciiDoc の文法と異なっている

拡張がない状態でも意味を保てるよう、拡張はできるだけシンプルな (可能なら何も処理しない) 変換でコンテンツを生成するのが好ましい。
しかし内容を toml で記述すると、拡張のコードなしには出力を一切制御できなくなる。
これはデータがコードに密結合になる点で大変よろしくない。

### 他の手を思い付いた

これは文法を似せたいという話とも絡む。
もうちょっと microdata や RDFa に近い形で、「blockquote 中に特定のブロック (たとえば `[quotemeta]` みたいな) を含めるとそれが caption 扱いされる」みたいな方式にした方が、フォールバック等が楽になりそうだという思い付きを得た。
そちらで実装してみる予定なのでこのコードは没。
