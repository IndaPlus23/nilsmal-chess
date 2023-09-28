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
pub struct Move{
    game_board: [[String;8];8],
    x1: i8,
    x2: i8,
    y1: i8,
    y2: i8,
    move_checker: bool,
    move_granted: bool
}

impl Move{
    fn new_move(game_board: [[String;8];8], x1:i8, y1:i8, x2:i8, y2:i8, move_checker: bool) -> Move{
        Move{
            game_board: game_board,
            x1: x1 - 1,
            x2: x2 - 1,
            y1: y1 - 1,
            y2: y2 - 1,
            move_checker: move_checker,
            move_granted: false
        }
    }

    pub fn initialise_new_move(&mut self) -> Move{
        Move { 
            game_board: self.initialise_new_move_inner(), 
            x1: self.x1, 
            x2: self.x2, 
            y1: self.y1, 
            y2: self.y2, 
            move_checker: self.move_checker, 
            move_granted: self.move_granted 
        }
    }

    pub fn initialise_new_move_inner(&mut self) -> [[String;8];8]{
        self.move_granted = false;
        if !self.check_for_piece_in_position(&self.x1, &self.y1){
            println!("There is no piece in that position!");
            return self.return_board().clone();
        }
        if self.check_if_outside_bounds(){
            println!("You can't move outside the board!");
            return self.return_board().clone();
        }

        let piece: String = self.game_board[self.y1 as usize][self.x1 as usize].clone();
        match piece.as_str(){
            "w_P" => self.pawn(true),
            "b_P" => self.pawn(false),
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
            _=> self.return_board().clone()
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

    fn check_if_outside_bounds(&mut self) -> bool{
        if self.x2 < 0 || self.x2 > 7 || self.y2 < 0 || self.y2 > 7{
            return true
        }
        false
    }

    fn move_piece(&mut self){
        let piece: String = self.game_board[self.y1 as usize][self.x1 as usize].clone();
        self.game_board[self.y1 as usize][self.x1 as usize] = String::from("*");
        self.game_board[self.y2 as usize][self.x2 as usize] = String::from(piece);
        return;
    }

    fn return_board(&self) -> &[[String;8];8]{
        return &self.game_board;
    }

    fn check_if_piece_moves_though_another_piece(&self) -> bool{
        // the piece is moving horisontal or vertical
        if self.x1 == self.x2 || self.y1 == self.y2{ 
            // the piece is moving vertical 
            if self.x1 == self.x2{
                // the piece is moving up
                if self.y1 < self.y2{
                    for i in self.y1+1..self.y2 -1{
                        if self.check_for_piece_in_position(&self.x1, &i){
                            return true;
                        }
                    }
                    return false;
                }
                // the piece is moving down
                else{
                    for i in self.y2+1..self.y1 -1{
                        if self.check_for_piece_in_position(&self.x1, &i){
                            return true;
                        }
                    }
                    return false;
                }
            }
            // the piece is moving horisontal
            else{
                // the piece is right up
                if self.x1 < self.x2{
                    for i in self.x1+1..self.x2 -1{
                        if self.check_for_piece_in_position(&i, &self.y1){
                            return true;
                        }
                    }
                    return false;
                }
                // the piece is moving left
                else{
                    for i in self.x2+1..self.x1 -1{
                        if self.check_for_piece_in_position(&i, &self.y1){
                            return true;
                        }
                    }
                    return false;
                }
            }
        } else{
            return false;
        }
    }

    fn take_piece(&mut self){
        // check that the target piece is of opposite colour
        if self.game_board[self.y1 as usize][self.x1 as usize].contains("w_") && self.game_board[self.y2 as usize][self.x2 as usize].contains("b_") ||
           self.game_board[self.y1 as usize][self.x1 as usize].contains("b_") && self.game_board[self.y2 as usize][self.x2 as usize].contains("w_"){
            println!("You took a piece!");
            self.move_piece();
        }
        else {
            println!("You can't take your own piece!");
        }
    }

    fn common_piece_move_logic(&mut self) -> [[String;8];8]{
        if self.x1 == self.x2 && self.y1 == self.y2{
            println!("You can't move to the same position!");
            return self.return_board().clone();
        }
        if self.move_checker{
            self.move_granted = true;
            return self.return_board().clone();
        }
        println!("move granted!");
        if self.check_for_piece_in_position(&self.x2, &self.y2){
            println!("The target is populated!");
            self.take_piece();
        }
        else{
            println!("The target is empty!");
            self.move_piece();
        }
        return self.return_board().clone();
    }

    fn pawn(&mut  self, is_white: bool) -> [[String;8];8]{
        if self.x1 != self.x2 && self.y1 != self.y2 && !self.check_for_piece_in_position(&self.x2, &self.y2){
            println!("You can't move diagonally!");
            return self.return_board().clone();
        }
        else if self.x1 != self.x2 && self.y1 == self.y2{
            println!("You can't move horizontally!");
            return self.return_board().clone();
        }
        else if self.y2 != self.y1 + 1 && self.x1 == self.x2 && is_white || self.y2 != self.y1 + 2 && self.x1 == self.x2 && self.y1 != 1 && is_white || self.y2 != self.y1 - 1 && self.x1 == self.x2 && !is_white || self.y2 != self.y1 - 2 && self.x1 == self.x2 && self.y1 != 6 && !is_white { 
            println!("You can't move that far!");
            return self.return_board().clone();
        }
        if self.check_if_piece_moves_though_another_piece(){
            println!("A piece can't be moved through another piece!");
            return self.return_board().clone();
        }
        return self.common_piece_move_logic();
    }

    fn rook(&mut self) -> [[String;8];8]{
        if self.x1 != self.x2 && self.y1 != self.y2{
            println!("You can't move diagonally!");
            return self.return_board().clone();
        }
        if self.check_if_piece_moves_though_another_piece(){
            println!("A piece can't be moved through another piece!");
            return self.return_board().clone();
        }
        return self.common_piece_move_logic();
    }

    fn knight(&mut self) -> [[String;8];8]{
        if self.x1 != self.x2 && self.y1 == self.y2{
            println!("You can't move horizontally!");
            return self.return_board().clone();
        } else if self.x1 == self.x2 && self.y1 != self.y2{
            println!("You can't move vertically!");
            return self.return_board().clone();
        }
        else if self.x1 == self.x2 + 1 && self.y1 == self.y2 + 2 ||
                self.x1 == self.x2 + 1 && self.y1 == self.y2 - 2 ||
                self.x1 == self.x2 - 1 && self.y1 == self.y2 + 2 ||
                self.x1 == self.x2 - 1 && self.y1 == self.y2 - 2 ||
                self.x1 == self.x2 + 2 && self.y1 == self.y2 + 1 ||
                self.x1 == self.x2 + 2 && self.y1 == self.y2 - 1 ||
                self.x1 == self.x2 - 2 && self.y1 == self.y2 + 1 ||
                self.x1 == self.x2 - 2 && self.y1 == self.y2 - 1{
            return self.common_piece_move_logic();
        }
        else{
            println!("You can't move that far!");
            return self.return_board().clone();
        }
    }

    fn bishop(&mut self) -> [[String;8];8]{
        if self.x1 == self.x2 || self.y1 == self.y2{
            println!("You can't move horizontally or vertically!");
            return self.return_board().clone();
        }
        if self.check_if_piece_moves_though_another_piece(){
            println!("A piece can't be moved through another piece!");
            return self.return_board().clone();
        }
        if self.x1 - self.x2 == self.y1 - self.y2 || self.x1 - self.x2 == self.y2 - self.y1{
            return self.common_piece_move_logic();
        }
        else{
            println!("You can't move that far!");
            return self.return_board().clone();
        }
    }

    fn queen(&mut self) -> [[String;8];8]{
        if self.check_if_piece_moves_though_another_piece(){
            println!("A piece can't be moved through another piece!");
            return self.return_board().clone();
        }
        if self.x1 - self.x2 == self.y1 - self.y2 || self.x1 - self.x2 == self.y2 - self.y1 || self.x1 == self.x2 || self.y1 == self.y2{
            return self.common_piece_move_logic();
        }
        else{
            println!("You can't move that far!");
            return self.return_board().clone();
        }
    }

    fn king(&mut self) -> [[String;8];8]{
        if self.check_if_piece_moves_though_another_piece(){
            println!("A piece can't be moved through another piece!");
            return self.return_board().clone();
        }
        else if self.x2 == self.x1 + 1 && self.y2 == self.y1 + 1{
            return self.common_piece_move_logic();
        } else if self.x2 == self.x1 - 1 && self.y2 == self.y1 - 1{
            return self.common_piece_move_logic();
        } else if self.x2 == self.x1 + 1 && self.y2 == self.y1 - 1{
            return self.common_piece_move_logic();
        } else if self.x2 == self.x1 - 1 && self.y2 == self.y1 + 1{
            return self.common_piece_move_logic(); 
        } else if self.x2 == self.x1 && self.y2 == self.y1 + 1{
            return self.common_piece_move_logic(); 
        } else if self.x2 == self.x1 && self.y2 == self.y1 - 1{
            return self.common_piece_move_logic(); 
        } else if self.x2 == self.x1 + 1 && self.y2 == self.y1{
            return self.common_piece_move_logic(); 
        } else if self.x2 == self.x1 - 1 && self.y2 == self.y1{
            return self.common_piece_move_logic(); 
        }
        else{
            println!("You can't move that far!");
            return self.return_board().clone();
        }
        println!("You can't move that far!");
        return self.return_board().clone();
    }
}

pub struct Game{
    /* save board, active colour, ... */
    state: GameState,
    is_white_turn: bool,
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
            is_white_turn: true,
            // game_board: [
            //     [format!("{}{}", white, rook), 
            //      format!("{}{}", white, knight), 
            //      format!("{}{}", white, bishop), 
            //      format!("{}{}", white, queen), 
            //      format!("{}{}", white, king), 
            //      format!("{}{}", white, bishop), 
            //      format!("{}{}", white, knight), 
            //      format!("{}{}", white, rook)
            //     ],
            //     [();8].map(|_| format!("{}{}", white, pawn)),
            //     [();8].map(|_| String::from("*")),
            //     [();8].map(|_| String::from("*")),
            //     [();8].map(|_| String::from("*")),
            //     [();8].map(|_| String::from("*")),
            //     [();8].map(|_| format!("{}{}", black, pawn)),
            //     [format!("{}{}", black, rook), 
            //     format!("{}{}", black, knight), 
            //     format!("{}{}", black, bishop), 
            //     format!("{}{}", black, queen), 
            //     format!("{}{}", black, king), 
            //     format!("{}{}", black, bishop), 
            //     format!("{}{}", black, knight), 
            //     format!("{}{}", black, rook)
            //    ]
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
                [();8].map(|_| String::from("*")),
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

    fn update_game_board(&mut self, game_board: [[String;8];8]){
        self.game_board = game_board;
        self.is_white_turn = !self.is_white_turn;
    }

    /// Set the piece type that a peasant becames following a promotion.
    pub fn set_promotion(&mut self, x:i8, y:i8, piece: String) -> () {
        if piece.contains("Q"){
            print("You can't promote to king!");
            return
        }
        if self.game_board[y as usize][x as usize].contains("P"){
            if self.game_board[y as usize][x as usize].contains("w_") && y == 7{
                self.game_board[y as usize][x as usize] = format!("w_{}", piece);
            }
            else if self.game_board[y as usize][x as usize].contains("b_") && y == 0{
                self.game_board[y as usize][x as usize] = format!("b_{}", piece);
            }else{
                println!("You can't promote!");
            }
        }
    }

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

fn get_all_possible_moves(game: &mut Game, x: i8, y:i8){
    let mut move_vector: Vec<[i8;2]> = Vec::new();
    for i in 1..9{
        for j in 1..9{
            println!("Moving pice: {} {} to position: {} {}", x, y, i, j);
            // create_move(game, x, y, i, j, true).initialise_new_move();
            let mut _move:Move = Move::new_move(game.game_board.clone(), x, y, i, j, true);
            _move.initialise_new_move();
            if _move.move_granted{
                move_vector.push([i, j])
            }
        }
    }
    println!("All possible moves for: {} {} is:", x, y);

    for e in move_vector.iter(){
        println!("{} {}", e[0], e[1]);
    }
}

fn create_move(game: &Game, x1: i8, y1: i8, x2: i8, y2: i8, move_checker: bool) -> [[String; 8]; 8] {
    let mut _move = Move::new_move(game.game_board.clone(), x1, y1, x2, y2, move_checker);
    _move.initialise_new_move();
    return _move.game_board.clone()
}

pub fn main(){
    println!("This is running from main.rs!");

    let mut game: Game = Game::new();

    println!("{:?}", game_board_format(&game.game_board));

    game.update_game_board(create_move(&game,5, 1, 5, 2, false));

    println!("{:?} \n {}", game_board_format(&game.game_board), &game.is_white_turn);
    
    game.update_game_board(create_move(&game,5, 2, 5, 3, false));
    
    println!("{:?} \n {}", game_board_format(&game.game_board), &game.is_white_turn);


    game.update_game_board(create_move(&game,5, 3, 4, 2, false));
    
    println!("{:?} \n {}", game_board_format(&game.game_board), &game.is_white_turn);


    game.update_game_board(create_move(&game,4, 2, 3, 2, false));
    
    println!("{:?} \n {}", game_board_format(&game.game_board), &game.is_white_turn);

    game.set_promotion(1, 1, String::from("Q"));

    get_all_possible_moves(&mut game, 1, 1);

    println!("\n\n");
    
}
