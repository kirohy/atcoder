use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut horizontal = vec![vec![-1; 1]; h];
    let mut vertical = vec![vec![-1; 1]; w];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                horizontal[i].push(j as i32);
                vertical[j].push(i as i32);
            }
        }
    }
    for v in horizontal.iter_mut() {
        v.push(w as i32);
    }
    for v in vertical.iter_mut() {
        v.push(h as i32);
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                let search_h = j as i32;
                let search_w = i as i32;
                let h_index = horizontal[i]
                    .binary_search_by(|x| x.cmp(&search_h))
                    .unwrap_err();
                let w_index = vertical[j]
                    .binary_search_by(|x| x.cmp(&search_w))
                    .unwrap_err();
                let light = (horizontal[i][h_index] - horizontal[i][h_index - 1] - 1)
                    + (vertical[j][w_index] - vertical[j][w_index - 1] - 1)
                    - 1;
                if ans < light {
                    ans = light;
                }
            }
        }
    }

    println!("{}", ans);
}
