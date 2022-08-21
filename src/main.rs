use cargo_snippet::snippet;

#[snippet]
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let _x = gcd(12, 18);
    println!("Hello, world!");
}
