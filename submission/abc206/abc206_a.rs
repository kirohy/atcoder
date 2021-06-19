use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
    }
    let price = (n as f32 * 1.08).floor() as i32;
    if price < 206 {
        println!("Yay!");
    } else if price > 206 {
        println!(":(");
    } else {
        println!("so-so");
    }
}
