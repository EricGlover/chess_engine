// works with bitboards
use crate::bit_board::{
    self, BitBoard, A_FILE, B_FILE, G_FILE, H_FILE, ROW_1, ROW_2, ROW_7, ROW_8,
};
use crate::board::{Color, Coordinate, Piece, PieceType};
use crate::board::{HIGH_X, HIGH_Y, LOW_X, LOW_Y};
use crate::move_generator::Move;

// array of bitboards with the attack from the corresponding index
const BLACK_PAWN_ATTACKS: [u64; 64] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    2,
    5,
    10,
    20,
    40,
    80,
    160,
    64,
    512,
    1280,
    2560,
    5120,
    10240,
    20480,
    40960,
    16384,
    131072,
    327680,
    655360,
    1310720,
    2621440,
    5242880,
    10485760,
    4194304,
    33554432,
    83886080,
    167772160,
    335544320,
    671088640,
    1342177280,
    2684354560,
    1073741824,
    8589934592,
    21474836480,
    42949672960,
    85899345920,
    171798691840,
    343597383680,
    687194767360,
    274877906944,
    2199023255552,
    5497558138880,
    10995116277760,
    21990232555520,
    43980465111040,
    87960930222080,
    175921860444160,
    70368744177664,
    562949953421312,
    1407374883553280,
    2814749767106560,
    5629499534213120,
    11258999068426240,
    22517998136852480,
    45035996273704960,
    18014398509481984,
];

const WHITE_PAWN_ATTACKS: [u64; 64] = [
    512,
    1280,
    2560,
    5120,
    10240,
    20480,
    40960,
    16384,
    131072,
    327680,
    655360,
    1310720,
    2621440,
    5242880,
    10485760,
    4194304,
    33554432,
    83886080,
    167772160,
    335544320,
    671088640,
    1342177280,
    2684354560,
    1073741824,
    8589934592,
    21474836480,
    42949672960,
    85899345920,
    171798691840,
    343597383680,
    687194767360,
    274877906944,
    2199023255552,
    5497558138880,
    10995116277760,
    21990232555520,
    43980465111040,
    87960930222080,
    175921860444160,
    70368744177664,
    562949953421312,
    1407374883553280,
    2814749767106560,
    5629499534213120,
    11258999068426240,
    22517998136852480,
    45035996273704960,
    18014398509481984,
    144115188075855872,
    360287970189639680,
    720575940379279360,
    1441151880758558720,
    2882303761517117440,
    5764607523034234880,
    11529215046068469760,
    4611686018427387904,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];

// array of bitboards, for knight attacks
const KNIGHT_ATTACKS: [u64; 64] = [
    132096,
    329728,
    659712,
    1319424,
    2638848,
    5277696,
    10489856,
    4202496,
    33816580,
    84410376,
    168886289,
    337772578,
    675545156,
    1351090312,
    2685403152,
    1075839008,
    8657044482,
    21609056261,
    43234889994,
    86469779988,
    172939559976,
    345879119952,
    687463207072,
    275414786112,
    2216203387392,
    5531918402816,
    11068131838464,
    22136263676928,
    44272527353856,
    88545054707712,
    175990581010432,
    70506185244672,
    567348067172352,
    1416171111120896,
    2833441750646784,
    5666883501293568,
    11333767002587136,
    22667534005174272,
    45053588738670592,
    18049583422636032,
    145241105196122112,
    362539804446949376,
    725361088165576704,
    1450722176331153408,
    2901444352662306816,
    5802888705324613632,
    11533718717099671552,
    4620693356194824192,
    288234782788157440,
    576469569871282176,
    1224997833292120064,
    2449995666584240128,
    4899991333168480256,
    9799982666336960512,
    1152939783987658752,
    2305878468463689728,
    1128098930098176,
    2257297371824128,
    4796069720358912,
    9592139440717824,
    19184278881435648,
    38368557762871296,
    4679521487814656,
    9077567998918656,
];
// array of bitboards, for king attacks
const KING_ATTACKS: [u64; 64] = [
    770,
    1797,
    3594,
    7188,
    14376,
    28752,
    57504,
    49216,
    197123,
    460039,
    920078,
    1840156,
    3680312,
    7360624,
    14721248,
    12599488,
    50463488,
    117769984,
    235539968,
    471079936,
    942159872,
    1884319744,
    3768639488,
    3225468928,
    12918652928,
    30149115904,
    60298231808,
    120596463616,
    241192927232,
    482385854464,
    964771708928,
    825720045568,
    3307175149568,
    7718173671424,
    15436347342848,
    30872694685696,
    61745389371392,
    123490778742784,
    246981557485568,
    211384331665408,
    846636838289408,
    1975852459884544,
    3951704919769088,
    7903409839538176,
    15806819679076352,
    31613639358152704,
    63227278716305408,
    54114388906344448,
    216739030602088448,
    505818229730443264,
    1011636459460886528,
    2023272918921773056,
    4046545837843546112,
    8093091675687092224,
    16186183351374184448,
    13853283560024178688,
    144959613005987840,
    362258295026614272,
    724516590053228544,
    1449033180106457088,
    2898066360212914176,
    5796132720425828352,
    11592265440851656704,
    4665729213955833856,
];

