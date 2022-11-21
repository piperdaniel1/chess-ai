mod board;

// Goal: Make board.legal_moves() a function that takes
// an immutable self reference while also not having it
// make a copy of itself.
//
// This is tricky because we have to statically analyze the
// board to determine if a given psuedo-legal move will be
// legal or not.
//
// Mainly, we worry about three main cases:
// 1. Normal pieces moving
//     - Here we are mainly just worried about the piece being
//       pinned to the king.
// 2. The king moving one square
//     - Here we are only worried about the king moving into
//       a square that is attacked by an enemy piece.
// 3. The king castling
//     - Here we are also worried about (2) but also must consider
//       the square between the king and where it is castling to.
//
// It seems like 2 and 3 could be solved by storing which squares
// are under attack by which pieces.

fn main() {
    let board = board::Board::new();
    board.print();
    // let first_space_ind = test_str.find(' ').unwrap();
    // println!("first_space_ind: {}", first_space_ind);
    // println!("chars around first space: {}",
    //          &test_str[first_space_ind - 1..first_space_ind + 2]);
    // println!("Char right before first space: {}",
    //          test_str.chars().nth(first_space_ind - 1).unwrap());

    // let second_space_ind = &test_str[first_space_ind..&test_str].find(' ').unwrap();
    // println!("second_space_ind: {}", second_space_ind);
    
    // let matches = test_str.match_indices(' ').collect::<Vec<_>>();
    // println!("matches: {:?}", matches);
    // let collection: Vec<&str> = test_str.split(' ').collect();
    // println!("collection: {:?}", collection);
    /*
    let mut board = board::Board::new();
    board.setup();
    board.print();
    println!("Fen: {}", board.fen());*/
}
