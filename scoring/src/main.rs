use proconio::{input, marker::Usize1};

const TYPE: usize = 26;

struct Input {
    D: usize,
    s: Vec<Vec<i64>>,
    c: Vec<i64>,
    t: Vec<usize>,
}

fn read_input() -> Input {
    input! {
        D: usize,
        c: [i64; TYPE],
        s: [[i64; TYPE]; D],
        t: [Usize1; D]
    }
    Input { D, s, c, t }
}

fn main() {

    let input = read_input();
    let mut score = 0;
    let mut last = vec![0; TYPE];
    for d in 0..input.D {
        score += input.s[d][input.t[d]];
        last[input.t[d]] = d + 1;
        for i in 0..TYPE {
            score -= (d + 1 - last[i]) as i64 * input.c[i];
        }

        println!("{}", score);
    }

}
