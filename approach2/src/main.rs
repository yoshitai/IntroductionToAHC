use proconio::{input};
use rand::Rng;

const TYPE: usize = 26;

pub fn get_time() -> f64 {
	static mut STIME: f64 = -1.0;
	let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
	let ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
	unsafe {
		if STIME < 0.0 {
			STIME = ms;
		}
		ms - STIME
	}
}
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
    const TL: f64 = 1.9; // 制約時間
    let mut rng = rand::thread_rng();
    let mut out = (0..input.date).map(|_| rng.gen_range(0..26)).collect::<Vec<_>>();
    let mut score = compute_score(&input, &out);

    while get_time() < TL {
        let d = rng.gen_range(0..input.date);
        let q = rng.gen_range(0..26);
        let old = out[d];
        out[d] = q;
        let new_score = compute_score(&input, &out);
        if score > new_score {
            out[d] = old;
        } else {
            score = new_score;
        }
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
