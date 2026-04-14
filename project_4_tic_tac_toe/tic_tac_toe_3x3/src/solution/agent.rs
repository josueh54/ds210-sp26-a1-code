use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
pub struct SolutionAgent {}

// Put your solution here.
impl Agent for SolutionAgent 
{
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.
fn solve(board: &mut Board, player: Player) -> (i32, usize, usize) {
        // If you want to make a recursive call to this solution, use
        // `SolutionAgent::solve(...)`
        if board.game_over() {
            return (board.score(), 0, 0);
        }

        let moves = board.moves();
        let mut best_score = match player {
            Player::X => i32::MIN,
            Player::O => i32::MAX,
        };
        
        let mut best_move = moves[0];
        for m in moves {
            let mut new_board = board.clone();
            new_board.apply_move(m, player);
            
            let (score, _, _) = SolutionAgent::solve(&mut new_board, player.flip());

            match player {
                Player::X => {
                    if score > best_score {
                        best_score = score;
                        best_move = m;
                    }
                }
                Player::O => {
                    if score < best_score {
                        best_score = score;
                        best_move = m;
                    }
                }
            }
        }
        (best_score, best_move.0, best_move.1)
    }


        // If you want to make a recursive call to this solution, use
        // `SolutionAgent::solve(...)`
}

