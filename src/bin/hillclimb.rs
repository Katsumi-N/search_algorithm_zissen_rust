use rand::prelude::*;
use std::cmp::Ordering;

const H: usize = 30;
const W: usize = 30;
const CHARACTER_N: usize = 3;
const END_TURN: usize = 100;
const DX: [isize; 4] = [1, -1, 0, 0];
const DY: [isize; 4] = [0, 0, 1, -1];

// 座標を保持する
#[derive(Clone,Copy)]
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
#[derive(Clone,Copy)]
struct AutoMoveMazeState {
    points: [[usize; W]; H], // 床のポイントを1~9で表現する
    turn: usize,             // 現在のターン
    characters: [Coord; CHARACTER_N],
    game_score: usize, // ゲーム上で実際に得たスコア
    evaluated_score: usize,
}

impl Ord for AutoMoveMazeState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.evaluated_score.cmp(&other.evaluated_score)
    }
}

impl PartialOrd for AutoMoveMazeState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for AutoMoveMazeState {}

impl PartialEq for AutoMoveMazeState {
    fn eq(&self, other: &Self) -> bool {
        self.evaluated_score == other.evaluated_score
    }
}

impl AutoMoveMazeState {
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
            characters: [Coord {x: 0, y: 0}; CHARACTER_N],
            game_score: 0,
            evaluated_score: 0,
        }
    }

    fn move_player(&mut self, character_id: usize) {
        let character = &mut self.characters[character_id];
        let mut best_point: isize = -100000000;
        let mut best_action_index: usize = 0;
        for action in 0..4 {
            let ty = character.y + DY[action];
            let tx = character.x + DX[action];
            if ty >= 0 && ty < H as isize && tx >= 0 && tx < W as isize {
                let point = self.points[ty as usize][tx as usize];
                if point as isize > best_point {
                    best_point = point as isize;
                    best_action_index = action;
                }
            }
        }

        character.y += DY[best_action_index];
        character.x += DX[best_action_index];

    }

    // ゲームの終了判定
    fn is_done(&self) -> bool {
        self.turn == END_TURN
    }

    // 指定したactionでゲームを1ターン進める
    fn advance(&mut self) {
        for character_id in 0..CHARACTER_N {
            self.move_player(character_id);
        }
        for character in &mut self.characters {
            let point = &mut self.points[character.y as usize][character.x as usize];
            self.game_score += *point;
            *point = 0;
        }
        self.turn += 1;
    }

    fn get_score(&self, is_print: bool) -> usize {
        let mut tmp_state = self.clone();
        for character in &mut tmp_state.characters {
            let point = &mut tmp_state.points[character.y as usize][character.x as usize];
            *point = 0;
        }
        while !tmp_state.is_done() {
            tmp_state.advance();
            if is_print {
                println!("{}", tmp_state.to_string());
            }
        }
        tmp_state.game_score
    }

    fn to_string(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("turn:\t{}\n", self.turn));

        output.push_str(&format!("score:\t{}\n", self.game_score));
        let mut board_chars = vec![vec!['.'; W]; H];
        for h in 0..H {
            for w in 0..W {
                let mut is_written = false;
                for character in &self.characters {
                    if character.y == h as isize && character.x == w as isize {
                        is_written = true;
                        break;
                    }
                    board_chars[character.y as usize][character.x as usize] = '@';
                }
                if !is_written {
                    if self.points[h][w] > 0 {
                        output.push_str(&self.points[h][w].to_string());
                    } else {
                        output.push('.');
                    }
                }
            }
            output.push('\n');
        }
        output
    }

    // ランダムに１匹を遷移させる
    fn transition(&mut self) {
        let mut rng = rand::thread_rng();
        let mut character = &mut self.characters[rng.gen_range(0..CHARACTER_N)];
        character.x = rng.gen_range(0..100000) % W as isize;
        character.y = rng.gen_range(0..100000) % H as isize;
    }

    fn init(&mut self) {
        let mut rng = rand::thread_rng();
        for character in &mut self.characters {
            character.y = rng.gen_range(0..100000) % H as isize;
            character.x = rng.gen_range(0..100000) % W as isize;
        }
    }
}

type State = AutoMoveMazeState;
type AIFunction = Box<dyn Fn(&State) -> State>;

struct StringAIPair {
    name: String,
    function: AIFunction,
}

fn hill_climb(state: &State, number: usize) -> State {
    let mut now_state = state.clone();
    now_state.init();
    let mut best_score = now_state.get_score(false);
    for _ in 0..number {
        let mut next_state = now_state.clone();
        next_state.transition();
        let next_score = next_state.get_score(false);
        if next_score > best_score {
            best_score = next_score;
            now_state = next_state;
        }
    }
    now_state
}

fn play_game(ai: &StringAIPair, seed: i32) {
    let mut state = State::new(seed as u8);
    state = (ai.function)(&state);
    println!("{}", state.to_string());
    let score = state.get_score(true);
    println!("Score of {}: {}", ai.name, score);
}

fn main() {
    let ai = StringAIPair {
        name: "randomAction".to_string(),
        function: Box::new(|state| hill_climb(state, 100_000)),
    };
    play_game(&ai, 0);
}