以下のようなコードを書いたけど上手くいかない

````rust
use std::io::{self, Read};

// n = 5
// 完全な合計 S = 1 + 2 + 3 + 4 + 5 = 15
// 読んだ数: 2, 3, 1, 5
// 読んだ合計 R = 2 + 3 + 1 + 5 = 11
// 欠けた数 = 15 - 11 = 4 ✓
fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();
    //第1行: nを読みこむ
    let mut n: u32 = lines.next().unwrap().trim().parse().unwrap();

    //第2行： n-1を読みこむ
    let number: Vec<u32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut complete_result: u32 = 0;
    while n == 0 {
        complete_result += 1;
        n -= 1
    }
    let mut read_result: u32 = 0;
    for element in number {
        read_result += element;
    }
    let result = complete_result - read_result;

    println!("{}", result);
    Ok(())
}
````

**結論：** ロジックエラーが2つある。`while n == 0` の条件と、`u32` のオーバーフロー対策が必要。

---

## 問題点の指摘

**問題1：ループ条件が逆**

```rust
while n == 0 {  // ❌ n が 0 になったら回る
    complete_result += 1;
    n -= 1
}
```

`n = 5` で始まるのに、`n == 0` は最初から `false` なので、ループが **1回も実行されない**。

完全な合計を計算するなら：

- `n != 0` の間回す、または
- `n > 0` の間回す

**問題2：型が小さすぎる**
`n ≤ 2 × 10^5` のとき、完全合計は最大：

```
1 + 2 + ... + 200000 ≈ 2 × 10^10
```

`u32` の上限は `4 × 10^9` なので **オーバーフロー** する。`u64` を使う必要がある。

---

## 修正のヒント（Level 2.5）

**修正すべき部分：**

1. **ループ条件を直す**

   ```
   while n > 0 {  // または n != 0
       complete_result += n;
       n -= 1;
   }
   ```

   または、もっと数学的に：

   ```
   完全合計 = n * (n + 1) / 2
   ```

   （1 から n までの合計の公式）

2. **型を `u64` に統一**

   ```rust
   let mut n: u64 = ...
   let number: Vec<u64> = ...
   let mut complete_result: u64 = 0;
   ```

---

## 代替案

**案1（現在のコード改善）：** ループで累積

- メリット：直感的
- デメリット：ループのオーバーヘッド

**案2（推奨）：** 数学公式を使う

```
complete_sum = n * (n + 1) / 2
```

- メリット：O(1)、読んだ数の合計計算だけで済む
- デメリット：公式を覚える必要

どちらで進める？修正してから `next` か `stuck` を言ってくれ。

参考：

- [Rust 整数型の範囲](https://doc.rust-lang.org/std/primitive.u32.html)
- [Gauss の合計公式](https://ja.wikipedia.org/wiki/%E5%92%8C%E3%81%AE%E5%85%AC%E5%BC%8F)
