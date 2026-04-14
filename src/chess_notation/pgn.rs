use regex::Regex;

use crate::board::{Board, BoardTrait, Color, Coordinate, PieceType, CastlingRights};
use crate::chess_notation::fen_reader::make_board;
use crate::chess_notation::read_move;
use crate::chess_notation::{fen_reader, ParsedMoveType};
use crate::game::Game as chess_game;
use crate::game_state::GameState;
use crate::move_generator::{
    self, gen_legal_moves, pseudo_legal_move_generator as p_gen, Move, MoveType,
};
use std::fmt;
use std::fmt::Formatter;
use std::fs;

const TEST_PGN: &str = r#"[Event "F/S Return Match"]
[Site "Belgrade, Serbia JUG"]
[Date "1992.11.04"]
[Round "29"]
[White "Fischer, Robert J."]
[Black "Spassky, Boris V."]
[Result "1/2-1/2"]

1. e4 e5 2. Nf3 Nc6 3. Bb5 a6 {This opening is called the Ruy Lopez.}
4. Ba4 Nf6 5. O-O Be7 6. Re1 b5 7. Bb3 d6 8. c3 O-O 9. h3 Nb8 10. d4 Nbd7
11. c4 c6 12. cxb5 axb5 13. Nc3 Bb7 14. Bg5 b4 15. Nb1 h6 16. Bh4 c5 17. dxe5
Nxe4 18. Bxe7 Qxe7 19. exd6 Qf6 20. Nbd2 Nxd6 21. Nc4 Nxc4 22. Bxc4 Nb6
23. Ne5 Rae8 24. Bxf7+ Rxf7 25. Nxf7 Rxe1+ 26. Qxe1 Kxf7 27. Qe3 Qg5 28. Qxg5
hxg5 29. b3 Ke6 30. a3 Kd6 31. axb4 cxb4 32. Ra5 Nd5 33. f3 Bc8 34. Kf2 Bf5
35. Ra7 g6 36. Ra6+ Kc5 37. Ke1 Nf4 38. g3 Nxh3 39. Kd2 Kb5 40. Rd6 Kc5 41. Ra6
Nf2 42. g4 Bd3 43. Re6 1/2-1/2"#;

enum Termination {
    Abandoned,
    Adjudication,
    Death,
    Emergency,
    Normal,
    Time,
    Forfeit,
    RulesInfraction,
    Unterminated,
}
impl fmt::Display for Termination {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let str = match self {
            Termination::Abandoned => "abandoned",
            Termination::Adjudication => "adjudication",
            Termination::Death => "death",
            Termination::Emergency => "emergency",
            Termination::Normal => "normal",
            Termination::Time => "time",
            Termination::Forfeit => "forfeit",
            Termination::RulesInfraction => "rulesInfraction",
            Termination::Unterminated => "unterminated",
        };
        write!(f, "{}", str)
    }
}
enum Mode {
    OverTheBoard,
    InternetChessServer,
}
impl fmt::Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let str = match self {
            Mode::OverTheBoard => "OTB",
            Mode::InternetChessServer => "ICS",
        };
        write!(f, "{}", str)
    }
}

pub fn read_pgn_file() {
    //  let res = fs::metadata("./Games");
    //     let is_dir: bool = match res {
    //         Ok(f) => f.is_dir(),
    //         Err(err) => false,
    //     };
    //     if is_dir {
    //         let mut path_str = format!("./Games/");
    //         path_str.push_str("1.pgn");
    //         let path = Path::new(path_str.as_str());
    //         let display = path.display();
    //         let mut contents: String = match std::fs::read_to_string(&path) {
    //             Err(err) => panic!("couldn't read {}: {}", display, err),
    //             Ok(file) => file,
    //         };
    //         contents = contents
    //             .trim_start_matches('\u{feff}')
    //             .replace("\r\n", "\n");
    //         let moves = pgn::moves_from_pgn(contents.as_str());
    //         game.run_sim_game(moves);
    //         return;
}