const ROOK_ATTACKS: [u64; 64] = [
    72340172838076926,
    14468034567615359,
    28936069135230693,
    57872138270461362,
    11574427654092269,
    23148855308184537,
    46297710616369071,
    92595421232738141,
    72340172838141441,
    14468034567621760,
    28936069135236992,
    57872138270467456,
    11574427654092838,
    23148855308185024,
    46297710616369395,
    92595421232738138,
    72340172854657281,
    14468034569260288,
    28936069136849408,
    57872138272027648,
    11574427654238412,
    23148855308309709,
    46297710616452301,
    92595421232737486,
    72340177082712321,
    14468034988723456,
    28936069549627904,
    57872138671436800,
    11574427691505459,
    23148855340229017,
    46297710637676135,
    92595421232570369,
    72341259464802561,
    14468142371294464,
    28936175220922880,
    57872240920179712,
    11574437231869337,
    23148863511572070,
    46297716070977536,
    92595421189788468,
    72618349279904001,
    14495632309472512,
    28963227072436736,
    57898416598365184,
    11576879565022208,
    23150955375393587,
    46299106996136346,
    92595410237621863,
    14355334194587264,
    21533056483052800,
    35888501059983872,
    64599390213846016,
    12202116852157030,
    23686472513701888,
    46655183836791603,
    92592606482971034,
    18302911464433844,
    18231136449196065,
    18087586418720506,
    17800486357769390,
    17226286235867156,
    16077885992062689,
    13781085504453754,
    91874845292358862,
];

pub fn test() {
    init_gen_rook_attacks();
    // init_gen_king_attacks();
    // init_gen_knight_attacks();
    // init_gen_pawn_attacks();
}

fn init_gen_queen_attacks() {
    // @todo:
}

fn init_gen_rook_attacks() {
    let mut rook_attacks: [u64; 64] = [0; 64];
    for (idx, bit_board) in rook_attacks.iter_mut().enumerate() {
        let i = (idx + 1) as u64;
        let start_bit = BitBoard::set_bit(0, i);
        // find file
        let file = BitBoard::get_file_for_bit(start_bit);
        // find row
        let row = BitBoard::get_row_for_bit(start_bit);
        *bit_board = file ^ row;
    }
    // print
    for (idx, bit_board) in rook_attacks.iter().enumerate() {
        println!("{bit_board} {idx}");
        BitBoard::print_bitboard(*bit_board);
    }
    for bit in rook_attacks.iter() {
        println!("{}", bit);
    }
}

fn init_gen_bishop_attacks() {
    // @todo: do the same for the bishop as you did the rooks
}

