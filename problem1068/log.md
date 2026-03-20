以下のコードだと

```rust
use std::io;
fn main() -> anyhow::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    let mut number: i64 = input.trim().parse()?;
    while number != 0 {
        if number % 2 == 0 {
            number /= 2;
        } else {
            number = number * 3 + 1;
        }
        number -= 1;
    }
    Ok(())
}

```

overflowします。なぜ？

````shell
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/problem1068`
3

thread 'main' (117478) panicked at problem1068/src/main.rs:12:22:
attempt to multiply with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

[Process exited 101]
````

# オーバーフロー原因の分析

**結論**: 奇数時に `number * 3 + 1` が `i64` の最大値を超える。これは問題ロジックの誤りではなく、**データ型選択の問題**。

---

## 何が起きているか

**ケース: `n = 3`**

```
3 → (3*3+1=10) → 5 → (5*3+1=16) → 8 → 4 → 2 → 1
```

途中で値が大きくなるが、Collatzは理論的に最終的に1に収束する（未証明だが実践では成り立つ）。

**問題のコード**

```rust
number = number * 3 + 1;  // ← ここでオーバーフロー発生
```

`i64` の最大値は `2^63 - 1 ≈ 9.2 × 10^18`。
十分大きな奇数に対して `*3+1` はこれを超える。

---

## 別問題: ロジックエラー

**重大なバグを発見**

```rust
while number != 0 {
    // ...
    number -= 1;  // ← なぜ毎回1を引く？
}
```

この `number -= 1` は**問題仕様にない**。
Collatzアルゴリズムは単に `3n+1` または `n/2` のみ。

正しい終了条件は `number != 1` のはず。

---

## 解決策（代替案比較）

| 案 | 型 | メリット | デメリット |
|---|---|---------|----------|
| **u64** | 符号なし64ビット | 正の整数に適切、範囲広い | 負数は扱えない |
| **i128** | 符号付き128ビット | 完全に安全 | メモリ2倍、やや重い |
| チェック付き演算 | i64 + 方法 | メモリ節約 | 実装複雑 |

**推奨**: 制約 `n ≤ 10^6` でも途中で膨らむため、**`u64` で十分かつ適切**。

---

## 修正ポイント

1. `i64` → `u64` に変更
2. `number -= 1` を **削除**
3. 終了条件を `number == 1` に確認

コード構造は正しい。バグはロジックと型選択にある。

---

**質問**: Level 1の方向性に戻って、ロジックを一度確認してみないか？
