use rand::prelude::*;
use std::collections::BinaryHeap;

const H: usize = 30;
const W: usize = 30;
const END_TURN: usize = 100;
const DX: [isize; 4] = [1, -1, 0, 0];
const DY: [isize; 4] = [0, 0, 1, -1];
const SCORE_TYPE: usize = 100000000;
// 座標を保持する
#[derive(Clone, Copy, Debug, Ord, Eq, PartialOrd, PartialEq)]
struct Coord {
    y: isize,
    x: isize,
}

impl Coord {
    fn new(y: isize, x: isize) -> Self {
        Self { y, x }
    }
}

// 一人ゲームの例
// 1ターンに上下左右四方向のいずれかに1マスずつ進む。
// 床にあるポイントを踏むと自身のスコアとなり、床のポイントが消える。
// END_TURNの時点のスコアを高くすることが目的
#[derive(Clone, Copy, Debug, Ord, Eq, PartialOrd, PartialEq)]
struct MazeState {
    points: [[usize; W]; H], // 床のポイントを1~9で表現する
    turn: usize,             // 現在のターン
    character: Coord,
    game_score: usize, // ゲーム上で実際に得たスコア
    evaluated_score: usize,
    first_action: isize,
}

impl MazeState {
    fn new(seed: u8) -> Self {
        let mut rng: rand::rngs::StdRng = rand::SeedableRng::from_seed([seed; 32]);
        let character = Coord::new(rng.gen_range(0..H) as isize, rng.gen_range(0..W) as isize);
        let mut points = [[0; W]; H];
        for y in 0..H {
            for x in 0..W {
                if y as isize == character.y && x as isize == character.x {
                    continue;
                }
                points[y][x] = rng.gen_range(0..10) as usize;
            }
        }
        Self {
            points,
            turn: 0,
            character,
            game_score: 0,
            evaluated_score: 0,
            first_action: -1,
        }
    }

    // ゲームの終了判定
    fn is_done(&self) -> bool {
        self.turn == END_TURN
    }

    // 指定したactionでゲームを1ターン進める
    fn advance(&mut self, action: usize) {
        self.character.x += DX[action];
        self.character.y += DY[action];
        let point = &mut self.points[self.character.y as usize][self.character.x as usize];
        if *point > 0 {
            self.game_score += *point;
            *point = 0;
        }
        self.turn += 1;
    }

    // 現在の状況でプレイヤーが可能な行動を全て取得する
    fn legal_actions(&self) -> Vec<usize> {
        let mut actions = Vec::new();
        for action in 0..4 {
            let ty = self.character.y + DY[action];
            let tx = self.character.x + DX[action];
            if ty >= 0 && ty < H as isize && tx >= 0 && tx < W as isize {
                actions.push(action);
            }
        }
        actions
    }

    // 現在のゲーム状況を文字列にする
    fn to_string(&self) -> String {
        let mut ss = String::from("");

        for h in 0..H {
            for w in 0..W {
                if self.character.y == h as isize && self.character.x == w as isize {
                    ss += "@";
                } else if self.points[h][w] > 0 {
                    ss += &self.points[h][w].to_string();
                } else {
                    ss += ".";
                }
            }
            ss += "\n";
        }
        ss += &format!("turn: {} score: {}", self.turn.to_string(), self.game_score.to_string());
        ss
    }

    fn evaluate_score(&mut self) {
        self.evaluated_score = self.game_score;
    }
}

fn opeartor(maze1: &mut MazeState, maze2: &mut MazeState) -> bool {
    maze1.evaluate_score() < maze2.evaluate_score()
}

fn beam_search_action(state: &MazeState, beam_width: usize, beam_depth: usize) -> usize {
    let mut now_beam: BinaryHeap<MazeState> = BinaryHeap::new();
    now_beam.push(*state);

    let mut best_state = state.clone();

    for t in 0..beam_depth {
        let mut next_beam: BinaryHeap<MazeState> = BinaryHeap::new();
        for _ in 0..beam_width {
            if now_beam.is_empty() {
                break;
            }
            let now_state = now_beam.pop().unwrap();
            let legal_actions = now_state.legal_actions();
            for action in legal_actions {
                let mut next_state = now_state.clone();
                next_state.advance(action);
                next_state.evaluate_score();
                if t == 0 {
                    next_state.first_action = action as isize;
                }
                next_beam.push(next_state);
            }
        }

        now_beam = next_beam;
        best_state = now_beam.pop().unwrap();

        if best_state.is_done() {
            break;
        }
    }
    best_state.first_action as usize
}


fn greedy_action(state: &MazeState) -> usize {
    let legal_actions = state.legal_actions();
    let mut best_score: isize = -1;
    let mut best_action: isize = -1;

    for action in legal_actions {
        let mut now_state = state.clone();
        now_state.advance(action);
        now_state.evaluate_score();
        if now_state.evaluated_score as isize > best_score {
            best_score = now_state.evaluated_score as isize;
            best_action = action as isize;
        }
    }

    best_action as usize
}

fn play_game(seed: u8, scores: &mut Vec<usize>) {
    let mut state = MazeState::new(seed);
    println!("{}", state.to_string());
    while !state.is_done() {
        state.advance(beam_search_action(&state, 5, 2)); // ビームサーチ
        // state.advance(greedy_action(&state));
        println!("{}", state.to_string());
    }
    scores.push(state.game_score);
}

fn calc_average(score: &Vec<usize>) -> usize {
    let sum: usize = score.iter().sum();
    let average = sum / score.len();
    average
}
fn main() {
    let mut rng = rand::thread_rng();
    let mut scores = vec![0 as usize; 100];

    for _ in 0..100 {
        let seed = rng.gen_range(0..100) as u8;
        play_game(seed, &mut scores);
    }

    println!("average score: {}", calc_average(&scores))
}
