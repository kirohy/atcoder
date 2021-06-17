use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
        l: i32,
    }
    let sum = (n - 1) * n / 2 + n * l;
    if l >= 0 {
        println!("{}", sum - l);
    } else if n + l <= 0 {
        println!("{}", sum - n - l + 1);
    } else {
        println!("{}", sum);
    }
}
