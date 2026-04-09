// works with bitboards
use crate::bit_board::{
    self, BitBoard, A_FILE, BLACK_KINGSIDE_CASTLE_BLOCKERS, BLACK_QUEENSIDE_CASTLE_BLOCKERS,
    B_FILE, DARK_DIAGONALS_UP_RIGHT, G_FILE, H_FILE, ROW_1, ROW_2, ROW_7, ROW_8,
    WHITE_KINGSIDE_CASTLE_BLOCKERS, WHITE_QUEENSIDE_CASTLE_BLOCKERS,
};
use crate::board::{Color, Coordinate, Piece, PieceType};
use crate::board::{HIGH_X, HIGH_Y, LOW_X, LOW_Y};
use crate::game_state::GameState;
use crate::move_generator::{Move, MoveType};

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
    144680345676153597,
    289360691352306939,
    578721382704613623,
    1157442765409226991,
    2314885530818453727,
    4629771061636907199,
    9259542123273814143,
    72340172838141441,
    144680345676217602,
    289360691352369924,
    578721382704674568,
    1157442765409283856,
    2314885530818502432,
    4629771061636939584,
    9259542123273813888,
    72340172854657281,
    144680345692602882,
    289360691368494084,
    578721382720276488,
    1157442765423841296,
    2314885530830970912,
    4629771061645230144,
    9259542123273748608,
    72340177082712321,
    144680349887234562,
    289360695496279044,
    578721386714368008,
    1157442769150545936,
    2314885534022901792,
    4629771063767613504,
    9259542123257036928,
    72341259464802561,
    144681423712944642,
    289361752209228804,
    578722409201797128,
    1157443723186933776,
    2314886351157207072,
    4629771607097753664,
    9259542118978846848,
    72618349279904001,
    144956323094725122,
    289632270724367364,
    578984165983651848,
    1157687956502220816,
    2315095537539358752,
    4629910699613634624,
    9259541023762186368,
    143553341945872641,
    215330564830528002,
    358885010599838724,
    645993902138460168,
    1220211685215703056,
    2368647251370188832,
    4665518383679160384,
    9259260648297103488,
    18302911464433844481,
    18231136449196065282,
    18087586418720506884,
    17800486357769390088,
    17226286235867156496,
    16077885992062689312,
    13781085504453754944,
    9187484529235886208,
];

const BISHOP_ATTACKS: [u64; 64] = [
    9241421688590303744,
    36099303471056128,
    141012904249856,
    550848566272,
    6480472064,
    1108177604608,
    283691315142656,
    72624976668147712,
    4620710844295151618,
    9241421688590368773,
    36099303487963146,
    141017232965652,
    1659000848424,
    283693466779728,
    72624976676520096,
    145249953336262720,
    2310355422147510788,
    4620710844311799048,
    9241421692918565393,
    36100411639206946,
    424704217196612,
    72625527495610504,
    145249955479592976,
    290499906664153120,
    1155177711057110024,
    2310355426409252880,
    4620711952330133792,
    9241705379636978241,
    108724279602332802,
    145390965166737412,
    290500455356698632,
    580999811184992272,
    577588851267340304,
    1155178802063085600,
    2310639079102947392,
    4693335752243822976,
    9386671504487645697,
    326598935265674242,
    581140276476643332,
    1161999073681608712,
    288793334762704928,
    577868148797087808,
    1227793891648880768,
    2455587783297826816,
    4911175566595588352,
    9822351133174399489,
    1197958188344280066,
    2323857683139004420,
    144117404414255168,
    360293502378066048,
    720587009051099136,
    1441174018118909952,
    2882348036221108224,
    5764696068147249408,
    11529391036782871041,
    4611756524879479810,
    567382630219904,
    1416240237150208,
    2833579985862656,
    5667164249915392,
    11334324221640704,
    22667548931719168,
    45053622886727936,
    18049651735527937,
];

const QUEEN_ATTACK: [u64; 64] = [
    9313761861428380670,
    180779649147209725,
    289501704256556795,
    578721933553179895,
    1157442771889699055,
    2314886638996058335,
    4630054752952049855,
    9332167099941961855,
    4693051017133293059,
    9386102034266586375,
    325459994840333070,
    578862399937640220,
    1157444424410132280,
    2315169224285282160,
    4702396038313459680,
    9404792076610076608,
    2382695595002168069,
    4765391190004401930,
    9530782384287059477,
    614821794359483434,
    1157867469641037908,
    2387511058326581416,
    4775021017124823120,
    9550042029937901728,
    1227517888139822345,
    2455035776296487442,
    4910072647826412836,
    9820426766351346249,
    1266167048752878738,
    2460276499189639204,
    4920271519124312136,
    9840541934442029200,
    649930110732142865,
    1299860225776030242,
    2600000831312176196,
    5272058161445620104,
    10544115227674579473,
    2641485286422881314,
    5210911883574396996,
    10421541192660455560,
    361411684042608929,
    722824471891812930,
    1517426162373248132,
    3034571949281478664,
    6068863523097809168,
    12137446670713758241,
    5827868887957914690,
    11583398706901190788,
    287670746360127809,
    575624067208594050,
    1079472019650937860,
    2087167920257370120,
    4102559721436811280,
    8133343319517438240,
    16194909420462031425,
    13871017173176583298,
    18303478847064064385,
    18232552689433215490,
    18090419998706369540,
    17806153522019305480,
    17237620560088797200,
    16100553540994408480,
    13826139127340482880,
    9205534180971414145,
];




