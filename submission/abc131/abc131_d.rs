use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
        mut works: [(i32,i32) ;n]
    }
    works.sort_by_key(|x| x.1);
    let mut total = 0;
    for i in works.iter() {
        total += i.0;
        if total > i.1 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