fn init_gen_king_attacks() {
    let mut king_attacks: [u64; 64] = [0; 64];
    let mut start_bit = BitBoard::set_bit(0u64, 19); //(3,3), (c,3)
    BitBoard::print_bitboard(start_bit);

    let mut bit_board = 0u64;
    //ups
    bit_board = bit_board | start_bit << 7;
    bit_board = bit_board | start_bit << 8;
    bit_board = bit_board | start_bit << 9;
    //right
    bit_board = bit_board | start_bit << 1;
    //downs
    bit_board = bit_board | start_bit >> 7;
    bit_board = bit_board | start_bit >> 8;
    bit_board = bit_board | start_bit >> 9;
    //left
    bit_board = bit_board | start_bit >> 1;
    BitBoard::print_bitboard(bit_board);

    for (idx, bit_board) in king_attacks.iter_mut().enumerate() {
        let mut res: u64 = 0;
        let mut start_bit = BitBoard::set_bit(0u64, (idx + 1) as u64);
        // up moves
        // UP 1 LEFT 1
        if !BitBoard::bit_on_bit_board(start_bit, ROW_8 | A_FILE) {
            res = res | start_bit << 7;
        }
        // UP 1
        if !BitBoard::bit_on_bit_board(start_bit, ROW_8) {
            res = res | start_bit << 8;
        }
        // UP 1 RIGHT 1
        if !BitBoard::bit_on_bit_board(start_bit, ROW_8 | H_FILE) {
            res = res | start_bit << 9;
        }
        // right
        if !BitBoard::bit_on_bit_board(start_bit, H_FILE) {
            res = res | start_bit << 1;
        }
        // DOWN MOVES
        // DOWN 1 RIGHT 1
        if !BitBoard::bit_on_bit_board(start_bit, ROW_1 | H_FILE) {
            res = res | start_bit >> 7;
        }
        // DOWN 1
        if !BitBoard::bit_on_bit_board(start_bit, ROW_1) {
            res = res | start_bit >> 8;
        }
        // DOWN 1 LEFT 1
        if !BitBoard::bit_on_bit_board(start_bit, ROW_1 | A_FILE) {
            res = res | start_bit >> 9;
        }

        // LEFT
        if !BitBoard::bit_on_bit_board(start_bit, A_FILE) {
            res = res | start_bit >> 1;
        }

        *bit_board = res;
    }

    // print
    for (idx, bit_board) in king_attacks.iter().enumerate() {
        println!("{bit_board} {idx}");
        BitBoard::print_bitboard(*bit_board);
    }
    for bit in king_attacks.iter() {
        println!("{}", bit);
    }
}

fn init_gen_knight_attacks() {
    let mut knight_attacks: [u64; 64] = [0; 64];
    let mut start_bit = BitBoard::set_bit(0u64, 19); //(3,3), (c,3)
    BitBoard::print_bitboard(start_bit);

    let mut bit_board = 0u64;
    //down 1, right 2
    // (f,2), not g file, not h file, not row 1
    bit_board = bit_board | start_bit >> 6;
    //down 1, left 2
    // (c, 2) not a file, not b file, not row 1
    bit_board = bit_board | start_bit >> 10;
    //down 2, right 1
    // (g, 3), not h file, not row 1, not row 2
    bit_board = bit_board | start_bit >> 15;
    //down 2, left 1
    //not A file, not row 1, not row 2
    bit_board = bit_board | start_bit >> 17;

    // up 1, left 2
    //not on file A, file B, or row 8
    bit_board = bit_board | start_bit << 6;
    // up 1, right 2
    //not on file G, file H, or row 8
    bit_board = bit_board | start_bit << 10;
    //up 2, left 1
    //not on row 7, row 8, or file A
    bit_board = bit_board | start_bit << 15;
    //up 2, right 1
    //not on row 7, row 8, or file H
    bit_board = bit_board | start_bit << 17;

    BitBoard::print_bitboard(bit_board);

    for (idx, bit_board) in knight_attacks.iter_mut().enumerate() {
        // base case : all moves
        // c3 - c6, d3-d6, e3-e6, f3-f6
        //todo : gen a mask for this
        let mut start_bit = BitBoard::set_bit(0u64, (idx + 1) as u64);

        let mut res = 0u64;

        // DOWN MOVES
        if !BitBoard::bit_on_bit_board(start_bit, G_FILE | H_FILE | ROW_1) {
            res = res | start_bit >> 6;
        }
        if !BitBoard::bit_on_bit_board(start_bit, A_FILE | B_FILE | ROW_1) {
            res = res | start_bit >> 10;
        }

        if !BitBoard::bit_on_bit_board(start_bit, H_FILE | ROW_1 | ROW_2) {
            res = res | start_bit >> 15;
        }

        if !BitBoard::bit_on_bit_board(start_bit, A_FILE | ROW_1 | ROW_2) {
            res = res | start_bit >> 17;
        }

        // UP MOVES
        if !BitBoard::bit_on_bit_board(start_bit, A_FILE | B_FILE | ROW_8) {
            res = res | start_bit << 6;
        }
        if !BitBoard::bit_on_bit_board(start_bit, G_FILE | H_FILE | ROW_8) {
            res = res | start_bit << 10;
        }
        if !BitBoard::bit_on_bit_board(start_bit, ROW_7 | ROW_8 | A_FILE) {
            res = res | start_bit << 15;
        }
        if !BitBoard::bit_on_bit_board(start_bit, ROW_7 | ROW_8 | H_FILE) {
            res = res | start_bit << 17;
        }

        *bit_board = res;
    }
    //printing results
    for (idx, bit_board) in knight_attacks.iter().enumerate() {
        println!("{bit_board} {idx}");
        BitBoard::print_bitboard(*bit_board);
    }
    for bit in knight_attacks.iter() {
        println!("{}", bit);
    }
}

