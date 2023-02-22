use proconio::{input};
use rand::Rng;

const TYPE: usize = 26;

pub trait SetMinMax {
	fn setmin(&mut self, v: Self) -> bool;
	fn setmax(&mut self, v: Self) -> bool;
}

impl<T> SetMinMax for T where T: PartialOrd {
	fn setmin(&mut self, v: T) -> bool {
		*self > v && { *self = v; true }
	}
	fn setmax(&mut self, v: T) -> bool {
		*self < v && { *self = v; true }
	}
}

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

fn cost(a: usize, b: usize) -> i64 {
    let d = b - a;
    (d * (d - 1) / 2) as i64
}

struct State {
    out: Vec<usize>,
    score: i64,
    ds: Vec<Vec<usize>>, // コンテストタイプごとの開催日
}

impl State {
    fn new(input: &Input, out: Vec<usize>) -> State {
        let mut ds = vec![vec![]; 26];
        for d in 0..input.date {
            ds[out[d]].push(d + 1);
        }
        let score = compute_score(&input, &out);
        State { out, score, ds }
    }

    fn change(&mut self, input:&Input, d: usize, new_i: usize) {
        let old_i = self.out[d];
        let p = self.ds[old_i].iter().position(|a| *a == d + 1).unwrap();
        let prev = self.ds[old_i].get(p - 1).cloned().unwrap_or(0);
        let next = self.ds[old_i].get(p + 1).cloned().unwrap_or(input.date + 1);
        self.ds[old_i].remove(p);
        self.score += (cost(prev, d + 1) + cost(d + 1, next) - cost(prev, next)) * input.c[old_i];
        let p = self.ds[new_i].iter().position(|a| *a > d + 1).unwrap_or(self.ds[new_i].len());
        let prev = self.ds[new_i].get(p - 1).cloned().unwrap_or(0);
        let next = self.ds[new_i].get(p).cloned().unwrap_or(input.date + 1);
        self.ds[new_i].insert(p, d + 1);
        self.score -= (cost(prev, d + 1) + cost(d + 1, next) - cost(prev, next)) * input.c[new_i];
        self.score += input.s[d][new_i] - input.s[d][old_i];
        self.out[d] = new_i;
    }
}

fn solve(input: &Input) -> Vec<usize> {
    const T0: f64 = 2e3;
    const T1: f64 = 6e2;
    const TL: f64 = 1.9; // 制約時間
    let mut rng = rand::thread_rng();
    let mut state = State::new(input, (0..input.date).map(|_| rng.gen_range(0..26)).collect());
    let mut cnt = 0;
    let mut T = T0;
    let mut best = state.score;
    let mut best_out = state.out.clone();

    loop {
        cnt += 1;
        if cnt % 100 == 0 {
            let t = get_time() / TL; // 経過時間を正規化
            if t >= 1.0 {
                break;
            }
            T = T0.powf(1.0 - t) * T1.powf(t);
        }
        let old_score = state.score;
        if rng.gen_bool(0.5) {
            let d = rng.gen_range(0..input.date);
            let old = state.out[d];
            state.change(input, d, rng.gen_range(0..26));
            if old_score > state.score && !rng.gen_bool(f64::exp((state.score - old_score) as f64 / T)) {
                state.change(input, d, old);
            }
        } else {
            let d1 = rng.gen_range(0..input.date - 1);
            let d2 = rng.gen_range((d1 + 1)..(d1 + 16).min(input.date));
            let (a, b) = (state.out[d1], state.out[d2]);
            state.change(input, d1, b);
            state.change(input, d2, a);
            if old_score > state.score && !rng.gen_bool(f64::exp((state.score - old_score) as f64 / T)) {
                state.change(input, d1, a);
                state.change(input, d2, b);
            }
        }
        if best.setmax(state.score) {
            best_out = state.out.clone();
        }
    }
    best_out
}

fn main() {
    let input = read_input();
    let out = solve(&input);

    for i in 0..input.date {
        println!("{}", out[i] + 1);
    }
}
