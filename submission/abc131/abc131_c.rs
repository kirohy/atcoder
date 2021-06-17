use num::Integer;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    }
    let div_c = b / c - (a - 1) / c;
    let div_d = b / d - (a - 1) / d;
    let div_cd = b / c.lcm(&d) - (a - 1) / c.lcm(&d);
    println!("{}", b - a + 1 - div_c - div_d + div_cd);
}
