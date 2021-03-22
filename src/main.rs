use console::{style, Attribute};

use matrix_display::*;
use std::thread;
use std::time::Duration;

use console::Term;
use std::cell::Cell;

/**
                                                     _:_
                                                    '-.-'
                                           ()      __.'.__
                                        .-:--:-.  |_______|
                                 ()      \____/    \=====/
                                 /\      {====}     )___(
                      (\=,      //\\      )__(     /_____\
      __    |'-'-'|  //  .\    (    )    /____\     |   |
     /  \   |_____| (( \_  \    )__(      |  |      |   |
     \__/    |===|   ))  `\_)  /____\     |  |      |   |
    /____\   |   |  (/     \    |  |      |  |      |   |
     |  |    |   |   | _.-'|    |  |      |  |      |   |
     |__|    )___(    )___(    /____\    /____\    /_____\
    (====)  (=====)  (=====)  (======)  (======)  (=======)
    }===={  }====={  }====={  }======{  }======{  }======={
   (______)(_______)(_______)(________)(________)(_________)
   author -  Joan G. Stark
**/
#[derive(Copy, Clone)]
enum Color {
    White,
    Black
}

#[derive(Copy, Clone)]
struct Piece {
    piece_type: PieceType,
    color: Color
}

impl Piece {
    fn make_white_pawn() -> Piece {
        Piece{piece_type: PieceType::Pawn, color: Color::White}
    }
    fn make_black_pawn() -> Piece {
        Piece{piece_type: PieceType::Pawn, color: Color::Black}
    }
    fn make_white_rook() -> Piece {
        Piece{piece_type: PieceType::Rook, color: Color::White}
    }
    fn make_black_rook() -> Piece {
        Piece{piece_type: PieceType::Rook, color: Color::Black}
    }
    fn make_white_knight() -> Piece {
        Piece{piece_type: PieceType::Knight, color: Color::White}
    }
    fn make_black_knight() -> Piece {
        Piece{piece_type: PieceType::Knight, color: Color::Black}
    }
    fn make_white_bishop() -> Piece {
        Piece{piece_type: PieceType::Bishop, color: Color::White}
    }
    fn make_black_bishop() -> Piece {
        Piece{piece_type: PieceType::Bishop, color: Color::Black}
    }
    fn make_white_queen() -> Piece {
        Piece{piece_type: PieceType::Queen, color: Color::White}
    }
    fn make_black_queen() -> Piece {
        Piece{piece_type: PieceType::Queen, color: Color::Black}
    }
    fn make_white_king() -> Piece {
        Piece{piece_type: PieceType::King, color: Color::White }
    }
    fn make_black_king() -> Piece {
        Piece{piece_type: PieceType::King, color: Color::Black}
    }
}

#[derive(Copy, Clone)]
enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}
struct Coordinate {
    x: u8,
    y: u8,
}
struct Square<'a> {
    coordinate: Coordinate,
    piece: Option<&'a Piece>,
}
// @todo : make squares and board 
//
// fn make_row() -> Vec<Square> {
//
// }
//
// fn make_squares() -> Vec<Vec<Square>> {
//
// }

fn make_initial_board() -> Vec<Option<Piece>> {
    vec![
        Some(Piece::make_black_rook()), Some(Piece::make_black_knight()), Some(Piece::make_black_bishop()), Some(Piece::make_black_queen()), Some(Piece::make_black_king()), Some(Piece::make_black_bishop()),  Some(Piece::make_black_knight()), Some(Piece::make_black_rook()),
        Some(Piece::make_black_pawn()),Some(Piece::make_black_pawn()), Some(Piece::make_black_pawn()), Some(Piece::make_black_pawn()), Some(Piece::make_black_pawn()), Some(Piece::make_black_pawn()), Some(Piece::make_black_pawn()), Some(Piece::make_black_pawn()),
        None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None,
        Some(Piece::make_white_pawn()),Some(Piece::make_white_pawn()), Some(Piece::make_white_pawn()), Some(Piece::make_white_pawn()), Some(Piece::make_white_pawn()), Some(Piece::make_white_pawn()), Some(Piece::make_white_pawn()), Some(Piece::make_white_pawn()),
        Some(Piece::make_white_rook()), Some(Piece::make_white_knight()), Some(Piece::make_white_bishop()), Some(Piece::make_white_queen()), Some(Piece::make_white_king()), Some(Piece::make_white_bishop()),  Some(Piece::make_white_knight()), Some(Piece::make_white_rook()),
    ]
}

fn main() {
    // I can use the terminal but I don't quite see the point .... at least it's interactive / clearable
    // use terminal to print a board .....

    // let term = Term::stdout();
    //
    // term.write_line(style("Hello World! !♔♔♔♔♔♔").cyan().attr(Attribute::Bold).to_string().as_str());
    // thread::sleep(Duration::from_millis(2000));
    // term.clear_line();

    // println!("Hello, world!♔♔♔♔♔♔♕");
    // println!("This is {} neat", style("quite").cyan());

    // how to print board
    // squares background color set to black and white
    // pieces color set to red and green ?
    // pieces are letters (maybe use ascii art later
    // use the matrix thing for the background colors
    let board: Vec<Option<Piece>> = make_initial_board();
    print_board(&board);
}

fn print_board(board: &Vec<Option<Piece>>) {
    let format = Format::new(7, 3);
    let board_cells = board
        .iter()
        .enumerate()
        .map(|(i, x)| {
            // @todo: check if None ?
            // ansi 8 bit color scheme
            let mut foreground = 0;
            let mut value = ' ';
            if x.is_some() {
                let piece = x.unwrap();
                foreground =  match piece.color {
                    Color::Black => 1,  // red
                    Color::White => 2,  // green
                };
                value = match piece.piece_type {
                    PieceType::King => 'K',
                    PieceType::Queen => 'Q',
                    PieceType::Bishop => 'B',
                    PieceType::Knight => 'N',
                    PieceType::Rook => 'R',
                    PieceType::Pawn => 'P',
                }
            }

            // @todo : change to use square color later
            let mut ansi_bg = 0;
            if i % 2 + (i / 8) % 2 == 1 {
                ansi_bg = 7;
            }
            cell::Cell::new(value, foreground, ansi_bg)
        })
        .collect::<Vec<_>>();
    let mut data = matrix::Matrix::new(8, board_cells);
    let mut display = MatrixDisplay::new(&format, &mut data);
    display.print(&mut std::io::stdout(), &style::BordersStyle::None);
}
