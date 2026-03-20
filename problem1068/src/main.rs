use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let mut number: u64 = input.trim().parse().unwrap();
    while number != 1 {
        println!("{}", number);

        if number % 2 == 0 {
            number /= 2;
        } else {
            number = number * 3 + 1;
        }
    }
    println!("1");
}
