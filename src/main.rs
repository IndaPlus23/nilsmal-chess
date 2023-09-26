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

pub struct Move<'a>{
    game_board: &'a [[String;8];8],
    x1: i8,
    x2: i8,
    y1: i8,
    y2: i8
}

impl <'a> Move <'a>{
    fn new_move(game_board: &[[String;8];8], x1:i8, y1:i8, x2:i8, y2:i8) -> Move{
        Move{
            game_board: game_board,
            x1: x1 - 1,
            x2: x2 - 1,
            y1: y1 - 1,
            y2: y2 - 1
        }
    }

    pub fn initialise_new_move(self){
        if !self.check_for_piece_in_position(&self.x1, &self.y1){
            return println!("There is no piece in that position!");
        }
        let piece: String = self.game_board[self.y1 as usize][self.x1 as usize].clone();
        match piece.as_str(){
            "w_P" => self.pawn(),
            "b_P" => self.pawn(),
            "w_R" => self.rook(),
            "b_R" => self.rook(),
            "w_Kn" => self.knight(),
            "b_Kn" => self.knight(),
            "w_Bi" => self.bishop(),
            "b_Bi" => self.bishop(),
            "w_Q" => self.queen(),
            "b_Q" => self.queen(),
            "w_K" => self.king(),
            "b_K" => self.king(),
            _ => println!("invalid piece name found!")
        } 
    }

    fn check_for_piece_in_position(&self, x:&i8 ,y:&i8) -> bool{
        if self.game_board[*y as usize][*x as usize] == String::from("*"){
            return false;
        }
        else{
            return true;
        }
    }

    fn pawn(&self){
        if self.x1 == self.x2 && self.y1 == self.y2{
            println!("You can't move to the same position!");
            return
        }
        else if self.x1 != self.x2 && self.y1 != self.y2 && !self.check_for_piece_in_position(&self.x2, &self.y2){
            println!("You can't move diagonally!");
            return
        }
        else if self.x1 == self.x2 && self.y1 != self.y2{
            println!("You can't move horizontally!");
            return
        }
        else if self.x2 != self.x1 + 1 && self.y1 == self.y2{
            println!("You can't move that far!");
            return
        }
        println!("move granted!");
    }

    fn rook(&self){

    }

    fn knight(&self){

    }

    fn bishop(&self){

    }

    fn queen(&self){

    }

    fn king(&self){

    }
}

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

fn game_board_format(game_board: &[[String;8];8]){
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

    println!("{:?}", game_board_format(&game.game_board));
    // println!("{:?}", game.make_move(String::from("22"), String::from("23")))

    Move::new_move(&game.game_board, 1, 2, 3, 4).initialise_new_move();


    println!("\n\n");
    
}
