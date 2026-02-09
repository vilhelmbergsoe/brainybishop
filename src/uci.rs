use std::io::{self, Write};
use crate::board::{Board, Color};
use crate::error::Result;
use crate::movegen::{generate_moves, Move};
use crate::eval::evaluate_position;

pub struct UciEngine {
    board: Board,
}

impl UciEngine {
    pub fn new() -> Self {
        Self {
            board: Board::default(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut input = String::new();

        loop {
            input.clear();
            match io::stdin().read_line(&mut input) {
                Ok(0) => break,
                Ok(_) => {
                    let command = input.trim();
                    if let Err(e) = self.handle_uci_command(command) {
                        eprintln!("Error: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Input error: {}", e);
                    break;
                }
            }
        }

        Ok(())
    }

    fn handle_uci_command(&mut self, command: &str) -> Result<()> {
        let parts: Vec<&str> = command.split_whitespace().collect();

        if parts.is_empty() {
            return Ok(());
        }

        match parts[0] {
            "uci" => self.uci_identify(),
            "isready" => self.is_ready(),
            "ucinewgame" => self.uci_new_game(),
            "position" => self.uci_position(&parts[1..])?,
            "go" => self.uci_go()?,
            "stop" => {}
            "quit" => std::process::exit(0),
            _ => {}
        }

        Ok(())
    }

    fn uci_identify(&self) {
        println!("id name BrainyBishop 0.1.0");
        println!("id author BrainyBishop Team");
        println!("uciok");
    }

    fn is_ready(&self) {
        println!("readyok");
    }

    fn uci_new_game(&mut self) {
        self.board = Board::default();
    }

    fn uci_position(&mut self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            return Ok(());
        }

        match args[0] {
            "startpos" => {
                self.board = Board::default();
                if args.len() > 1 && args[1] == "moves" {
                    for move_str in &args[2..] {
                        self.apply_move(move_str)?;
                    }
                }
            }
            "fen" => {
                let mut fen_parts = Vec::new();
                let mut i = 1;
                while i < args.len() && args[i] != "moves" {
                    fen_parts.push(args[i]);
                    i += 1;
                }
                let fen_string = fen_parts.join(" ");
                self.board = Board::from_fen(&fen_string)?;

                if i < args.len() && args[i] == "moves" {
                    for move_str in &args[i + 1..] {
                        self.apply_move(move_str)?;
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn uci_go(&self) -> Result<()> {
        let best_move = find_best_move(&self.board, 3);

        match best_move {
            Some(mv) => println!("bestmove {}", mv),
            None => println!("bestmove 0000"),
        }

        Ok(())
    }

    fn apply_move(&mut self, move_str: &str) -> Result<()> {
        let moves = generate_moves(&self.board);

        for mv in moves.iter() {
            if mv.to_string() == move_str {
                self.board = self.board.make_move(*mv);
                return Ok(());
            }
        }

        Err(crate::error::Error::InvalidMove(move_str.to_string()))
    }
}

impl Default for UciEngine {
    fn default() -> Self {
        Self::new()
    }
}

fn find_best_move(board: &Board, depth: u32) -> Option<Move> {
    let moves = generate_moves(board);

    if moves.is_empty() {
        return None;
    }

    let maximizing = board.side_to_move() == Color::White;
    let mut best_move = None;
    let mut best_score = if maximizing { i32::MIN } else { i32::MAX };

    for mv in moves.iter() {
        let new_board = board.make_move(*mv);
        let score = minimax(&new_board, depth - 1, i32::MIN, i32::MAX, !maximizing);

        if maximizing {
            if score > best_score {
                best_score = score;
                best_move = Some(*mv);
            }
        } else if score < best_score {
            best_score = score;
            best_move = Some(*mv);
        }
    }

    best_move
}

fn minimax(board: &Board, depth: u32, mut alpha: i32, mut beta: i32, maximizing: bool) -> i32 {
    if depth == 0 {
        return evaluate_position(board);
    }

    let moves = generate_moves(board);

    if moves.is_empty() {
        // Check if it's checkmate or stalemate
        let king_sq = board.king_square(board.side_to_move());
        let in_check = is_square_attacked(board, king_sq, board.side_to_move().opposite());

        if in_check {
            // Checkmate
            return if maximizing { -100000 + depth as i32 } else { 100000 - depth as i32 };
        } else {
            // Stalemate
            return 0;
        }
    }

    if maximizing {
        let mut max_eval = i32::MIN;
        for mv in moves.iter() {
            let new_board = board.make_move(*mv);
            let eval = minimax(&new_board, depth - 1, alpha, beta, false);
            max_eval = max_eval.max(eval);
            alpha = alpha.max(eval);
            if beta <= alpha {
                break;
            }
        }
        max_eval
    } else {
        let mut min_eval = i32::MAX;
        for mv in moves.iter() {
            let new_board = board.make_move(*mv);
            let eval = minimax(&new_board, depth - 1, alpha, beta, true);
            min_eval = min_eval.min(eval);
            beta = beta.min(eval);
            if beta <= alpha {
                break;
            }
        }
        min_eval
    }
}

fn is_square_attacked(board: &Board, sq: usize, by_color: Color) -> bool {
    use crate::magic::{bishop_attacks, rook_attacks};
    use crate::tables::{KING_ATTACKS, KNIGHT_ATTACKS, PAWN_ATTACKS};
    use crate::board::PieceType;

    let occ = board.all_occupancy();

    // Pawn attacks
    let pawn_attacks = PAWN_ATTACKS[by_color.opposite().index()][sq];
    if pawn_attacks & board.pieces(PieceType::Pawn, by_color) != 0 {
        return true;
    }

    // Knight attacks
    if KNIGHT_ATTACKS[sq] & board.pieces(PieceType::Knight, by_color) != 0 {
        return true;
    }

    // Bishop/Queen attacks
    let bishop_rays = bishop_attacks(sq, occ);
    if bishop_rays & (board.pieces(PieceType::Bishop, by_color) | board.pieces(PieceType::Queen, by_color)) != 0 {
        return true;
    }

    // Rook/Queen attacks
    let rook_rays = rook_attacks(sq, occ);
    if rook_rays & (board.pieces(PieceType::Rook, by_color) | board.pieces(PieceType::Queen, by_color)) != 0 {
        return true;
    }

    // King attacks
    if KING_ATTACKS[sq] & board.pieces(PieceType::King, by_color) != 0 {
        return true;
    }

    false
}

pub fn run_interactive_mode(player_color: Color) -> Result<()> {
    println!("brainybishop - {}", env!("CARGO_PKG_VERSION"));

    let computer_color = player_color.opposite();
    let mut board = Board::default();
    let mut input = String::new();

    loop {
        board.display();
        println!();

        let moves = generate_moves(&board);
        if moves.is_empty() {
            let king_sq = board.king_square(board.side_to_move());
            let in_check = is_square_attacked(&board, king_sq, board.side_to_move().opposite());

            if in_check {
                println!("checkmate");
            } else {
                println!("stalemate");
            }
            break;
        }

        if board.side_to_move() == computer_color {
            if let Some(mv) = find_best_move(&board, 4) {
                println!("{}", mv);
                board = board.make_move(mv);
            } else {
                break;
            }
            continue;
        }

        print!("> ");
        io::stdout().flush()?;

        input.clear();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {
                let command = input.trim();

                if command == "quit" || command == "q" {
                    break;
                }

                if command == "help" || command == "?" {
                    println!("moves: e2e4, g1f3, e7e8q");
                    println!("quit, q: exit");
                    continue;
                }

                if command.is_empty() {
                    continue;
                }

                // Try to parse as a move
                let mut found = false;
                for mv in moves.iter() {
                    if mv.to_string() == command {
                        board = board.make_move(*mv);
                        found = true;
                        break;
                    }
                }

                if !found {
                    println!("invalid: {}", command);
                }
            }
            Err(e) => {
                eprintln!("Input error: {}", e);
                break;
            }
        }
    }

    Ok(())
}

