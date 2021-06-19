use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut ans = n * (n - 1) / 2;
    let mut hashmap = HashMap::new();
    for x in a {
        let counter = hashmap.entry(x).or_insert(0);
        *counter += 1;
    }
    let mut hash_vec: Vec<_> = hashmap.into_iter().collect();
    hash_vec.sort_by(|x, y| y.1.cmp(&x.1));
    for i in hash_vec.iter() {
        if i.1 <= 1 {
            break;
        }
        ans -= i.1 * (i.1 - 1) / 2;
    }
    println!("{}", ans);
}
