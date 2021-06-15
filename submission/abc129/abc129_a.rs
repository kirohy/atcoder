use proconio::input;

fn main() {
    input! {
        mut dist: [i32; 3]
    }
    dist.sort();
    println!("{}", dist[0] + dist[1]);
}
