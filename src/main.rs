use std::fmt;

// #[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress(bool),
    Check(bool),
    GameOver(bool),
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
            x1: x1,
            x2: x2,
            y1: y1,
            y2: y2,
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
        if !self.move_checker{println!("move granted!");}
        if self.check_for_piece_in_position(&self.x2, &self.y2){
            if !self.move_checker{println!("The target is populated!");}
            self.take_piece();
        }
        else{
            if !self.move_checker{println!("The target is empty!");}
            self.move_piece();
        }
        return self.return_board().clone();
    }

    fn pawn(&mut  self, is_white: bool) -> [[String;8];8]{
        if self.x1 != self.x2 && self.y1 != self.y2 && !self.check_for_piece_in_position(&self.x2, &self.y2){
            if !self.move_checker{println!("You can't move diagonally!");}
            return self.return_board().clone();
        }
        else if self.x1 != self.x2 && self.y1 == self.y2{
            println!("You can't move horizontally!");
            return self.return_board().clone();
        }
        else if self.y2 != self.y1 + 1 && self.x1 == self.x2 && is_white || self.y2 != self.y1 + 2 && self.x1 == self.x2 && self.y1 != 1 && is_white || self.y2 != self.y1 - 1 && self.x1 == self.x2 && !is_white || self.y2 != self.y1 - 2 && self.x1 == self.x2 && self.y1 != 6 && !is_white { 
            if !self.move_checker{println!("You can't move that far!");}
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
            if !self.move_checker{println!("You can't move diagonally!");}
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
            if !self.move_checker{println!("You can't move horizontally!");}
            return self.return_board().clone();
        } else if self.x1 == self.x2 && self.y1 != self.y2{
            if !self.move_checker{println!("You can't move vertically!");}
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
            if !self.move_checker{println!("You can't move that far!");}
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
            if !self.move_checker{println!("You can't move that far!");}
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
            if !self.move_checker{println!("You can't move that far!");}
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
            if !self.move_checker{println!("You can't move that far!");}
            return self.return_board().clone();
        }
        if !self.move_checker{println!("You can't move that far!");}
        return self.return_board().clone();
    }
}

