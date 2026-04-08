use crate::bit_board::BitBoard;
use crate::board::{BoardTrait, Color, Coordinate, PieceType};
use matrix_display::Format;
use matrix_display::*;

//@note : something about the square colors is backwards here btw..
//@bug : to fix
pub fn print_board(board: &dyn BoardTrait) {
    let mut board_cells = vec![];
    for row_idx in (1..=8).rev() {
        for col_idx in (1..=8) {
            let c = Coordinate::new(col_idx, row_idx);
            let piece_opt = board.get_piece_at(&c);
            let color = BitBoard::get_square_color_at(c);
            // ansi 8 bit color scheme
            let mut foreground = 0;
            let mut value = ' ';

            // if there's a piece
            if piece_opt.is_some() {
                let piece = piece_opt.unwrap();
                foreground = match piece.color {
                    Color::Black => 1, // red
                    Color::White => 5, // purple
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
            let ansi_bg = match color {
                Color::White => 0,
                Color::Black => 7,
            };
            board_cells.push(cell::Cell::new(value, foreground, ansi_bg));
        }
    }
    let format = Format::new(7, 3);
    let mut data = matrix::Matrix::new(8, board_cells);
    let display = MatrixDisplay::new(&format, &mut data);
    display.print(&mut std::io::stdout(), &style::BordersStyle::None);
}

pub fn print_bit_board(bit_board: &BitBoard) {
    let mut board_cells = vec![];
    // loop over rows in reverse 
    for row_idx in (1..=8).rev() {
        for col_idx in (1..=8) {
            let c = Coordinate::new(col_idx, row_idx);
            // ansi 8 bit color scheme
            let mut foreground = 0;
            let mut value = ' ';
            // get square color 
            let ansi_bg = match BitBoard::get_square_color_at(c) {
                Color::White => 7,
                Color::Black => 0,
            };

            let piece_opt = bit_board.get_piece_at(&c);
            if piece_opt.is_some() {
                let piece = piece_opt.unwrap();
                foreground = match piece.color {
                    Color::Black => 1, // red
                    Color::White => 5, // purple
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


            board_cells.push(cell::Cell::new(value, foreground, ansi_bg));
        }
    }


    let format = Format::new(7, 3);
    let mut data = matrix::Matrix::new(8, board_cells);
    let display = MatrixDisplay::new(&format, &mut data);
    display.print(&mut std::io::stdout(), &style::BordersStyle::None);
}
