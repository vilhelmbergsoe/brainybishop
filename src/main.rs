const WHITE_PAWN: usize = 0;
const WHITE_KNIGHT: usize = 1;
const WHITE_BISHOP: usize = 2;
const WHITE_ROOK: usize = 3;
const WHITE_QUEEN: usize = 4;
const WHITE_KING: usize = 5;
const BLACK_PAWN: usize = 6;
const BLACK_KNIGHT: usize = 7;
const BLACK_BISHOP: usize = 8;
const BLACK_ROOK: usize = 9;
const BLACK_QUEEN: usize = 10;
const BLACK_KING: usize = 11;

#[derive(Debug)]
enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug)]
enum Color {
    White,
    Black,
}

#[derive(Debug)]
struct Square(u8);

// impl std::fmt::Display for Square {

// }

impl Square {
    fn from(file: u8, rank: u8) -> Square {
        Square(rank * 8 + file)
    }

    fn from_algebraic(algebraic: &str) -> Option<Square> {
        let algebraic = algebraic.as_bytes();

        if algebraic.len() != 2 {
            return None;
        }

        let file = algebraic[0];
        let rank = algebraic[1];

        if file < b'a' || file > b'h' || rank < b'1' || rank > b'8' {
            return None;
        }

        Some(Square::from(file - b'a', rank - b'1'))
    }

    fn to_algebraic(&self) -> String {
        let file = self.0 % 8;
        let rank = self.0 / 8;

        let mut algebraic = String::new();

        algebraic.push((file + b'a') as char);
        algebraic.push((rank + b'1') as char);

        algebraic
    }
}

struct Board {
    // 0-5: white, 6-11: black
    // 0: pawn, 1: knight, 2: bishop, 3: rook, 4: queen, 5: king
    // 6: pawn, 7: knight, 8: bishop, 9: rook, 10: queen, 11: king
    bitboard: [u64; 12],
}

impl std::default::Default for Board {
    fn default() -> Self {
        Board {
            // Default starting position
            bitboard: [
                0xFF00,
                0x42,
                0x24,
                0x81,
                0x08,
                0x16,
                0xFF000000000000,
                0x4200000000000000,
                0x2400000000000000,
                0x8100000000000000,
                0x0800000000000000,
                0x1600000000000000,
            ],
        }
    }
}

impl Board {
    fn new() -> Board {
        Board { bitboard: [0; 12] }
    }
    fn get_piece_at(&self, square: &Square) -> Option<(Piece, Color)> {
        let square = square.0;

        if square > 63 {
            return None;
        }

        for (index, &bitboard) in self.bitboard.iter().enumerate() {
            if bitboard & (1u64 << square) != 0 {
                match index {
                    WHITE_PAWN => return Some((Piece::Pawn, Color::White)),
                    BLACK_PAWN => return Some((Piece::Pawn, Color::Black)),
                    WHITE_KNIGHT => return Some((Piece::Knight, Color::White)),
                    BLACK_KNIGHT => return Some((Piece::Knight, Color::Black)),
                    WHITE_BISHOP => return Some((Piece::Bishop, Color::White)),
                    BLACK_BISHOP => return Some((Piece::Bishop, Color::Black)),
                    WHITE_ROOK => return Some((Piece::Rook, Color::White)),
                    BLACK_ROOK => return Some((Piece::Rook, Color::Black)),
                    WHITE_QUEEN => return Some((Piece::Queen, Color::White)),
                    BLACK_QUEEN => return Some((Piece::Queen, Color::Black)),
                    WHITE_KING => return Some((Piece::King, Color::White)),
                    BLACK_KING => return Some((Piece::King, Color::Black)),
                    _ => unreachable!(),
                };
            }
        }

        None
    }
    fn set_piece_at(&mut self, square: Square, piece: (Piece, Color)) {
        if square.0 > 63 {
            return;
        }

        self.remove_piece_at(&square);

        let idx = self.get_piece_idx(piece);

        self.bitboard[idx] |= 1u64 << square.0;
    }
    fn remove_piece_at(&mut self, square: &Square) {
        let prev_piece = match self.get_piece_at(square) {
            Some(v) => v,
            None => return,
        };

        let idx = self.get_piece_idx(prev_piece);

        self.bitboard[idx] &= !(1u64 << square.0);
    }
    fn move_piece(&mut self, from: Square, to: Square) {
        let piece = self.get_piece_at(&from).unwrap();

        if from.0 > 63 || to.0 > 63 {
            return;
        }

        self.remove_piece_at(&to);

        let idx = self.get_piece_idx(piece);

        self.bitboard[idx] &= !(1u64 << from.0);
        self.bitboard[idx] |= 1u64 << to.0;
    }
    fn get_piece_idx(&self, piece: (Piece, Color)) -> usize {
        match piece {
            (Piece::Pawn, Color::White) => 0,
            (Piece::Knight, Color::White) => 1,
            (Piece::Bishop, Color::White) => 2,
            (Piece::Rook, Color::White) => 3,
            (Piece::Queen, Color::White) => 4,
            (Piece::King, Color::White) => 5,
            (Piece::Pawn, Color::Black) => 6,
            (Piece::Knight, Color::Black) => 7,
            (Piece::Bishop, Color::Black) => 8,
            (Piece::Rook, Color::Black) => 9,
            (Piece::Queen, Color::Black) => 10,
            (Piece::King, Color::Black) => 11,
        }
    }
    fn get_legal_moves(&self, square: Square) -> Option<Vec<Square>> {
        let (piece, color) = match self.get_piece_at(&square) {
            Some(piece_color) => piece_color,
            None => return None, // No piece at the given square
        };

        match piece {
            Piece::Pawn => self.get_pawn_moves(square, color),
            _ => todo!(),
        }
    }

