use std::fmt;

// #[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver,
}

pub enum Colour {
    White,
    Black,
}


/* IMPORTANT:
 * - Document well!
 * - Write well structured and clean code!
 */

// pub struct Piece{
//     white: bool,
//     killed: bool,
//     x_pos: i8,
//     y_pos: i8
// }

// impl Piece{
//     pub fn new(is_white: bool) -> Piece{
//         let white: bool = false;
//         if white{
//             white = true;
//         }

//         Piece{
//             white: is_white,
            
//         }
//     }

//     pub fn Pawn(){

//     }
// }



// pub struct Board{
//     board_layout: [[String; 8]; 8],
// }

// impl Board{
    
// }

pub struct Game{
    /* save board, active colour, ... */
    state: GameState,
    black: u64,
    white: u64,
    kings: u64,
    game_board: [[String;8];8]
}

impl Game{
    /// Initialises a new board with pieces.
    pub fn new() -> Game{
        let pawn: String = String::from("P");
        let rook: String = String::from("R");
        let knight: String = String::from("Kn");
        let bishop: String = String::from("Bi");
        let queen: String = String::from("Q");
        let king: String = String::from("K");
        let white: String = String::from("w_");
        let black: String = String::from("b_");
        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            black: 1,
            white: 1,
            kings: 1,
            game_board: [
                [format!("{}{}", white, rook), 
                 format!("{}{}", white, knight), 
                 format!("{}{}", white, bishop), 
                 format!("{}{}", white, queen), 
                 format!("{}{}", white, king), 
                 format!("{}{}", white, bishop), 
                 format!("{}{}", white, knight), 
                 format!("{}{}", white, rook)
                ],
                [();8].map(|_| format!("{}{}", white, pawn)),
                [();8].map(|_| String::from("*")),
                [();8].map(|_| String::from("*")),
                [();8].map(|_| String::from("*")),
                [();8].map(|_| String::from("*")),
                [();8].map(|_| format!("{}{}", black, pawn)),
                [format!("{}{}", black, rook), 
                format!("{}{}", black, knight), 
                format!("{}{}", black, bishop), 
                format!("{}{}", black, queen), 
                format!("{}{}", black, king), 
                format!("{}{}", black, bishop), 
                format!("{}{}", black, knight), 
                format!("{}{}", black, rook)
               ]
            ] 
        }
    }

    /// If the current game state is InProgress and the move is legal,
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, from: String, to: String) -> Option<GameState> {
        let mut vec: Vec<String> = Vec::with_capacity(60);

        None
    }

    /// Set the piece type that a peasant becames following a promotion.
    pub fn set_promotion(&mut self, piece: String) -> () {
        ()
    }

    /// Get the current game state.
    // pub fn get_game_state(&self) -> GameState {
    //     self.state
    // }

    /// If a piece is standing on the given tile, return all possible
    /// new positions of that piece. Don't forget to the rules for check.
    ///
    /// (optional) Don't forget to include en passent and castling.
    pub fn get_possible_moves(&self, postion: String) -> Option<Vec<String>> {
        None
    }
}

/// Implement print routine for Game.
///
/// Output example:
/// 
/// 
/// 
/// 
/// |:----------------------:|
/// | R  Kn B  K  Q  B  Kn R |
/// | P  P  P  P  P  P  P  P |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | P  P  P  P  P  P  P  P |
/// | R  Kn B  K  Q  B  Kn R |
/// |:----------------------:|
/// 
/// 
/// 
/// 
/// 
impl  fmt::Debug for Game{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */

        write!(f, "This is game output!")
    }

}

fn game_board_format(game_board: [[String;8];8]){
    print!("+------------------------------------------+");
    for hosrisontal_grid in game_board{
        for (i, piece_name) in hosrisontal_grid.iter().enumerate(){
            if i == 0{
                print!("\n| ");
            }

            if piece_name.len() > 3{
                print!(" {}", piece_name)
            }
            else if piece_name.len() > 2{
                print!(" {} ", piece_name);
            }
            else{
                print!(" {}   ", piece_name);
            }

            if i == hosrisontal_grid.len() -1{
                print!(" |");
            }
        }
    }
    print!("\n+------------------------------------------+\n");
}

pub fn main(){
    println!("This is running from main.rs!");

    let game: Game = Game::new();

    println!("{:?}", game_board_format(game.game_board));
    // println!("{:?}", game.make_move(String::from("22"), String::from("23")))

    
}
