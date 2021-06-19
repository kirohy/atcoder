use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
    }
    let mut index = 1;
    let mut sum = 0;
    loop {
        sum += index;
        if sum >= n {
            break;
        }
        index += 1;
    }
    println!("{}", index);
}