pub fn moves_from_pgn(pgn: &str) -> Vec<Move> {
    // reading from a pgn time
    let mut moves: Vec<Move> = Vec::new();
    // let mut scrap_board = fen_reader::make_board(fen_reader::INITIAL_BOARD);
    let mut scrap_game_state = GameState::starting_game();

    // for the moment were skipping the informational section
    // break pgn into lines then grab all the lines with move text in them
    // let lines: Vec<&str> = pgn.split('\n').collect();

    let info_line_matcher = Regex::new(r"\s?\[.*\]\s?$").unwrap();
    let empty_space_matcher = Regex::new(r"^\s$").unwrap();
    let mut move_lines: Vec<&str> = Vec::new();
    // let mut info_section = true;
    // let mut move_section = false;
    for _s in pgn.split("\n").into_iter() {
        if info_line_matcher.is_match(_s) {
            continue;
        } else if _s.is_empty() || empty_space_matcher.is_match(_s) {
            continue;
        } else {
            move_lines.push(_s);
        }
    }

    let move_text = move_lines.join("");

    // remove the comments
    let comment_matcher = Regex::new(r"(\{[^*\}]*\})").unwrap();
    let stuff: Vec<&str> = comment_matcher.split(&move_text).collect();

    // put it in a string again
    let mut move_text_2 = String::new();
    for _s in stuff {
        move_text_2.push_str(_s);
    }

    // take out all move turn stuff (eg. 1. 2. 3. )
    let turn_matcher = Regex::new(r"(\d+\.)").unwrap();
    let mut t = String::new();
    let stuff: Vec<&str> = turn_matcher.split(move_text_2.as_str()).collect();
    for _s in stuff {
        t.push_str(_s);
    }

    //now we have mostly just move text and some $1, $2 stuff && empty lines
    let move_candidates: Vec<&str> = t.split(' ').collect();
    let mut color_to_move = Color::White;

    for _m in move_candidates {
        // skip empty
        if _m.is_empty() {
            continue;
        }
        // if is valid move

        let res = read_move(_m);
        if res.is_none() {
            // println!("MOVE NOT FOUND");
            continue;
        } else {
            let (piece_type, coordinate, parsed_move, promotion_type) = res.unwrap();
            println!(
                "{} {} {} ",
                piece_type,
                coordinate.unwrap_or(Coordinate::new(0, 0)),
                promotion_type.unwrap_or(PieceType::Pawn)
            );

            if coordinate.is_none()
                && (ParsedMoveType::LongCastles == parsed_move
                    || ParsedMoveType::ShortCastles == parsed_move)
            {
                // @todo :: handle castles
                let mut new_move = match parsed_move {
                    ParsedMoveType::LongCastles => Move::castle_queen_side(&CastlingRights::new(false, false), color_to_move ),
                    ParsedMoveType::ShortCastles => Move::castle_king_side(&CastlingRights::new(false, false), color_to_move ),
                    ParsedMoveType::Promotion => break,
                    ParsedMoveType::Move => break,
                };
                moves.push(new_move);
                scrap_game_state.make_move_mut(&mut new_move);
                color_to_move = match color_to_move {
                    Color::White => Color::Black,
                    Color::Black => Color::White,
                };
                continue;
            }

            if promotion_type.is_some() {
                //@todo
            } else {
            }

            let to = coordinate.unwrap();
            // //@todo :::: make moves
            let mut found_pieces =
                scrap_game_state.find_pieces_can_move_to_square(color_to_move, piece_type, to);
            if found_pieces.is_empty() {
                break;
            }
            let found_piece = found_pieces.pop().unwrap();
            if found_piece.at().is_none() {
                break;
            }
            let from = found_piece.at().unwrap();
            let move_type = match parsed_move {
                ParsedMoveType::LongCastles => break,
                ParsedMoveType::ShortCastles => break,
                ParsedMoveType::Promotion => MoveType::Promotion(promotion_type.unwrap()),
                ParsedMoveType::Move => MoveType::Move,
            };
            let mut new_move = p_gen::make_move_to(from, &to, found_piece, move_type, &scrap_game_state);
            moves.push(new_move);
            scrap_game_state.make_move_mut(&mut new_move);
            //@todo :: this roughly works

            // flip color to move
            color_to_move = match color_to_move {
                Color::White => Color::Black,
                Color::Black => Color::White,
            };
        }
    }

    return moves;
}

pub struct Game {
    pub event: String,
    pub site: String,
    pub date: String,
    pub round: String,
    pub white: String,
    pub black: String,
    pub result: String,
    pub move_text: String,
}

