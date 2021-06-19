use proconio::{fastout, input};

fn dfs(node: i32, graph: &Vec<Vec<i32>>, seen: &mut Vec<bool>) {
    seen[node as usize] = true;
    for i in graph[node as usize].iter() {
        if !seen[*i as usize] {
            dfs(*i, graph, seen);
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let max_n = 200010;

    let mut seen = vec![true; max_n];
    let mut graph: Vec<Vec<i32>> = vec![vec![]; max_n];
    let mut ans = 0;

    for i in a.iter() {
        if seen[*i as usize] {
            seen[*i as usize] = false;
            ans += 1;
        }
    }

    for i in 0..n / 2 {
        graph[a[i] as usize].push(a[n - i - 1]);
        graph[a[n - i - 1] as usize].push(a[i]);
    }

    for i in 0..max_n {
        if !seen[i] {
            dfs(i as i32, &graph, &mut seen);
            ans -= 1;
        }
    }

    println!("{}", ans);
}
