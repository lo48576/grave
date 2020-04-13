# String types with small string optimization capability

没: 2020-04-12

特定の feature が有効化されているときだけ small string optimization が有効になるような文字列型を作りたかった。
でも微妙。

## 概要

std の `String` のように使える文字列 `SmallString` 型のようにしたかった。

* `string.rs` は `inlinable_string ^0.1.11` を使って `String` っぽい型を用意したもの。
* `string2.rs` はとりあえず `String` っぽいものを作っただけで中身はそのまま `String` なもの。

## 死因

### 汎用のオレオレ文字列型より、用途を絞った型を露出すべき

文書変換ライブラリで使いたい気持ちがあったが、どうも文字列と交換可能だったり相互運用性の高い型を露出しようというのがあまり賢くない気がしてきた。

```rust
// 最初考えていたこと

pub struct SomeNode {
    id: Option<SmallString>,
    class: Option<Vec<SmallString>>,
}

impl SomeNode {
    pub fn id(&self) -> Option<&str> { .. }
    pub fn set_id(&mut self, s: impl Into<SmallString>) { .. }
    // これ要るか？
    pub fn id_mut(&mut self) -> Option<&mut SmallString> { .. }
    // ..
}
```

いちライブラリの都合で、あらゆる場所で共有できると限らない文字列を使うのはあまり素敵ではない。
まあそもそも getter と setter と `&mut` 返すやつ全部用意してる時点でカプセル化が全くできてないんだけど。

```rust
// もうちょっと正解に近そうな感じのやつ

pub struct SomeNode {
    id: Option<SmallString>,
    class: Option<Vec<SmallString>>,
    // ..
}

impl SomeNode {
    pub fn id(&self) -> Option<&str> { .. }
    // 引数 `id` の型については検討の余地がある
    pub fn set_id(&mut self, id: impl AsRef<str> + Into<String>) { .. }
    // `&mut` は渡さないので、効率的に弄りたいなら所有権ごと奪うか `&str` を使ってね
    pub fn take_id(&mut self) -> Option<String> { .. }
    // ..
}
```

こっちの方が文字列の内部実装を隠蔽できるし正解に近そう。

ちゃんと設計考えような……
