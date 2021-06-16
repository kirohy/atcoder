use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
        x: i32,
        l: [i32; n]
    }
    let mut ans = 1;
    let mut total = 0;
    for v in l.iter() {
        total += v;
        if total > x {
            break;
        }
        ans += 1;
    }
    println!("{}", ans);
}