pub fn test() {
    // init_gen_queen_attacks();
    // init_gen_bishop_attacks();
    // init_gen_rook_attacks();
    // init_gen_king_attacks();
    // init_gen_knight_attacks();
    // init_gen_pawn_attacks();
}

fn init_gen_queen_attacks() {
    let mut queen_attacks: [u64; 64] = [0; 64];
    for (idx, bit_board) in queen_attacks.iter_mut().enumerate() {
        let i = (idx + 1) as u64;
        let start_bit = BitBoard::set_bit(0, i);
        *bit_board = ROOK_ATTACKS[idx] | BISHOP_ATTACKS[idx];
    }
    // print
    for (idx, bit_board) in queen_attacks.iter().enumerate() {
        println!("{bit_board} {idx}");
        BitBoard::print_bitboard(*bit_board);
    }
    for bit in queen_attacks.iter() {
        println!("{}", bit);
    }
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
    let mut bishop_attacks: [u64; 64] = [0; 64];
    for (idx, bit_board) in bishop_attacks.iter_mut().enumerate() {
        let i = (idx + 1) as u64;
        let start_bit = BitBoard::set_bit(0, i);
        let color = BitBoard::get_square_color(start_bit);
        let diagonals = BitBoard::get_diagonals_for_bit(start_bit);

        *bit_board = diagonals ^ start_bit;
    }
    // print
    for (idx, bit_board) in bishop_attacks.iter().enumerate() {
        println!("{bit_board} {idx}");
        BitBoard::print_bitboard(*bit_board);
    }
    for bit in bishop_attacks.iter() {
        println!("{}", bit);
    }
}

