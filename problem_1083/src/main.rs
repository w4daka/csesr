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
    let mut n: u64 = lines.next().unwrap().trim().parse().unwrap();

    //第2行： n-1を読みこむ
    let number: Vec<u64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut complete_result: u64 = 0;
    while n > 0 {
        complete_result += n;
        n -= 1
    }
    let mut read_result: u64 = 0;
    for element in number {
        read_result += element;
    }
    let result = complete_result - read_result;

    println!("{}", result);
    Ok(())
}
