use std::fmt::format;

use rand::prelude::*;

const H: usize = 10;
const W: usize = 10;
const END_TURN: usize = 10;
const DX: [isize; 4] = [1, -1, 0, 0];
const DY: [isize; 4] = [0, 0, 1, -1];

// 座標を保持する
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
struct MazeState {
    points: [[usize; W]; H], // 床のポイントを1~9で表現する
    turn: usize,             // 現在のターン
    character: Coord,
    game_score: usize, // ゲーム上で実際に得たスコア
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
}

fn random_action(state: &MazeState) -> usize {
    let legal_actions = state.legal_actions();
    println!("legal: {:?}", legal_actions);
    legal_actions[0]
}

fn play_game(seed: u8) {
    let mut state = MazeState::new(seed);
    println!("{}", state.to_string());
    while !state.is_done() {
        state.advance(random_action(&state));
        println!("{}", state.to_string());
    }

}
fn main() {
    play_game(11);
}