fn init_gen_pawn_attacks() {
    let mut wpa: [u64; 64] = [0; 64];
    let mut bpa: [u64; 64] = [0; 64];

    // interesting rust tid bit
    // for(x, idx ) in wpa.iter_mut().zip((0..).into_iter()) {
    //     println!("{x} {idx}");
    // }
    // WHITE PAWN ATTACKS
    let gen_white = false;
    if gen_white {
        for (idx, bit_board) in wpa.iter_mut().enumerate() {
            let mut start_bit = BitBoard::set_bit(0u64, (idx + 1) as u64);
            if BitBoard::on_row(start_bit, ROW_8) {
                continue;
            }
            let on_a_file = BitBoard::on_file(start_bit, A_FILE);
            let on_h_file = BitBoard::on_file(start_bit, H_FILE);

            let mut res = 0u64;
            if !on_a_file {
                res = start_bit << 7;
            }
            if !on_h_file {
                res = res | start_bit << 9;
            }
            *bit_board = res;
            println!("{bit_board} {idx}, {} {}", on_a_file, on_h_file);
        }

        //printing results
        for (idx, bit_board) in wpa.iter().enumerate() {
            println!("{bit_board} {idx}");
            BitBoard::print_bitboard(*bit_board);
        }
        for idx in wpa.iter() {
            println!("{}", idx);
        }
    }
    let mut b = BitBoard::set_bit(0u64, 1);
    println!("{}", 1);
    BitBoard::print_bitboard(b >> 7);

    b = BitBoard::set_bit(0u64, 2);
    println!("{}", 2);
    BitBoard::print_bitboard(b >> 7);

    b = BitBoard::set_bit(0u64, 7);
    println!("===================");
    BitBoard::print_bitboard(b);
    println!("{}", 7);
    BitBoard::print_bitboard(b >> 7);

    b = BitBoard::set_bit(0u64, 8);
    println!("===================");
    BitBoard::print_bitboard(b);
    println!("{}", 8);
    BitBoard::print_bitboard(b >> 7);

    // b = BitBoard::set_bit(0u64, 9);
    // println!("{1}");
    // BitBoard::print_bitboard(b >> 7);

    // b = BitBoard::set_bit(0u64, 15);
    // println!("{1}");
    // BitBoard::print_bitboard(b >> 7);

    //BLACK PAWN ATTACKS
    for (idx, bit_board) in bpa.iter_mut().enumerate() {
        let mut start_bit = BitBoard::set_bit(0u64, (idx + 1) as u64);

        let on_a_file = BitBoard::on_file(start_bit, A_FILE);
        let on_h_file = BitBoard::on_file(start_bit, H_FILE);

        if BitBoard::on_row(start_bit, ROW_1) {
            continue;
        }

        let mut res = 0u64;
        if !on_a_file {
            res = start_bit >> 9;
        }
        if !on_h_file {
            res = res | start_bit >> 7;
        }
        *bit_board = res;
        println!("{bit_board} {idx}, {} {}", on_a_file, on_h_file);
    }

    //printing results
    for (idx, bit_board) in bpa.iter().enumerate() {
        println!("{bit_board} {idx}");
        BitBoard::print_bitboard(*bit_board);
    }
    for idx in bpa.iter() {
        println!("{}", idx);
    }
}

/**
 * @todo
one square move, two square move, capturing diagonally forward, pawn promotion, en passant
**/
pub fn gen_pawn_moves(board: &BitBoard, piece: &Piece) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    return moves;
}

/** HELPER FUNCTIONS  */
fn square_is_empty(board: &BitBoard, at: &Coordinate) -> bool {
    board.is_piece_at_coordinate(at)
}

// if square is off board || square has friendly price => false
fn square_occupiable_by(board: &BitBoard, at: &Coordinate, color: Color) -> bool {
    if !is_on_board(at) {
        return false;
    }
    board.get_piece_at(at).map_or(true, |p| p.color != color)
}

fn has_enemy_piece(board: &BitBoard, at: &Coordinate, own_color: Color) -> bool {
    if !is_on_board(at) {
        return false;
    }
    board
        .get_piece_at(at)
        .map_or(false, |piece| piece.color == own_color.opposite())
}

fn is_on_board(c: &Coordinate) -> bool {
    c.x() >= LOW_X && c.x() <= HIGH_X && c.y() >= LOW_Y && c.y() <= HIGH_Y
}
