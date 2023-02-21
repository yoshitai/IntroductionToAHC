use proconio::{input};

const TYPE: usize = 26;

struct Input {
    date: usize,
    s: Vec<Vec<i64>>,
    c: Vec<i64>,
}

fn read_input() -> Input {
    input! {
        date: usize,
        c: [i64; TYPE],
        s: [[i64; TYPE]; date],
    }
    Input { date, s, c }
}

fn compute_score(input: &Input, out: &Vec<usize>) -> i64 {
    let mut score = 0;
    let mut last = vec![0; TYPE];
    for d in 0..out.len() {
        last[out[d]] = d + 1;
        for i in 0..26 {
            score -= (d + 1 - last[i]) as i64 * input.c[i];
        }
        score += input.s[d][out[d]];
    }
    score
}

fn solve(input: &Input) -> Vec<usize> {
    let mut out = vec![];
    for _ in 0..input.date {
        let mut max_score = std::i64::MIN;
        let mut best_i = 0;
        for i in 0..TYPE {
            out.push(i);
            let score = compute_score(&input, &out);
            if max_score < score {
                max_score = score;
                best_i = i;
            }
            out.pop();
        }
        out.push(best_i);
    }
    out
}

fn main() {

    let input = read_input();
    let out = solve(&input);

    for i in 0..input.date {
        println!("{}", out[i] + 1);
    }
}
