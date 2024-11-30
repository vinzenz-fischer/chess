pub struct ChessBoard {
    board: [Occupation; 64],
    game_state: GameState
}

impl ChessBoard {
    // Creates a new empty ChessBoard.
    pub fn new() -> Self {
        Self {
            board: [const { Occupation::Neutral }; 64],
            game_state: GameState::WhiteToMove,
        }
    }

    /// Creates a new ChessBoard from a string. Returns an empty error if the string isn't valid **fen notation**.
    pub fn from_fen(fen: String) -> Result<Self, ()> {
        let mut board = [const { Occupation::Neutral }; 64];
        
        let mut i = 0;
        for chr in fen.chars() {
            match chr {
                '1'..'9' => i += chr.to_ascii_lowercase() as usize - '0' as usize, // Skip x squares.
                '/' => i = i - (i % 8) + 8, // Skip to next row.
                'K' => { board[i] = Occupation::White(ChessPiece::King);   i += 1 },
                'Q' => { board[i] = Occupation::White(ChessPiece::Queen);  i += 1 },
                'R' => { board[i] = Occupation::White(ChessPiece::Rook);   i += 1 },
                'N' => { board[i] = Occupation::White(ChessPiece::Knight); i += 1 },
                'B' => { board[i] = Occupation::White(ChessPiece::Bishop); i += 1 },
                'P' => { board[i] = Occupation::White(ChessPiece::Pawn);   i += 1 },
                'k' => { board[i] = Occupation::Black(ChessPiece::King);   i += 1 },
                'q' => { board[i] = Occupation::Black(ChessPiece::Queen);  i += 1 },
                'r' => { board[i] = Occupation::Black(ChessPiece::Rook);   i += 1 },
                'n' => { board[i] = Occupation::Black(ChessPiece::Knight); i += 1 },
                'b' => { board[i] = Occupation::Black(ChessPiece::Bishop); i += 1 },
                'p' => { board[i] = Occupation::Black(ChessPiece::Pawn);   i += 1 },
                _ => return Err(())
            }
            if i >= 64 { break }
        }

        todo!("There is more to fen notation (eg. \" w - -\")");

        // Ok( Self { board, current_team: Team::White } )
    }
}

impl Default for ChessBoard {
    fn default() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_owned()).expect("Failed to generate default chess board from fen-string.")
    }
}

pub enum GameState {
    WhiteToMove,
    BlackToMove,
    WhiteWon,
    BlackWon,
    Tie,
}

pub enum Occupation {
    Neutral,
    White(ChessPiece),
    Black(ChessPiece),
}

pub enum ChessPiece {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
}

impl ChessPiece {
    pub fn get_value(&self) -> u8 {
        match self {
            Self::King   => u8::MAX,
            Self::Queen  => 9,
            Self::Rook   => 5,
            Self::Knight => 3,
            Self::Bishop => 3,
            Self::Pawn   => 1,
        }
    }
}