pub struct Game{
    /* save board, active colour, ... */
    state: GameState,
    is_white_turn: bool,
    is_check: bool,
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
            state: GameState::InProgress(false),
            is_white_turn: true,
            is_check: false,
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
            // game_board: [
            //     [format!("{}{}", black, pawn), 
            //      format!("{}{}", white, knight), 
            //      format!("{}{}", white, bishop), 
            //      format!("{}{}", white, queen), 
            //      format!("{}{}", white, king), 
            //      format!("{}{}", white, bishop), 
            //      format!("{}{}", white, knight), 
            //      format!("{}{}", white, rook)
            //     ],
            //     [();8].map(|_| String::from("*")),
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
            ] 
        }
    }

    pub fn make_move(&mut self, from: String, to:String){
        let mut from_arr: Vec<String> = from.chars().map(|char| char.to_string()).collect();
        let mut to_arr: Vec<String> = to.chars().map(|char| char.to_string()).collect();

        fn match_function(mut input_vector:Vec<String>) -> Vec<String>{
            let x1: i8 = match input_vector[0].as_str(){
                "A" => 1,
                "B" => 2,
                "C" => 3,
                "D" => 4,
                "E" => 5,
                "F" => 6,
                "G" => 7,
                "H" => 8,
                _ => {
                    println!("Invalid move input!");
                    return input_vector;
                }
            };
            input_vector[0] = x1.to_string();
            return input_vector;
        }
        from_arr = match_function(from_arr);
        to_arr = match_function(to_arr);

        let x1: i8 = from_arr[0].parse::<i8>().unwrap();
        let y1: i8 = from_arr[1].parse::<i8>().unwrap();
        let x2: i8 = to_arr[0].parse::<i8>().unwrap();
        let y2: i8 = to_arr[1].parse::<i8>().unwrap();

        println!("{} {}", x1, y1);
        println!("{} {}", x2, y2);

        let mut _move = Move::new_move(self.game_board.clone(), x1 - 1, y1 - 1, x2 - 1, y2 - 1, false);
        _move.initialise_new_move();

        self.update_game_board(_move.game_board.clone());

        println!("{:?}", game_board_format(&self.game_board));

    }

    pub fn update_game_board(&mut self, game_board: [[String;8];8]){
        self.game_board = game_board;
        self.is_check = self.is_check();
        GameState::Check(self.is_check);
        GameState::InProgress(true);
        GameState::GameOver(false);
        self.is_white_turn = !self.is_white_turn;
    }

    /// Set the piece type that a peasant becames following a promotion.
    pub fn set_promotion(&mut self, x:i8, y:i8, piece: String) -> () {
        let x_index: i8 = x - 1;
        let y_index: i8 = y - 1;
        if piece.contains("K"){
            println!("You can't promote to king!");
            return
        } 
        if self.game_board[y_index as usize][x_index as usize].contains("P"){
            if self.game_board[y_index as usize][x_index as usize].contains("w_") && y == 8{
                println!("Promoting to: {}{}", "w_", piece);
                self.game_board[y_index as usize][x_index as usize] = format!("w_{}", piece);
            }
            else if self.game_board[y_index as usize][x_index as usize].contains("b_") && y == 1{
                self.game_board[y_index as usize][x_index as usize] = format!("b_{}", piece);
                println!("Promoting to: {}{}", "b_", piece);
            }else{
                println!("You can't promote!");
            }
        } else{
            println!("You can't promote pieces other than Pawns!");
        }
    }

    pub fn get_all_possible_moves(&mut self, x: i8, y:i8) -> Vec<[i8;2]>{
        let mut move_vector: Vec<[i8;2]> = Vec::new();
        for i in 1..9{
            for j in 1..9{
                println!("Moving pice: {} {} to position: {} {}", x, y, i, j);
                // create_move(game, x, y, i, j, true).initialise_new_move();
                let mut _move:Move = Move::new_move(self.game_board.clone(), x, y, i, j, true);
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

        move_vector
    }

    pub fn is_check(&self) -> bool{
        let mut king_position: [[i8;2];2] = [[0,0], [0,0]];
        for i in 0..8{
            for j in 0..8{
                if self.game_board[i][j].contains("w_K"){
                    king_position[0][0] = i as i8;
                    king_position[0][1] = j as i8;
                }
                if self.game_board[i][j].contains("b_K"){
                    king_position[1][0] = i as i8;
                    king_position[1][1] = j as i8;
                }
            }
        }

        for i in 0..8{
            for j in 0..8{
                if self.game_board[i][j].contains("b_"){
                    let mut _move:Move = Move::new_move(self.game_board.clone(), j as i8, i as i8, king_position[0][1], king_position[0][0], true);
                    _move.initialise_new_move();
                    if _move.move_granted{
                        println!("Black king is in check!");
                        return true;
                    }
                }
                if self.game_board[i][j].contains("w_"){
                    let mut _move:Move = Move::new_move(self.game_board.clone(), j as i8, i as i8, king_position[1][1], king_position[1][0], true);
                    _move.initialise_new_move();
                    if _move.move_granted{
                        println!("White king is in check!");
                        return true;
                    }
                }
            }
        }

        false
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

fn create_move(game: &Game, x1: i8, y1: i8, x2: i8, y2: i8, move_checker: bool) -> [[String; 8]; 8] {
    let mut _move = Move::new_move(game.game_board.clone(), x1, y1, x2, y2, move_checker);
    _move.initialise_new_move();
    return _move.game_board.clone()
}

pub fn main(){
    println!("This is running from main.rs!");

    let mut game: Game = Game::new();

    game.make_move(String::from("D2"), String::from("D3"));
    game.make_move(String::from("D3"), String::from("D4"));
    game.make_move(String::from("D4"), String::from("D5"));
    game.make_move(String::from("D5"), String::from("D6"));



    println!("\n\n");
    
}
