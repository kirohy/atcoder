use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    for i in 0..3 {
        if s[i] == s[i + 1] {
            println!("Bad");
            return;
        }
    }
    println!("Good");
}
