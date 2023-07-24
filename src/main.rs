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

struct Square(u8);

impl Square {
    fn new(file: u8, rank: u8) -> Square {
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

        Some(Square::new(file - b'a', rank - b'1'))
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

impl Board {
    fn new() -> Board {
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
    fn get_piece_at(&self, square: Square) -> Option<(Piece, Color)> {
        let square = square.0;

        if square > 63 {
            return None;
        }

        for (index, &bitboard) in self.bitboard.iter().enumerate() {
            if bitboard & (1u64 << square) != 0 {
                let piece = match index {
                    0 | 6 => Piece::Pawn,
                    1 | 7 => Piece::Knight,
                    2 | 8 => Piece::Bishop,
                    3 | 9 => Piece::Rook,
                    4 | 10 => Piece::Queen,
                    5 | 11 => Piece::King,
                    _ => unreachable!(),
                };

                let color = if index < 6 {
                    Color::White
                } else {
                    Color::Black
                };

                return Some((piece, color));
            }
        }

        None
    }
    fn set_piece_at(&mut self, square: Square, piece: (Piece, Color)) {
        let square = square.0;

        if square > 63 {
            return;
        }

        let index = match (piece.0, piece.1) {
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
        };

        self.bitboard[index] |= 1u64 << square;
    }
    fn move_piece(&mut self, from: Square, to: Square) {
        let from = from.0;
        let to = to.0;

        if from > 63 || to > 63 {
            return;
        }

        for bitboard in self.bitboard.iter_mut() {
            *bitboard &= !(1u64 << from);
            *bitboard |= 1u64 << to;
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
    let mut board = Board::new();

    // pretty print the binary representation of the first bitboard
    // println!("{:064b}", board.bitboard[7]);

    print!("{}", board);

    let piece = board.get_piece_at(Square::new(0, 0)).unwrap();

    println!("piece at (0, 0) is a {:?} {:?}", piece.1, piece.0);

    board.set_piece_at(Square::new(0, 0), (Piece::Pawn, Color::White));

    println!(
        "{:?}",
        board.get_piece_at(Square::from_algebraic("b8").unwrap())
    );

    println!("{:?}", Square::from_algebraic("b8").unwrap().to_algebraic());

    board.move_piece(Square::from_algebraic("a2").unwrap(), Square::from_algebraic("a4").unwrap());

    print!("{}", board);
}
