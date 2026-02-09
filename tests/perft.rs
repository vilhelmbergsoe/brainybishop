use brainybishop::board::Board;
use brainybishop::movegen::generate_moves;

fn perft(board: Board, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }

    let moves = generate_moves(&board);
    if depth == 1 {
        return moves.len() as u64;
    }

    let mut nodes = 0u64;
    for mv in moves.iter() {
        let new_board = board.make_move(*mv);
        nodes += perft(new_board, depth - 1);
    }

    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perft_starting_position() {
        let board = Board::default();
        
        // Depth 1: 20 moves
        assert_eq!(perft(board, 1), 20);
        
        // Depth 2: 400 moves
        assert_eq!(perft(board, 2), 400);
    }

    #[test]
    fn test_perft_en_passant_position() {
        let board = Board::from_fen("rnbqkbnr/ppp1p1pp/8/3pPp2/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 1").unwrap();
        
        // Depth 1: 31 moves (including en passant capture)
        assert_eq!(perft(board, 1), 31);
    }

    #[test]
    fn test_perft_kiwi_pawn_position() {
        let board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
        
        // Depth 1: 48 moves
        assert_eq!(perft(board, 1), 48);
        
        // Depth 2: 2039 moves
        assert_eq!(perft(board, 2), 2039);
    }

    #[test]
    fn test_move_generation_consistency() {
        let board = Board::default();
        let moves = generate_moves(&board);
        
        // Should have exactly 20 legal moves from starting position
        assert_eq!(moves.len(), 20);
        
        // All moves should be unique
        let mut move_strings: Vec<String> = moves.iter().map(|mv| mv.to_string()).collect();
        move_strings.sort();
        move_strings.dedup();
        assert_eq!(move_strings.len(), 20);
    }
}