impl Game {
    pub fn new_from_pgn(pgn: String) -> chess_game {
        // reading from a pgn time

        // for the moment were skipping the informational section
        // break pgn into lines then grab all the lines with move text in them
        let lines: Vec<&str> = pgn.split('\n').collect();
        let info_line_matcher = Regex::new(r"\[.*\]$").unwrap();
        let mut move_lines: Vec<&str> = Vec::new();
        let info_section = true;
        let mut move_section = false;
        for _s in pgn.split('\n').into_iter() {
            if move_section {
                move_lines.push(_s);
            }
            if !info_line_matcher.is_match(_s) && _s.is_empty() {
                move_section = true;
            }
        }

        // put all move text in a string
        let mut move_text: String = String::new();
        for _s in move_lines.into_iter() {
            
            move_text.push_str(_s);
        }

        // remove the comments
        let comment_matcher = Regex::new(r"(\{[^*\}]*\})").unwrap();
        let stuff: Vec<&str> = comment_matcher.split(&move_text).collect();

        // put it in a string again
        let mut move_text_2 = String::new();
        for _s in stuff {
            move_text_2.push_str(_s);
        }

        // take out all move turn stuff (eg. 1. 2. 3. )
        let turn_matcher = Regex::new(r"(\d+\.)").unwrap();
        let mut t = String::new();
        let stuff: Vec<&str> = turn_matcher.split(move_text_2.as_str()).collect();
        for _s in stuff {
            t.push_str(_s);
        }

        //now we have mostly just move text and some $1, $2 stuff && empty lines
        let move_candidates: Vec<&str> = t.split(' ').collect();
        for _m in move_candidates {
            // skip empty
            if _m.is_empty() {
                continue;
            }
            // if is valid move
            // do stuff
            
            let res = read_move(_m);
            if res.is_none() {
                println!("MOVE NOT FOUND");
            } else {
                let (piece_type, coordinate, parsed_move, promotion_type) = res.unwrap();
                // println!(
                //     "{} {} {} ",
                //     piece_type,
                //     coordinate.unwrap_or(Coordinate::new(0, 0)),
                //     promotion_type.unwrap_or(PieceType::Pawn)
                // );
            }
        }
        return chess_game::new();
    }
    pub fn new_from_game(game: &chess_game) -> Game {
        // partition and step_by work here ?
        // try step_by and zip
        // zip together moves
        fn make_move_pairs(mut moves: Vec<String>) -> Vec<(String, Option<String>)> {
            let mut pairs = Vec::new();
            while moves.len() > 0 {
                let first = moves.remove(0);
                let second = if moves.len() > 0 {
                    Some(moves.remove(0))
                } else {
                    None
                };
                pairs.push((first, second));
            }
            pairs
        }
        let pairs = make_move_pairs(game.moves());

        let mut move_text =
            pairs
                .into_iter()
                .enumerate()
                .fold(String::new(), |mut acc, (idx, (white, black))| {
                    // let black = black.unwrap_or(String::new());
                    // let mut move_string = format!("{}. {} {} ", idx + 1, white, black);
                    let mut move_string = format!("{}. {}", idx + 1, white);
                    if black.is_some() {
                        move_string.push_str(format!(" {} ", black.unwrap()).as_str())
                    }
                    acc.push_str(move_string.as_str());
                    acc
                });
        match game.result() {
            InProgress => move_text.push_str(" *"),
            _ => (),
        };

        Game {
            event: String::from(r#""""#),
            site: String::from(r#""""#),
            date: String::from(r#""""#),
            round: String::from(r#""""#),
            white: String::from(r#""""#),
            black: String::from(r#""""#),
            result: String::from(r#""""#),
            move_text,
        }
    }
    fn print_tag(&self, name: &str, value: &String) -> String {
        format!("[{} {}]", name, value)
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let tags = vec![
            self.print_tag("Event", &self.event),
            self.print_tag("Site", &self.site),
            self.print_tag("Date", &self.date),
            self.print_tag("Round", &self.round),
            self.print_tag("White", &self.white),
            self.print_tag("Black", &self.black),
            self.print_tag("Result", &self.result),
        ];
        let str = format!("{}\n\n{}", tags.join("\n"), self.move_text);
        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::CastlingRights;

    #[test]
    fn move_from_game() {
        /*
                [Event "Let\\'s Play!"]
        [Site "Chess.com"]
        [Date "2021.05.04"]
        [Round "?"]
        [White "horatiofox"]
        [Black "chessincheck"]
        [Result "*"]
        [ECO "B12"]
        [WhiteElo "750"]
        [BlackElo "800"]
        [TimeControl "1/604800"]

        1. e4 c6 2. d4 d5 3. e5 f6 4. Nf3 Nd7 5. Bf4 fxe5 6. Nxe5 Nxe5 7. Bxe5 Nh6 8.
        Qh5+ Nf7 9. f4 g6 10. Qh4 h6 11. Bxh8 Nxh8 12. Nd2 *
        **/
        let mut game = chess_game::new();
        let mut moves = Vec::new();
        // e4
        let w_move = Move::new(
            Coordinate::new(5, 2),
            Coordinate::new(5, 4),
            PieceType::Pawn,
            MoveType::Move,
            None,
            None,
            None,
        );
        // c6
        let b_move = Move::new(
            Coordinate::new(3, 7),
            Coordinate::new(3, 6),
            PieceType::Pawn,
            MoveType::Move,
            None,
            None,
            None,
        );
        moves.push((w_move, Some(b_move)));
        // d4
        let w_move = Move::new(
            Coordinate::new(4, 2),
            Coordinate::new(4, 4),
            PieceType::Pawn,
            MoveType::Move,
            None,
            None,
            None,
        );
        // d5
        let b_move = Move::new(
            Coordinate::new(4, 7),
            Coordinate::new(4, 5),
            PieceType::Pawn,
            MoveType::Move,
            None,
            None,
            None,
        );
        moves.push((w_move, Some(b_move)));

        // e5
        let w_move = Move::new(
            Coordinate::new(5, 4),
            Coordinate::new(5, 5),
            PieceType::Pawn,
            MoveType::Move,
            None,
            None,
            None,
        );
        // f6
        let b_move = Move::new(
            Coordinate::new(6, 7),
            Coordinate::new(6, 6),
            PieceType::Pawn,
            MoveType::Move,
            None,
            None,
            None,
        );
        moves.push((w_move, Some(b_move)));

        //nf3
        let w_move = Move::new(
            Coordinate::new(7, 1),
            Coordinate::new(6, 3),
            PieceType::Knight,
            MoveType::Move,
            None,
            None,
            None,
        );
        //nd7
        let b_move = Move::new(
            Coordinate::new(2, 8),
            Coordinate::new(4, 7),
            PieceType::Knight,
            MoveType::Move,
            None,
            None,
            None,
        );
        moves.push((w_move, Some(b_move)));
        //bf4
        let w_move = Move::new(
            Coordinate::new(3, 1),
            Coordinate::new(6, 4),
            PieceType::Bishop,
            MoveType::Move,
            None,
            None,
            None,
        );
        //fxe5
        let b_move = Move::new(
            Coordinate::new(6, 6),
            Coordinate::new(5, 5),
            PieceType::Pawn,
            MoveType::Move,
            Some(PieceType::Pawn),
            None,
            None,
        );
        moves.push((w_move, Some(b_move)));

        //Nxe5
        let w_move = Move::new(
            Coordinate::new(6, 3),
            Coordinate::new(5, 5),
            PieceType::Knight,
            MoveType::Move,
            Some(PieceType::Pawn),
            None,
            None,
        );
        //Nxe5
        let b_move = Move::new(
            Coordinate::new(4, 7),
            Coordinate::new(5, 5),
            PieceType::Knight,
            MoveType::Move,
            Some(PieceType::Knight),
            None,
            None,
        );
        moves.push((w_move, Some(b_move)));

        //Bxe5
        let w_move = Move::new(
            Coordinate::new(6, 4),
            Coordinate::new(5, 5),
            PieceType::Bishop,
            MoveType::Move,
            Some(PieceType::Pawn),
            None,
            None,
        );
        //Nh6
        let b_move = Move::new(
            Coordinate::new(7, 8),
            Coordinate::new(8, 6),
            PieceType::Knight,
            MoveType::Move,
            None,
            None,
            None,
        );
        moves.push((w_move, Some(b_move)));

        //Qh5+
        let mut w_move = Move::new(
            Coordinate::new(4, 1),
            Coordinate::new(8, 5),
            PieceType::Queen,
            MoveType::Move,
            None,
            None,
            None,
        );
        w_move.is_check = true;
        //Nf7
        let b_move = Move::new(
            Coordinate::new(8, 6),
            Coordinate::new(6, 7),
            PieceType::Knight,
            MoveType::Move,
            None,
            None,
            None,
        );
        moves.push((w_move, Some(b_move)));

        //f4
        let w_move = Move::new(
            Coordinate::new(6, 2),
            Coordinate::new(6, 4),
            PieceType::Pawn,
            MoveType::Move,
            None,
            None,
            None,
        );
        //g6
        let b_move = Move::new(
            Coordinate::new(7, 7),
            Coordinate::new(7, 6),
            PieceType::Pawn,
            MoveType::Move,
            None,
            None,
            None,
        );
        moves.push((w_move, Some(b_move)));

        //Qh4
        let w_move = Move::new(
            Coordinate::new(8, 5),
            Coordinate::new(8, 4),
            PieceType::Queen,
            MoveType::Move,
            None,
            None,
            None,
        );
        //h6
        let b_move = Move::new(
            Coordinate::new(8, 7),
            Coordinate::new(8, 6),
            PieceType::Pawn,
            MoveType::Move,
            None,
            None,
            None,
        );
        moves.push((w_move, Some(b_move)));

        //@todo: update Castling rights black kingside is removed now
        //Bxh8
        let w_move = Move::new(
            Coordinate::new(5, 5),
            Coordinate::new(8, 8),
            PieceType::Bishop,
            MoveType::Move,
            Some(PieceType::Rook),
            None,
            Some(CastlingRights::new(true, false)),
        );
        //Nxh8
        let b_move = Move::new(
            Coordinate::new(6, 7),
            Coordinate::new(8, 8),
            PieceType::Knight,
            MoveType::Move,
            Some(PieceType::Bishop),
            None,
            None,
        );
        moves.push((w_move, Some(b_move)));
        //Nd2
        let w_move = Move::new(
            Coordinate::new(2, 1),
            Coordinate::new(4, 2),
            PieceType::Knight,
            MoveType::Move,
            None,
            None,
            None,
        );
        moves.push((w_move, None));
        game.make_moves(moves);
        let pgn = Game::new_from_game(&game);
        let expected_move_text = String::from(
            "1. e4 c6 2. d4 d5 3. e5 f6 4. Nf3 Nd7 5. Bf4 fxe5 6. Nxe5 Nxe5 7. Bxe5 Nh6 8. Qh5+ Nf7 9. f4 g6 10. Qh4 h6 11. Bxh8 Nxh8 12. Nd2 *",
        );
        assert_eq!(
            pgn.move_text, expected_move_text,
            "Move text is correct for pgn."
        );
    }

    #[test]
    fn test_pgn() {
        let game = Game {
            event: String::from(r#""F/S Return Match""#),
            site: String::from(r#""Belgrade, Serbia JUG""#),
            date: String::from(r#""1992.11.04""#),
            round: String::from(r#""29""#),
            white: String::from(r#""Fischer, Robert J.""#),
            black: String::from(r#""Spassky, Boris V.""#),
            result: String::from(r#""1/2-1/2""#),
            move_text: String::from(
                "1. e4 e5 2. Nf3 Nc6 3. Bb5 a6 {This opening is called the Ruy Lopez.}
4. Ba4 Nf6 5. O-O Be7 6. Re1 b5 7. Bb3 d6 8. c3 O-O 9. h3 Nb8 10. d4 Nbd7
11. c4 c6 12. cxb5 axb5 13. Nc3 Bb7 14. Bg5 b4 15. Nb1 h6 16. Bh4 c5 17. dxe5
Nxe4 18. Bxe7 Qxe7 19. exd6 Qf6 20. Nbd2 Nxd6 21. Nc4 Nxc4 22. Bxc4 Nb6
23. Ne5 Rae8 24. Bxf7+ Rxf7 25. Nxf7 Rxe1+ 26. Qxe1 Kxf7 27. Qe3 Qg5 28. Qxg5
hxg5 29. b3 Ke6 30. a3 Kd6 31. axb4 cxb4 32. Ra5 Nd5 33. f3 Bc8 34. Kf2 Bf5
35. Ra7 g6 36. Ra6+ Kc5 37. Ke1 Nf4 38. g3 Nxh3 39. Kd2 Kb5 40. Rd6 Kc5 41. Ra6
Nf2 42. g4 Bd3 43. Re6 1/2-1/2",
            ),
        };
        let str = String::from(TEST_PGN);
        assert_eq!(str.to_string(), format!("{}", game).to_string())
    }
}
