use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;
use std::time::Instant;
use tic_tac_toe_stencil::board::Cell;


// Your solution solution.
fn heuristic(board: &Board, player: Player) -> i32 {
    let base = board.score() * 50;
    let extra = count_twos(board);
    let total = base + extra;
    
    match player {
        Player::X => total,
        Player::O => -total,
    }
}

fn count_twos(board: &Board) -> i32 {
    let cells = board.get_cells();
    let n = cells.len();
    let mut value = 0;


    let mut eval = |a: &Cell, b: &Cell, c: &Cell| {
        let mut xs = 0;
        let mut os = 0;
        let mut empties = 0;

        for cell in [a, b, c] {
            match cell {
                Cell::X => xs += 1,
                Cell::O => os += 1,
                Cell::Empty => empties += 1,
                Cell::Wall => return,
            }
        }

        if xs == 2 && empties == 1 {
            value += 10;
        }
        if os == 2 && empties == 1 {
            value -= 10;
        }
        if xs == 1 && empties == 2 {
            value += 2;
        }
        if os == 1 && empties == 2 {
            value -= 2;
        }
    };

    for i in 0..n {
        for j in 0..(n-2) {
            eval(&cells[i][j], &cells[i][j + 1], &cells[i][j + 2]);
        }
    }

    for j in 0..n {
        for i in 0..(n-2) {
            eval(&cells[i][j], &cells[i + 1][j], &cells[i + 2][j]);
        }
    }

    for i in 0..(n-2) {
        for j in 0..(n-2) {
            eval(&cells[i][j], &cells[i + 1][j + 1], &cells[i + 2][j + 2]);
        }
    }

    for i in 0..(n-2) {
        for j in 2..n {
            eval(&cells[i][j], &cells[i + 1][j - 1], &cells[i + 2][j - 2]);
        }
    }

    value
}

fn minimax(board: &mut Board, player: Player, depth: u32, max_depth: u32, start: Instant, _time_limit: u64, maximizing_player: Player,) -> i32 {
    if _time_limit > 0 && start.elapsed().as_millis() as u64 >= _time_limit {
        return heuristic(board, maximizing_player);
    }
    if board.game_over() {
        let score = board.score();
        return if maximizing_player == Player::X { score } else { -score };
    }
    if depth == max_depth {
        return heuristic(board, maximizing_player);
    }

    let mut moves = board.moves();
    let size = board.get_cells().len();

    moves.sort_by_key(|&(x, y)| {
        let center = (size / 2) as i32;
        let dx = x as i32 - center;
        let dy = y as i32 - center;
        dx * dx + dy * dy
    });
    
    let maximizing_turn = player == maximizing_player; 

    let mut best_score = if maximizing_turn { i32::MIN } else { i32::MAX }; 

    for m in moves {
        if _time_limit > 0 && start.elapsed().as_millis() as u64 >= _time_limit {
            break;
        }

        let mut new_board = board.clone();
        new_board.apply_move(m, player);
        let next_player = player.flip();

        let score = minimax(
            &mut new_board,
            next_player,
            depth + 1,
            max_depth,
            start,
            _time_limit,
            maximizing_player,
        );

        if maximizing_turn { 
            best_score = best_score.max(score); 
        } 
        else {
            best_score = best_score.min(score); 
        }
    }
    best_score
}
pub struct SolutionAgent {}

impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        let start = Instant::now();
        let size = board.get_cells().len();
        let max_depth = if size == 3 { 6 } else { 4 };
    
        let mut moves = board.moves();
        moves.sort_by_key(|&(x, y)| {
            let center = (size / 2) as i32;
            let dx = x as i32 - center;
            let dy = y as i32 - center;
            dx * dx + dy * dy
        });

        let mut best_score = i32::MIN; 
        let mut best_move = moves[0];

        for m in moves {
            let mut new_board = board.clone();
            new_board.apply_move(m, player);
            let score = minimax(
                &mut new_board,
                player.flip(),
                1,
                max_depth,
                start,
                _time_limit,
                player,
            );
        
            if score > best_score { 
                best_score = score; 
                best_move = m; 
            }
        }
        (best_score, best_move.0, best_move.1)
    }
}