fn init_gen_king_attacks() {
    let mut king_attacks: [u64; 64] = [0; 64];
    let start_bit = BitBoard::set_bit(0u64, 19); //(3,3), (c,3)
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
        let start_bit = BitBoard::set_bit(0u64, (idx + 1) as u64);
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
    let start_bit = BitBoard::set_bit(0u64, 19); //(3,3), (c,3)
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
        let start_bit = BitBoard::set_bit(0u64, (idx + 1) as u64);

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
            let start_bit = BitBoard::set_bit(0u64, (idx + 1) as u64);
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
        let start_bit = BitBoard::set_bit(0u64, (idx + 1) as u64);

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

pub fn gen_moves_for(game_state: &GameState, piece: &Piece) -> Vec<Move> {
    let moves = match piece.piece_type {
        PieceType::King => gen_king_moves(piece, game_state),
        PieceType::Queen => gen_queen_moves(piece, game_state),
        PieceType::Bishop => gen_bishop_moves(piece, game_state),
        PieceType::Knight => gen_knight_moves(piece, game_state),
        PieceType::Rook => gen_rook_moves(piece, game_state),
        PieceType::Pawn => gen_pawn_moves(piece, game_state),
    };
    return moves;
}

/** These are X-Rays, they ignore enemy blocking pieces */
pub fn gen_vectors_for(game_state: &GameState, piece: &Piece) -> Vec<Move> {
    let moves = match piece.piece_type {
        PieceType::King => gen_king_moves(piece, game_state),
        PieceType::Queen => gen_queen_vector(game_state, piece),
        PieceType::Bishop => gen_bishop_vector(game_state, piece),
        PieceType::Knight => gen_knight_moves(piece, game_state),
        PieceType::Rook => gen_rook_vector(game_state, piece),
        PieceType::Pawn => gen_pawn_moves(piece, game_state),
    };
    return moves;
}

pub fn gen_queen_vector(game_state: &GameState, piece: &Piece) -> Vec<Move> {
    let mut all_moves = gen_rook_vector(game_state, piece);
    let bishop_moves = gen_bishop_vector(game_state, piece);
    for m in bishop_moves {
        all_moves.push(m);
    }
    return all_moves;
}

pub fn gen_bishop_vector(game_state: &GameState, piece: &Piece) -> Vec<Move> {
    let board = game_state.get_board();
    let at = piece.at().unwrap();
    let idx = BitBoard::coordinate_to_idx(*at);
    let start_bit = BitBoard::coordinate_to_bit(*at);
    let enemy_bits = match piece.color {
        Color::White => board.get_black_pieces_board(),
        Color::Black => board.get_white_pieces_board(),
    };
    let friendly_bits = match piece.color {
        Color::White => board.get_white_pieces_board(),
        Color::Black => board.get_black_pieces_board(),
    };
    
    let less_board = start_bit - 1;
    let mut to_move_board: u64 = 0;
    let mut captures_board: u64 = 0;
    // length for diagonals == 1 || 2
    let diagonals = BitBoard::get_diagonals_vec_for_bit(start_bit);

    let is_a_file = BitBoard::on_file(start_bit, A_FILE);
    let is_h_file = BitBoard::on_file(start_bit, H_FILE);

    // check files or this stuff will wrap around, if the direction is off the board
    // just set it to 0
    let up_right_bit = match is_h_file {
        true => 0,
        false => start_bit << 9,
    };
    let up_left_bit = match is_a_file {
        true => 0,
        false => start_bit << 7,
    };
    let down_right_bit = match is_h_file {
        true => 0,
        false => start_bit >> 7,
    };
    let down_left_bit = match is_a_file {
        true => 0,
        false => start_bit >> 9,
    };
    for diagonal in diagonals {
        /* UP RIGHT DIAGONAL */
        if BitBoard::bit_on_bit_board(up_right_bit, diagonal)
            || BitBoard::bit_on_bit_board(down_left_bit, diagonal)
        {
            // cut diagonal in half, top / bottom
            let up_path = ((diagonal ^ less_board) & diagonal) ^ start_bit;
            let down_path = (diagonal ^ up_path) ^ start_bit;
            if up_path > 0 {
                let occupied = up_path & friendly_bits;
                // get nearest
                let nearest = BitBoard::lsb(occupied);
                if nearest == 0 {
                    // add all
                    to_move_board = to_move_board | up_path;
                } else {
                    let is_enemy = BitBoard::bit_on_bit_board(nearest, enemy_bits);
                    let mut below_nearest = (nearest - 1) & up_path;
                    if is_enemy {
                        below_nearest = ((nearest - 1) & up_path) | nearest;
                        captures_board = captures_board | nearest;
                    }
                    BitBoard::print_bitboard(below_nearest);
                    to_move_board = to_move_board | below_nearest;
                }
            }
            if down_path > 0 {
                let occupied = down_path & friendly_bits;
                // get nearest
                let nearest = BitBoard::msb(occupied);
                if nearest == 0 {
                    // add all
                    to_move_board = to_move_board | down_path;
                } else {
                    let is_enemy = BitBoard::bit_on_bit_board(nearest, enemy_bits);
                    let mut above_nearest = (!(nearest - 1) & down_path) ^ nearest;
                    if is_enemy {
                        above_nearest = !(nearest - 1) & down_path;
                        captures_board = captures_board | nearest;
                    }
                    to_move_board = to_move_board | above_nearest;
                }
            }
        }

        /* UP LEFT DIAGONAL */
        if BitBoard::bit_on_bit_board(up_left_bit, diagonal)
            || BitBoard::bit_on_bit_board(down_right_bit, diagonal)
        {
            // cut diagonal in half, top / bottom
            let up_path = ((diagonal ^ less_board) & diagonal) ^ start_bit;
            let down_path = (diagonal ^ up_path) ^ start_bit;
            if up_path > 0 {
                let occupied = up_path & friendly_bits;
                // get nearest
                let nearest = BitBoard::lsb(occupied);
                if nearest == 0 {
                    // add all
                    to_move_board = to_move_board | up_path;
                } else {
                    let is_enemy = BitBoard::bit_on_bit_board(nearest, enemy_bits);
                    let mut below_nearest = (nearest - 1) & up_path;
                    if is_enemy {
                        below_nearest = ((nearest - 1) & up_path) | nearest;
                        captures_board = captures_board | nearest;
                    }
                    to_move_board = to_move_board | below_nearest;
                }
            }
            if down_path > 0 {
                let occupied = down_path & friendly_bits;
                // get nearest
                let nearest = BitBoard::msb(occupied);
                if nearest == 0 {
                    // add all
                    to_move_board = to_move_board | down_path;
                } else {
                    let is_enemy = BitBoard::bit_on_bit_board(nearest, enemy_bits);
                    let mut above_nearest = (!(nearest - 1) & down_path) ^ nearest;
                    if is_enemy {
                        above_nearest = !(nearest - 1) & down_path;
                        captures_board = captures_board | nearest;
                    }
                    to_move_board = to_move_board | above_nearest;
                }
            }
        }
    }
    let mut moves: Vec<Move> = vec![];
    let color = piece.color;

    while to_move_board > 0 {
        let to_bit = BitBoard::pop_bit(&mut to_move_board);
        let to = BitBoard::bit_to_coordinate(to_bit);

        if BitBoard::bit_on_bit_board(to_bit, captures_board) {
            moves.push(make_move_to(
                at,
                &to,
                piece,
                MoveType::Move,
                &board,
                game_state,
            ));
        } else {
            moves.push(make_quiet_move_with_castle_rights(
                at, &to, piece, game_state,
            ));
        }
    }

    return moves;
}

pub fn gen_queen_moves(piece: &Piece, game_state: &GameState) -> Vec<Move> {
    let mut all_moves = gen_rook_moves(piece, game_state);
    let bishop_moves = gen_bishop_moves(piece, game_state);
    for m in bishop_moves {
        all_moves.push(m);
    }
    return all_moves;
}


pub fn gen_rook_vector(game_state: &GameState, piece: &Piece) -> Vec<Move> {
    let board = game_state.get_board();
    let at = piece.at().unwrap();
    let idx: u64 = BitBoard::coordinate_to_idx(*at);

    //plan get the ray, remove this piece, check for nearest other piece
    //going up or right the nearest piece is the lsb
    let start_bit = BitBoard::coordinate_to_bit(*at);
    let enemy_bits = match piece.color {
        Color::White => board.get_black_pieces_board(),
        Color::Black => board.get_white_pieces_board(),
    };
    let friendly_bits = match piece.color {
        Color::White => board.get_white_pieces_board(),
        Color::Black => board.get_black_pieces_board(),
    };
    let less_board = start_bit - 1;
    let mut to_move_board: u64 = 0;
    let mut captures_board: u64 = 0;
    // break up the file into up/below sections
    //
    let file = BitBoard::get_file_for_bit(start_bit);
    let above_me_file = (file ^ (less_board | start_bit)) & file;
    let below_me_file = (file ^ above_me_file) ^ start_bit;
    // above_me_file == 0 then there's no where up to go
    if above_me_file != 0 {
        let occuppied = above_me_file & friendly_bits;
        let nearest = BitBoard::lsb(occuppied);
        if nearest == 0 {
            to_move_board = to_move_board | above_me_file;
        } else {
            // if the nearest piece is an enemy we include it
            let is_enemy = BitBoard::bit_on_bit_board(nearest, enemy_bits);
            // now get everything below this on the file
            let mut below_nearest = (nearest - 1) & above_me_file;
            if is_enemy {
                below_nearest = ((nearest - 1) | nearest) & above_me_file;
                captures_board = captures_board | nearest;
            }
            to_move_board = to_move_board | below_nearest;
        }
    }
    // below_me_file == 0 then there's no where down to go
    if below_me_file != 0 {
        let occuppied = below_me_file & friendly_bits;
        let nearest = BitBoard::msb(occuppied);
        if nearest == 0 {
            to_move_board = to_move_board | below_me_file;
        } else {
            // if the nearest piece is an enemy we include it
            let is_enemy = BitBoard::bit_on_bit_board(nearest, enemy_bits);
            let mut above_nearest = (!(nearest - 1) & below_me_file) ^ nearest;
            if is_enemy {
                above_nearest = !(nearest - 1) & below_me_file;
                captures_board = captures_board | nearest;
            }
            to_move_board = to_move_board | above_nearest;
        }
    }

    // break up the row into left/right sections
    let row = BitBoard::get_row_for_bit(start_bit);
    let left_row = less_board & row;
    let right_row = (row ^ left_row) ^ start_bit;
    //left row == 0 then there's nowhere left
    if left_row != 0 {
        let occupied = left_row & friendly_bits;
        let nearest = BitBoard::msb(occupied);
        if nearest == 0 {
            to_move_board = to_move_board | left_row;
        } else {
            // if the nearest piece is an enemy we include it
            let is_enemy = BitBoard::bit_on_bit_board(nearest, enemy_bits);
            let mut right_of_nearest = (!(nearest - 1) & left_row) ^ nearest;
            if is_enemy {
                right_of_nearest = !(nearest - 1) & left_row;
                captures_board = captures_board | nearest;
            }
            to_move_board = to_move_board | right_of_nearest;
        }
    }
    // right row == 0 then there's nowhere right
    if right_row != 0 {
        let occupied = right_row & friendly_bits;
        let nearest = BitBoard::lsb(occupied);
        if nearest == 0 {
            to_move_board = to_move_board | right_row;
        } else {
            // if the nearest piece is an enemy we include it
            let is_enemy = BitBoard::bit_on_bit_board(nearest, enemy_bits);
            let mut left_of_nearest = (nearest - 1) & right_row;
            if is_enemy {
                left_of_nearest = ((nearest - 1) | nearest) & right_row;
                captures_board = captures_board | nearest;
            }
            to_move_board = to_move_board | left_of_nearest;
        }
    }
    let mut moves: Vec<Move> = vec![];
    let color = piece.color;

    while to_move_board > 0 {
        let to_bit = BitBoard::pop_bit(&mut to_move_board);
        let to = BitBoard::bit_to_coordinate(to_bit);

        if BitBoard::bit_on_bit_board(to_bit, captures_board) {
            moves.push(make_move_to(
                at,
                &to,
                piece,
                MoveType::Move,
                &board,
                game_state,
            ));
        } else {
            moves.push(make_quiet_move_with_castle_rights(
                at, &to, piece, game_state,
            ));
        }
    }

    return moves;
}


pub fn gen_bishop_moves(piece: &Piece, game_state: &GameState) -> Vec<Move> {
    let board = game_state.get_board();
    let at = piece.at().unwrap();
    let idx = BitBoard::coordinate_to_idx(*at);
    let start_bit = BitBoard::coordinate_to_bit(*at);
    let enemy_bits = match piece.color {
        Color::White => board.get_black_pieces_board(),
        Color::Black => board.get_white_pieces_board(),
    };
    let pieces_bit_board = board.get_piece_board();
    let less_board = start_bit - 1;
    let mut to_move_board: u64 = 0;
    let mut captures_board: u64 = 0;
    // length for diagonals == 1 || 2
    let diagonals = BitBoard::get_diagonals_vec_for_bit(start_bit);

    let is_a_file = BitBoard::on_file(start_bit, A_FILE);
    let is_h_file = BitBoard::on_file(start_bit, H_FILE);

    // check files or this stuff will wrap around, if the direction is off the board
    // just set it to 0
    let up_right_bit = match is_h_file {
        true => 0,
        false => start_bit << 9,
    };
    let up_left_bit = match is_a_file {
        true => 0,
        false => start_bit << 7,
    };
    let down_right_bit = match is_h_file {
        true => 0,
        false => start_bit >> 7,
    };
    let down_left_bit = match is_a_file {
        true => 0,
        false => start_bit >> 9,
    };

    for diagonal in diagonals {
        /* UP RIGHT DIAGONAL */
        if BitBoard::bit_on_bit_board(up_right_bit, diagonal)
            || BitBoard::bit_on_bit_board(down_left_bit, diagonal)
        {
            // cut diagonal in half, top / bottom
            let up_path = ((diagonal ^ less_board) & diagonal) ^ start_bit;
            let down_path = (diagonal ^ up_path) ^ start_bit;
            if up_path > 0 {
                let occupied = up_path & pieces_bit_board;
                // get nearest
                let nearest = BitBoard::lsb(occupied);
                if nearest == 0 {
                    // add all
                    to_move_board = to_move_board | up_path;
                } else {
                    let is_enemy = BitBoard::bit_on_bit_board(nearest, enemy_bits);
                    let mut below_nearest = (nearest - 1) & up_path;
                    if is_enemy {
                        below_nearest = ((nearest - 1) & up_path) | nearest;
                        captures_board = captures_board | nearest;
                    }
                    BitBoard::print_bitboard(below_nearest);
                    to_move_board = to_move_board | below_nearest;
                }
            }
            if down_path > 0 {
                let occupied = down_path & pieces_bit_board;
                // get nearest
                let nearest = BitBoard::msb(occupied);
                if nearest == 0 {
                    // add all
                    to_move_board = to_move_board | down_path;
                } else {
                    let is_enemy = BitBoard::bit_on_bit_board(nearest, enemy_bits);
                    let mut above_nearest = (!(nearest - 1) & down_path) ^ nearest;
                    if is_enemy {
                        above_nearest = !(nearest - 1) & down_path;
                        captures_board = captures_board | nearest;
                    }
                    to_move_board = to_move_board | above_nearest;
                }
            }
        }

        /* UP LEFT DIAGONAL */
        if BitBoard::bit_on_bit_board(up_left_bit, diagonal)
            || BitBoard::bit_on_bit_board(down_right_bit, diagonal)
        {
            // cut diagonal in half, top / bottom
            let up_path = ((diagonal ^ less_board) & diagonal) ^ start_bit;
            let down_path = (diagonal ^ up_path) ^ start_bit;
            if up_path > 0 {
                let occupied = up_path & pieces_bit_board;
                // get nearest
                let nearest = BitBoard::lsb(occupied);
                if nearest == 0 {
                    // add all
                    to_move_board = to_move_board | up_path;
                } else {
                    let is_enemy = BitBoard::bit_on_bit_board(nearest, enemy_bits);
                    let mut below_nearest = (nearest - 1) & up_path;
                    if is_enemy {
                        below_nearest = ((nearest - 1) & up_path) | nearest;
                        captures_board = captures_board | nearest;
                    }
                    to_move_board = to_move_board | below_nearest;
                }
            }
            if down_path > 0 {
                let occupied = down_path & pieces_bit_board;
                // get nearest
                let nearest = BitBoard::msb(occupied);
                if nearest == 0 {
                    // add all
                    to_move_board = to_move_board | down_path;
                } else {
                    let is_enemy = BitBoard::bit_on_bit_board(nearest, enemy_bits);
                    let mut above_nearest = (!(nearest - 1) & down_path) ^ nearest;
                    if is_enemy {
                        above_nearest = !(nearest - 1) & down_path;
                        captures_board = captures_board | nearest;
                    }
                    to_move_board = to_move_board | above_nearest;
                }
            }
        }
    }
    let mut moves: Vec<Move> = vec![];
    let color = piece.color;

    while to_move_board > 0 {
        let to_bit = BitBoard::pop_bit(&mut to_move_board);
        let to = BitBoard::bit_to_coordinate(to_bit);

        if BitBoard::bit_on_bit_board(to_bit, captures_board) {
            moves.push(make_move_to(
                at,
                &to,
                piece,
                MoveType::Move,
                &board,
                game_state,
            ));
        } else {
            moves.push(make_quiet_move_with_castle_rights(
                at, &to, piece, game_state,
            ));
        }
    }

    return moves;
}

// this isn't precalculated / using magic numbers yet
// for the moment I'm just calculating it with fancy smancy bit manips
pub fn gen_rook_moves(piece: &Piece, game_state: &GameState) -> Vec<Move> {
    let board = game_state.get_board();
    let at = piece.at().unwrap();
    let idx: u64 = BitBoard::coordinate_to_idx(*at);
    let pieces_bit_board = board.get_piece_board();

    //plan get the ray, remove this piece, check for nearest other piece
    //going up or right the nearest piece is the lsb
    let start_bit = BitBoard::coordinate_to_bit(*at);
    let enemy_bits = match piece.color {
        Color::White => board.get_black_pieces_board(),
        Color::Black => board.get_white_pieces_board(),
    };
    let less_board = start_bit - 1;
    let mut to_move_board: u64 = 0;
    let mut captures_board: u64 = 0;

    // break up the file into up/below sections
    let file = BitBoard::get_file_for_bit(start_bit);
    let above_me_file = (file ^ (less_board | start_bit)) & file;
    let below_me_file = (file ^ above_me_file) ^ start_bit;

    // above_me_file == 0 then there's no where up to go
    if above_me_file != 0 {
        let occuppied = above_me_file & pieces_bit_board;
        let nearest = BitBoard::lsb(occuppied);
        if nearest == 0 {
            to_move_board = to_move_board | above_me_file;
        } else {
            // if the nearest piece is an enemy we include it
            let is_enemy = BitBoard::bit_on_bit_board(nearest, enemy_bits);
            // now get everything below this on the file
            let mut below_nearest = (nearest - 1) & above_me_file;
            if is_enemy {
                below_nearest = ((nearest - 1) | nearest) & above_me_file;
                captures_board = captures_board | nearest;
            }
            to_move_board = to_move_board | below_nearest;
        }
    }
    // below_me_file == 0 then there's no where down to go
    if below_me_file != 0 {
        let occuppied = below_me_file & pieces_bit_board;
        let nearest = BitBoard::msb(occuppied);
        if nearest == 0 {
            to_move_board = to_move_board | below_me_file;
        } else {
            // if the nearest piece is an enemy we include it
            let is_enemy = BitBoard::bit_on_bit_board(nearest, enemy_bits);
            let mut above_nearest = (!(nearest - 1) & below_me_file) ^ nearest;
            if is_enemy {
                above_nearest = !(nearest - 1) & below_me_file;
                captures_board = captures_board | nearest;
            }
            to_move_board = to_move_board | above_nearest;
        }
    }

    // break up the row into left/right sections
    let row = BitBoard::get_row_for_bit(start_bit);
    let left_row = less_board & row;
    let right_row = (row ^ left_row) ^ start_bit;
    
    //left row == 0 then there's nowhere left
    if left_row != 0 {
        let occupied = left_row & pieces_bit_board;
        let nearest = BitBoard::msb(occupied);
        if nearest == 0 {
            to_move_board = to_move_board | left_row;
        } else {
            // if the nearest piece is an enemy we include it
            let is_enemy = BitBoard::bit_on_bit_board(nearest, enemy_bits);
            let mut right_of_nearest = (!(nearest - 1) & left_row) ^ nearest;
            if is_enemy {
                right_of_nearest = !(nearest - 1) & left_row;
                captures_board = captures_board | nearest;
            }
            to_move_board = to_move_board | right_of_nearest;
        }
    }
    // right row == 0 then there's nowhere right
    if right_row != 0 {
        let occupied = right_row & pieces_bit_board;
        let nearest = BitBoard::lsb(occupied);
        if nearest == 0 {
            to_move_board = to_move_board | right_row;
        } else {
            // if the nearest piece is an enemy we include it
            let is_enemy = BitBoard::bit_on_bit_board(nearest, enemy_bits);
            let mut left_of_nearest = (nearest - 1) & right_row;
            if is_enemy {
                left_of_nearest = ((nearest - 1) | nearest) & right_row;
                captures_board = captures_board | nearest;
            }
            to_move_board = to_move_board | left_of_nearest;
        }
    }
    let mut moves: Vec<Move> = vec![];
    let color = piece.color;

    while to_move_board > 0 {
        let to_bit = BitBoard::pop_bit(&mut to_move_board);
        let to = BitBoard::bit_to_coordinate(to_bit);

        if BitBoard::bit_on_bit_board(to_bit, captures_board) {
            moves.push(make_move_to(
                at,
                &to,
                piece,
                MoveType::Move,
                &board,
                game_state,
            ));
        } else {
            moves.push(make_quiet_move_with_castle_rights(
                at, &to, piece, game_state,
            ));
        }
    }

    return moves;
}

pub fn gen_king_moves(piece: &Piece, game_state: &GameState) -> Vec<Move> {
    let board = game_state.get_board();
    let at = piece.at().unwrap();
    let idx = BitBoard::coordinate_to_idx(*at);
    let attack_board: u64 = KING_ATTACKS[(idx - 1) as usize];
    let mut moves: Vec<Move> = vec![];
    let color = piece.color;
    println!("{} {} {}", at, idx, color);
    BitBoard::print_bitboard(attack_board);

    for to in BitBoard::attack_map_to_coordinates(attack_board) {
        if square_occupiable_by(&board, &to, color) {
            moves.push(make_move_to(
                at,
                &to,
                piece,
                MoveType::Move,
                &board,
                game_state,
            ));
        }
    }

    // castling
    let rights = game_state.get_castling_rights(color);
    let king_side_blockers = match color {
        Color::White => WHITE_KINGSIDE_CASTLE_BLOCKERS,
        Color::Black => BLACK_KINGSIDE_CASTLE_BLOCKERS,
    };
    let queens_side_blockers = match color {
        Color::White => WHITE_QUEENSIDE_CASTLE_BLOCKERS,
        Color::Black => BLACK_QUEENSIDE_CASTLE_BLOCKERS,
    };
    if rights.king_side() & !board.has_piece_at(king_side_blockers) {
        moves.push(Move::castle_king_side(color));
    }
    if rights.queen_side() & !board.has_piece_at(queens_side_blockers) {
        moves.push(Move::castle_queen_side(color));
    }

    return moves;
}

/*@todo : test  */
pub fn gen_knight_moves(piece: &Piece, game_state: &GameState) -> Vec<Move> {
    let board = game_state.get_board();
    let at = piece.at().unwrap();
    let idx = BitBoard::coordinate_to_idx(*at);
    let attack_board: u64 = KNIGHT_ATTACKS[(idx - 1) as usize];
    println!("{} {} {}", at, idx, attack_board);
    BitBoard::print_bitboard(attack_board);
    let mut moves: Vec<Move> = vec![];
    let color = piece.color;
    //@todo : get captured piece type
    for to in BitBoard::attack_map_to_coordinates(attack_board) {
        if square_occupiable_by(&board, &to, color) {
            moves.push(make_move_to(
                at,
                &to,
                piece,
                MoveType::Move,
                &board,
                game_state,
            ));
        }
    }

    return moves;
}

/*
 *
one square move, two square move, capturing diagonally forward, pawn promotion, en passant
**/
pub fn gen_pawn_moves(piece: &Piece, game_state: &GameState) -> Vec<Move> {
    let board = game_state.get_board();
    let at = piece.at().unwrap();
    let idx = BitBoard::coordinate_to_idx(*at);
    // println!("{} {}", at, idx);
    let mut moves: Vec<Move> = vec![];
    let color = piece.color;

    // quiet pawn moves
    let enemy_pieces = match piece.color {
        Color::Black => board.get_white_pieces(),
        Color::White => board.get_black_pieces(),
    };
    let friendly_pieces = match piece.color {
        Color::White => board.get_white_pieces(),
        Color::Black => board.get_black_pieces(),
    };
    let direction = match piece.color {
        Color::White => 1,
        Color::Black => -1,
    };
    let start_y = match piece.color {
        Color::White => 2,
        Color::Black => 7,
    };
    let promotion_y = match piece.color {
        Color::White => 8,
        Color::Black => 1,
    };
    let promotion_pieces = [
        PieceType::Rook,
        PieceType::Queen,
        PieceType::Bishop,
        PieceType::Knight,
    ];
    let up_one = at.add(0, 1 * direction);
    let up_two = at.add(0, 2 * direction);
    if square_is_empty(&board, &up_one) {
        //promotion
        if up_one.y() == promotion_y {
            // make promotion moves
            for promotion_type in promotion_pieces.iter() {
                moves.push(Move::new(
                    *at,
                    up_one,
                    piece.piece_type,
                    MoveType::Promotion(*promotion_type),
                    None,
                    None,
                    None,
                ));
            }
        } else {
            moves.push(make_quiet_move(at, &up_one, piece));
            if square_is_empty(&board, &up_two) && at.y() == start_y {
                moves.push(make_quiet_move(at, &up_two, piece));
            }
        }
    }

    // pawn captures
    // check attack_board for enemy pieces
    let map = match piece.color {
        Color::Black => BLACK_PAWN_ATTACKS,
        Color::White => WHITE_PAWN_ATTACKS,
    };

    // check attack squares and enemy piece locations align
    let attack_board: u64 = map[(idx - 1) as usize];
    let capture_board = attack_board & enemy_pieces;
    // if capture board is not empty
    if capture_board > 0 {
        for to in BitBoard::attack_map_to_coordinates(capture_board) {
            //check promotion
            if to.y() == promotion_y {
                // make promotion moves
                for promotion_type in promotion_pieces.iter() {
                    moves.push(make_move_to(
                        at,
                        &to,
                        piece,
                        MoveType::Promotion(*promotion_type),
                        &board,
                        game_state,
                    ));
                }
            } else {
                moves.push(make_move_to(
                    at,
                    &to,
                    piece,
                    MoveType::Move,
                    &board,
                    game_state,
                ));
            }
        }
    }

    // en passant target
    let en_passant_opt = game_state.get_en_passant_target();
    // println!("{:?}", en_passant_opt);
    if en_passant_opt.is_some() {
        let to = en_passant_opt.unwrap();
        //todo::
        // let captured = board.get_piece_at(&to).unwrap();
        let bit = BitBoard::coordinate_to_bit(to);
        let target = bit & attack_board;
        // BitBoard::print_bitboard(attack_board);
        // BitBoard::print_bitboard(bit);
        // println!("{}, {}, {}", to, bit, target);
        if target > 0 {
            moves.push(Move::new(
                at.clone(),
                to.clone(),
                PieceType::Pawn,
                MoveType::EnPassant,
                Some(PieceType::Pawn),
                None,
                None,
            ));
        }
    }

    return moves;
}

/* HELPER FUNCTIONS  */
fn square_is_empty(board: &BitBoard, at: &Coordinate) -> bool {
    !board.is_piece_at_coordinate(at)
}

// if square is off board || square has friendly price => false
fn square_occupiable_by(board: &BitBoard, at: &Coordinate, color: Color) -> bool {
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

/* works if no castling rights have changed and no captures */
pub fn make_quiet_move(from: &Coordinate, to: &Coordinate, piece: &Piece) -> Move {
    return Move::new(
        from.clone(),
        to.clone(),
        piece.piece_type,
        MoveType::Move,
        None,
        None,
        None,
    );
}

pub fn make_quiet_move_with_castle_rights(
    from: &Coordinate,
    to: &Coordinate,
    piece: &Piece,
    game_state: &GameState,
) -> Move {
    return Move::new(
        from.clone(),
        to.clone(),
        piece.piece_type,
        MoveType::Move,
        None,
        game_state.get_castling_rights_changes_if_piece_moves(piece),
        None,
    );
}

pub fn make_move_to(
    from: &Coordinate,
    to: &Coordinate,
    piece: &Piece,
    move_type: MoveType,
    board: &BitBoard,
    game_state: &GameState,
) -> Move {
    let captured = board.get_piece_at(&to);
    Move::new(
        from.clone(),
        to.clone(),
        piece.piece_type,
        move_type,
        captured.map(|p| p.piece_type.clone()),
        game_state.get_castling_rights_changes_if_piece_moves(piece),
        captured.map_or_else(
            || None,
            |p| game_state.get_castling_rights_changes_if_piece_moves(&p),
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{board::BoardTrait, chess_notation::fen_reader, move_generator::plmg};

    #[test]
    fn test_gen_bishop_vector() {
        let white_bishop_pinned =
            "rnbqk1nr/pppp1ppp/4p3/8/1b1P4/5N2/PPPBPPPP/RN1QKB1R b KQkq - 3 3";
        let game_state = fen_reader::make_game_state(white_bishop_pinned);
        // white bishop on 4,2 should have 8 moves
        let d_2_bishop = game_state.get_piece_at(&Coordinate::new(4, 2));
        assert!(d_2_bishop.is_some(), "bishop should be on d2");
        let d_2_b = d_2_bishop.unwrap();
        let moves = plmg::gen_bishop_vector(&game_state, d_2_b);
        assert_eq!(moves.len(), 8);


        // white bishop on 6,1 should have 0 moves
        let f_1_bishop = game_state.get_piece_at(&Coordinate::new(6, 1));
        assert!(f_1_bishop.is_some(), "bishop should be on f1");
        let f_1_b = f_1_bishop.unwrap();
        let moves = plmg::gen_bishop_vector(&game_state, f_1_b);
        assert_eq!(moves.len(), 0);

        // black bishop on 2,4 should have 9 moves
        let b_4_bishop = game_state.get_piece_at(&Coordinate::new(2, 4));
        assert!(b_4_bishop.is_some(), "bishop should be on b4");
        let b_4_b = b_4_bishop.unwrap();
        let moves = plmg::gen_bishop_vector(&game_state, b_4_b);
        assert_eq!(moves.len(), 9);

        // black bishop on 3,8 should have 0 moves
        let c_8_bishop = game_state.get_piece_at(&Coordinate::new(3, 8));
        assert!(c_8_bishop.is_some(), "bishop should be on c8");
        let c_8_b = c_8_bishop.unwrap();
        let moves = plmg::gen_bishop_vector(&game_state, c_8_b);
        assert_eq!(moves.len(), 0);

    }

    #[test]
    fn test_gen_rook_vector() {

    }
}
