fn main() {
    let mut x = 5;
    let y = loop {
        x = x + 1;
        if x > 10 {
            break x * 2;
        }
    };
    println!("el numero es {}", add_one(y));
}

fn add_one(x: i64) -> i64 {
    return x + 1;
}
