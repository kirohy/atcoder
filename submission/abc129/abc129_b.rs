use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut w: [i32; n],
    }
    for i in 1..n {
        w[i] += w[i - 1];
    }
    let mut dist: Vec<i32> = Vec::new();
    for i in 0..n {
        dist.push((w[n - 1] - 2 * w[i]).abs());
    }
    println!("{}", dist.iter().min().unwrap());
}
