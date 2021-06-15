use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }
    let n_max = 100005;
    let modulo = 1000000007;

    let mut stairs: Vec<bool> = vec![true; n_max];
    for v in a.into_iter() {
        stairs[v] = false;
    }

    let mut dp: Vec<i64> = vec![0; n_max];
    dp[0] = 1;
    for i in 0..n {
        for j in 1..3 {
            if stairs[i + j] {
                dp[i + j] += dp[i];
                dp[i + j] %= modulo;
            }
        }
    }

    println!("{}", dp[n]);
}
