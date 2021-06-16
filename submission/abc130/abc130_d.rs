use proconio::{fastout, input};

fn get_total(buf: &Vec<i64>, first: usize, last: usize) -> i64 {
    if first == 0 {
        return buf[last];
    }
    buf[last] - buf[first - 1]
}

#[fastout]
fn main() {
    input! {
        n: usize,
        k: i64,
        mut a: [i64; n]
    }
    let mut ans: i64 = 0;
    let mut total = vec![a[0]; 1];
    for i in 1..n {
        total.push(total.last().unwrap() + a[i]);
    }
    let mut last_index = 0;
    for i in 0..n as usize {
        for j in last_index..n as usize {
            if get_total(&total, i, j) >= k {
                last_index = j;
                ans += (n - j) as i64;
                break;
            }
        }
    }
    println!("{}", ans);
}
