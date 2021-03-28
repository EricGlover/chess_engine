use crate::board::{Board, Color, PieceType, Square};
use matrix_display::Format;
use matrix_display::*;

pub fn print_board(board: &Board) {
    let board_cells = board
        .get_squares()
        .iter()
        .rev()
        .map(|square| {
            // ansi 8 bit color scheme
            let mut foreground = 0;
            let mut value = ' ';

            // if there's a piece
            if square.piece.is_some() {
                let piece = square.piece.unwrap();
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
            let ansi_bg = match square.color {
                Color::White => 0,
                Color::Black => 7,
            };
            cell::Cell::new(value, foreground, ansi_bg)
        })
        .collect();
    let format = Format::new(7, 3);
    let mut data = matrix::Matrix::new(8, board_cells);
    let display = MatrixDisplay::new(&format, &mut data);
    display.print(&mut std::io::stdout(), &style::BordersStyle::None);
}
