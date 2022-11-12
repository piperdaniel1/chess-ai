struct Board { 
    // Cells are represented as a 2D array of u8
    // 0,0 is the top left corner (a8)
    cells: [[u8; 8]; 8],

    // The current turn
    // True = white, False = black
    turn: bool,

    // En passant target square, if any
    en_passant: Option<Square>,

    // Castling rights
    // 0 = white king side, 1 = white queen side
    // 2 = black king side, 3 = black queen side
    castling: [bool; 4],

    // Halfmove clock
    // The number of halfmoves since the last capture or pawn advance
    halfmove_clock: u8,

    // Fullmove number
    // The number of the full move. 
    // It starts at 1, and is incremented after Black's move
    fullmove_number: u8,
}

struct Square { row: u8, col: u8 }
struct Move { from: Square, to: Square }

impl Move {
    fn new(from: Square, to: Square) -> Move {
        Move { from, to }
    }

    fn new_from_string(s: &str) -> Move {
        let from_square = Square::new_from_string(&s[0..2]);
        let to_square = Square::new_from_string(&s[2..4]);

        Move::new(from_square, to_square)
    }

    fn get_move_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.from.get_square_string());
        s.push_str(&self.to.get_square_string());
        s
    }
}

impl Square {
    fn new(row: u8, col: u8) -> Square {
        Square { row, col }
    }

    fn get_square_string(&self) -> String {
        let mut s = String::new();
        s.push((self.row + 97) as char);
        s.push((self.col + 49) as char);
        s
    }

    fn new_from_string(s: &str) -> Square {
        if s.len() != 2 {
            panic!("Invalid square string");
        }

        let mut chars = s.chars();
        let x = chars.next().unwrap() as u8 - 97;

        if x > 7 {
            panic!("Invalid square string");
        }

        let y = chars.next().unwrap() as u8 - 49;

        if y > 7 {
            panic!("Invalid square string");
        }

        Square { row: y, col: x }
    }
}

impl Board {
    fn new() -> Board {
        Board {
            cells: [[0; 8]; 8],
            turn: true,
            en_passant: None,
            castling: [true, true, true, true],
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }

    fn setup(&mut self) {
        // Black major pieces
        self.cells[0][0] = 9;
        self.cells[0][1] = 11;
        self.cells[0][2] = 10;
        self.cells[0][3] = 8;
        self.cells[0][4] = 7;
        self.cells[0][5] = 10;
        self.cells[0][6] = 11;
        self.cells[0][7] = 9;

        // Black pawns
        self.cells[1][0] = 12;
        self.cells[1][1] = 12;
        self.cells[1][2] = 12;
        self.cells[1][3] = 12;
        self.cells[1][4] = 12;
        self.cells[1][5] = 12;
        self.cells[1][6] = 12;
        self.cells[1][7] = 12;

        // White major pieces
        self.cells[7][0] = 3;
        self.cells[7][1] = 5;
        self.cells[7][2] = 4;
        self.cells[7][3] = 2;
        self.cells[7][4] = 1;
        self.cells[7][5] = 4;
        self.cells[7][6] = 5;
        self.cells[7][7] = 3;

        // White pawns
        self.cells[6][0] = 6;
        self.cells[6][1] = 6;
        self.cells[6][2] = 6;
        self.cells[6][3] = 6;
        self.cells[6][4] = 6;
        self.cells[6][5] = 6;
        self.cells[6][6] = 6;
        self.cells[6][7] = 6;

        // Empty cells
        for i in 2..6 {
            for j in 0..8 {
                self.cells[i][j] = 0;
            }
        }

        // Assign turn
        self.turn = true;
    }

    fn print(&self) {
        println!("    A   B   C   D   E   F   G   H");
        println!("  +---+---+---+---+---+---+---+---+");
        let mut row_ind = 0;
        for row in self.cells.iter() {
            row_ind += 1;

            print!("{} |", 9-row_ind);
            for cell in row.iter() {
                print!(" ");

                match cell {
                    0 => print!(" "),
                    1 => print!("♚"),
                    2 => print!("♛"),
                    3 => print!("♜"),
                    4 => print!("♝"),
                    5 => print!("♞"),
                    6 => print!("♟"),
                    7 => print!("♔"),
                    8 => print!("♕"),
                    9 => print!("♖"),
                    10 => print!("♗"),
                    11 => print!("♘"),
                    12 => print!("♙"),
                    _ => print!("?"),
                }

                print!(" ");
                print!("|");
            }

            println!("");
            println!("  +---+---+---+---+---+---+---+---+");
        }
    }

}

fn main() {
    let mut board = Board::new();
    board.setup();
    board.print();
}
