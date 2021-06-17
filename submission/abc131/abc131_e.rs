use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
        k: i32,
    }
    let mut k_tmp = (n - 1) * (n - 2) / 2;
    let mut ans: Vec<(i32, i32)> = Vec::new();
    for i in 2..n + 1 {
        ans.push((1, i));
    }

    if k > k_tmp {
        println!("-1");
    } else {
        for i in 2..n + 1 {
            for j in i + 1..n + 1 {
                if k == k_tmp {
                    break;
                }
                ans.push((i, j));
                k_tmp -= 1;
            }
        }
        println!("{}", ans.len());
        for i in ans.iter() {
            println!("{} {}", i.0, i.1);
        }
    }
}
