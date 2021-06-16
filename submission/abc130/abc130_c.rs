use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        w: i32,
        h: i32,
        x: i32,
        y: i32
    }
    print!("{}", (w as f32) * (h as f32) / 2.0);
    if w == x * 2 && h == y * 2 {
        print!(" 1\n");
    } else {
        print!(" 0\n");
    }
}
