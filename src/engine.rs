#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChessBoard {
    board: [Occupation; 64],
    game_state: GameState,

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
    pub fn from_fen(fen: String) -> Result<Self, &'static str> {
        let mut fen_groups = fen.split(' ');
        let fen_board = fen_groups.next().ok_or("Invalid FEN string.")?;

        // Group 1: Board
        let mut i = 0;
        let mut board = [const { Occupation::Neutral }; 64];
        for chr in fen_board.chars() {
            match chr {
                '1'..'9' => i += chr.to_ascii_lowercase() as usize - '0' as usize, // Skip x squares.
                '/' => match i % 8 == 0 {
                    false => i += 8 - (i % 8), // Skip to next row.
                    true => {} // We're already at the start of a row.
                }
                ' ' => break,
                _ => {
                    board[i] = Occupation::from_char(chr)?;
                    i += 1;
                },
            }
            if i >= 64 { break }
        }

        // Group 2: Active color
        let game_state = match fen_groups.next().ok_or("Invalid FEN string: No active color.")? {
            "w" => GameState::WhiteToMove,
            "b" => GameState::BlackToMove,
            _ => return Err("Invalid FEN string: Failed to determine active color."),
        };

        // [TODO] Group 3: Castling rights
        // [TODO] Group 4: En passant square
        // [TODO] Group 5: Halfmove clock (needed for remis chess variant)
        // [TODO] Group 6: Fullmove number

        Ok( Self { board, game_state } )
    }
}

impl Default for ChessBoard {
    fn default() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_owned())
            .expect("Failed to generate default chess board from fen-string.")
    }
}

impl std::fmt::Display for ChessBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Display the board in the terminal.
        for i in 0..8 {
            for j in 0..8 {
                // set background color
                match (i + j) % 2 == 0 {
                    true  => write!(f, "\x1b[48;2;161;111;90m")?,
                    false => write!(f, "\x1b[48;2;235;211;184m")?,
                }
                // set foreground color
                match self.board[i * 8 + j] {
                    Occupation::Neutral => {},
                    Occupation::White(_) => {
                        write!(f, "\x1b[38;2;255;255;255m")?
                    },
                    Occupation::Black(_) => {
                        write!(f, "\x1b[38;2;0;0;0m")?
                    },
                }

                write!(f, "{}", self.board[i * 8 + j])?;
            }
            // newline, reset background color
            write!(f, "\n\x1b[0m")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    WhiteToMove,
    BlackToMove,
    WhiteWon,
    BlackWon,
    Tie,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Occupation {
    Neutral,
    White(ChessPiece),
    Black(ChessPiece),
}

impl Occupation {
    pub fn from_char(c: char) -> Result<Self, &'static str> {
        match c.is_uppercase() {
            true => Ok(Self::White(ChessPiece::from_char(c)?)),
            false => Ok(Self::Black(ChessPiece::from_char(c)?)),
        }
    }
}

impl std::fmt::Display for Occupation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Neutral => write!(f, "  "),
            Self::White(p) => write!(f, "{} ", p.get_working_version()),
            Self::Black(p) => write!(f, "{} ", p.get_working_version()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    pub fn get_white_version(&self) -> char {
        match self {
            Self::King   => '♔', // ♔	White King
            Self::Queen  => '♕', // ♕	White Queen
            Self::Rook   => '♖', // ♖	White Rook
            Self::Knight => '♘', // ♘	White Knight
            Self::Bishop => '♗', // ♗	White Bishop
            Self::Pawn   => '♙', // ♙	White Pawn
        }
    }

    pub fn get_black_version(&self) -> char {
        match self {
            Self::King   => '♚', // ♚	Black King
            Self::Queen  => '♛', // ♛	Black Queen
            Self::Rook   => '♜', // ♜	Black Rook
            Self::Knight => '♞', // ♞	Black Knight
            Self::Bishop => '♝', // ♝	Black Bishop
            Self::Pawn   => '♟', // ♟	Black Pawn
        }
    }

    pub fn get_working_version(&self) -> char {
        match self {
            Self::King   => '♚',
            Self::Queen  => '♛',
            Self::Rook   => '♜',
            Self::Knight => '♞',
            Self::Bishop => '♝',
            Self::Pawn   => '♙',
        }
    }
    
    fn from_char(c: char) -> Result<Self, &'static str> {
        match c.to_ascii_uppercase() {
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'R' => Ok(Self::Rook),
            'N' => Ok(Self::Knight),
            'B' => Ok(Self::Bishop),
            'P' => Ok(Self::Pawn),
            _ => Err("Invalid character."),
        }
    }
}
