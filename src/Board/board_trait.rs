use super::*;

// getting pieces && squares return references
pub trait BoardTrait {
    fn clone(&self) -> Box<dyn BoardTrait>;

    // info about game going on
    fn player_to_move(&self) -> Color;
    fn en_passant_target(&self) -> Option<Coordinate>;
    fn half_move_clock(&self) -> u32;
    fn full_move_number(&self) -> u32;
    fn can_castle_queen_side(&self, color: Color) -> bool;
    fn can_castle_king_side(&self, color: Color) -> bool;
    fn white_castling_rights(&self) -> CastlingRights;
    fn black_castling_rights(&self) -> CastlingRights;

    // getting squares
    fn squares_list(&self) -> Vec<&Square>;
    fn get_rank(&self, y: u8) -> Vec<&Square>;
    fn get_files(&self) -> Vec<Vec<&Square>>;
    fn get_squares(&self) -> Vec<Vec<&Square>>;

    // moves
    // fn make_move(&self, m: &Move) -> Self where Self: Sized ;
    fn make_move_mut(&mut self, m: &Move);
    // fn unmake_move(&self, m: &Move) -> Self where Self: Sized ;
    fn unmake_move_mut(&mut self, m: &Move);

    // getting and setting pieces
    fn place_piece(&mut self, piece: Piece, at: &Coordinate);
    fn remove_piece(&mut self, piece: &Piece) -> Piece;
    fn has_piece(&self, at: &Coordinate) -> bool;
    // fn get_pieces_in(&self, area: Vec<Coordinate>) -> Vec<(Coordinate, Option<&Piece>)>;
    fn get_piece_at(&self, at: &Coordinate) -> Option<&Piece>;
    fn get_kings(&self) -> Vec<&Piece>;
    fn get_pieces(&self, color: Color, piece_type: PieceType) -> Vec<&Piece>;
    fn get_all_pieces(&self, color: Color) -> Vec<&Piece>;
    fn get_castling_rights_changes_if_piece_moves(&self, piece: &Piece) -> Option<CastlingRights>;
    fn get_castling_rights_changes_if_piece_is_captured(&self, piece: &Piece) -> Option<CastlingRights>;
}
