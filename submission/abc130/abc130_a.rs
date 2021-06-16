use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i32,
        x: i32,
    }
    if a < x {
        println!("0");
    } else {
        println!("10");
    }
}