    fn get_pawn_moves(&self, square: Square, color: Color) -> Option<Vec<Square>> {
        let direction = match color {
            Color::White => 1,
            Color::Black => -1,
        };

        let mut moves: Vec<Square> = Vec::new();

        // Normal pawn move (one square forward)
        let forward_one = square.0 as i8 + direction * 8;
        if forward_one >= 0 && forward_one < 64 {
            let target_square = Square(forward_one as u8);
            if self.get_piece_at(&target_square).is_none() {
                moves.push(target_square);
            }
        }

        // Double pawn move (two squares forward from starting position)
        let starting_rank = match color {
            Color::White => 0xFF00,
            Color::Black => 0xFF000000000000,
        };
        if (2_u64.pow(square.0.into())) & starting_rank != 0 {
            let forward_two = square.0 as i8 + (2 * direction * 8);
            if forward_two >= 0 && forward_two < 64 {
                let target_square = Square(forward_two as u8);
                if self.get_piece_at(&target_square).is_none() {
                    moves.push(target_square);
                }
            }
        }

        // Capture moves
        // let capture_files = [-1, 1];
        // for &file_offset in capture_files.iter() {
        //     let target_file = (square.0 % 8) as i8 + file_offset;
        //     let target_rank = square.0 as i8 + direction;

        //     if target_file >= 0 && target_file < 8 && target_rank >= 0 && target_rank < 8 {
        //         let target_square = Square((target_rank * 8 + target_file) as u8);
        //         if let Some((captured_piece, captured_color)) = self.get_piece_at(target_square) {
        //             if captured_color != color {
        //                 moves.push(target_square);
        //             }
        //         }
        //     }
        // }

        // En passant (to be implemented)

        // Promotion (to be implemented)

        if moves.is_empty() {
            None
        } else {
            Some(moves)
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chessboard = [
            '♟', '♞', '♝', '♜', '♛', '♚', // White pieces
            '♙', '♘', '♗', '♖', '♕', '♔', // Black pieces
        ];

        for rank in (0..8).rev() {
            write!(f, "{} ", rank + 1)?;

            for file in 0..8 {
                let square_index = rank * 8 + file;
                let mut piece_found = false;

                for (index, &bitboard) in self.bitboard.iter().enumerate() {
                    if bitboard & (1u64 << square_index) != 0 {
                        write!(f, "{} ", chessboard[index])?;
                        piece_found = true;
                        break;
                    }
                }

                if !piece_found {
                    write!(f, "  ")?;
                }
            }

            writeln!(f)?;
        }

        writeln!(f, "  a b c d e f g h")?;

        Ok(())
    }
}

fn main() {
    let mut board = Board::default();

    // pretty print the binary representation of the first bitboard
    // println!("{:064b}", board.bitboard[7]);

    print!("{}", board);

    board.set_piece_at(
        Square::from_algebraic("a1").unwrap(),
        (Piece::Pawn, Color::Black),
    );

    print!("{}", board);

    println!(
        "{:?}",
        board.get_legal_moves(Square::from_algebraic("a1").unwrap())
    );

    board.move_piece(Square(0), Square(8));

    board.move_piece(
        Square::from_algebraic("f2").unwrap(),
        Square::from_algebraic("f3").unwrap(),
    );

    print!("{}", board);

    println!(
        "legal moves for b2: {:?}",
        board
            .get_legal_moves(Square::from_algebraic("b2").unwrap())
            .unwrap()
            .iter()
            .map(|x| x.to_algebraic())
            .collect::<Vec<String>>()
    );

    println!(
        "legal moves for b7: {:?}",
        board
            .get_legal_moves(Square::from_algebraic("b7").unwrap())
            .unwrap()
            .iter()
            .map(|x| x.to_algebraic())
            .collect::<Vec<String>>()
    );

    println!(
        "legal moves for f3: {:?}",
        board
            .get_legal_moves(Square::from_algebraic("f3").unwrap())
            .unwrap()
            .iter()
            .map(|x| x.to_algebraic())
            .collect::<Vec<String>>()
    );
    // board.move_piece(
    //     Square::from_algebraic("c2").unwrap(),
    //     Square::from_algebraic("c3").unwrap(),
    // );
    // print!("{}", board);
    // println!(
    //     "{:?}",
    //     board.get_legal_moves(Square::from_algebraic("c3").unwrap())
    // );

    // let piece = board.get_piece_at(&Square::from(0, 0)).unwrap();

    // println!("piece at (0, 0) is a {:?} {:?}", piece.1, piece.0);

    // board.set_piece_at(Square::from(0, 0), (Piece::Pawn, Color::White));

    // println!(
    //     "piece at b8 is {:?}",
    //     board.get_piece_at(&Square::from_algebraic("b8").unwrap())
    // );

    // println!("{:?}", Square::from_algebraic("b8").unwrap().to_algebraic());

    // board.move_piece(
    //     Square::from_algebraic("a2").unwrap(),
    //     Square::from_algebraic("a4").unwrap(),
    // );

    // print!("{}", board);

    // board.set_piece_at(Square::from_algebraic("a1").unwrap(), (Piece::Pawn, Color::Black));

    // board.move_piece(Square::from_algebraic("a7").unwrap(), Square::from_algebraic("a5").unwrap());

    // print!("{}", board);

    // println!("legal moves for g8: {:?}", board.get_legal_moves(Square::from_algebraic("a1").unwrap()));
}
