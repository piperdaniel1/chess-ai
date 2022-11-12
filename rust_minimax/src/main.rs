mod board;

fn main() {
    let test_str = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
    let board = board::Board::new_from_fen(test_str);
    let mut board = match board {
        Ok(b) => b,
        Err(_) => panic!("Error, invalid FEN provided."),
    };
    board.print();

    board.import_from_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2").unwrap();
    board.print();

    board.import_from_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2").unwrap();
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
