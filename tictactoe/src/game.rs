use wasm_bindgen::prelude::*;
use open_ttt_lib::{ai, board, game};
use crate::utils::set_panic_hook;

/// The structure wrapping [`game::Game`], which handles all the logic
///
/// This wrapper exists so that we have an `impl` block that we can decorate
/// with the `#[wasm_bindgen]` attribute
#[wasm_bindgen]
pub struct Game(game::Game);

/// Return a character describing the player owning a particular board field
///
/// Since we cannot return Rust enum instances across the WASM boundary,
/// we map the values to [`char`]s (and later map the chars to [`u8`]s,
/// which we can return)
fn player_char(owner: board::Owner) -> char {
    match owner {
        board::Owner::PlayerO => 'O',
        board::Owner::PlayerX => 'X',
        board::Owner::None => '.',
    }
}

#[wasm_bindgen]
impl Game {
    /// Create a new instance
    ///
    /// We use the opportunity to call `set_panic_hook` so that any panics
    /// end up in the browser console
    pub fn new() -> Self {
        set_panic_hook();
        Self(game::Game::new())
    }

    /// Place a marker for the current player on the (row, column) field
    ///
    /// We cannot pass a [`Result`] across the WASM boundary, but the only
    /// possible error is that the board is already full, so a bool is sufficient
    ///
    /// For more expressive error reporting, we could return a struct
    pub fn do_move(&mut self, row: i32, column: i32) -> bool {
        let position = board::Position { row, column };
        self.0.do_move(position).is_ok()
    }

    /// Choose a move with AI and place it on the board
    ///
    /// `difficulty` is a byte that maps to [`ai::Difficulty`] enum values.
    /// Invalid difficulty values result in an error (false return value)
    pub fn do_ai_move(&mut self, difficulty: u8) -> bool {
        let ai_level = match difficulty {
            0 => ai::Difficulty::Easy,
            1 => ai::Difficulty::Medium,
            2 => ai::Difficulty::Hard,
            // 3 => ai::Difficulty::Unbeatable,
            _ => return false,
        };

        let opponent = ai::Opponent::new(ai_level);
        if let Some(ai_position) = opponent.get_move(&self.0) {
            self.do_move(ai_position.row, ai_position.column)
        } else {
            false
        }
    }

    /// Return a representation of the board as a vector of bytes
    ///
    /// The iterator from [`board::Board`] yields tuples of position (which we ignore)
    /// and owner (which we map to [`char`]s and then to [`u8`]s)
    ///
    /// Then we collect the resulting iterator over `u8`s to a vector and return it
    pub fn get_board(&self) -> Vec<u8> {
        self.0.board().iter().map(|(_, owner)| player_char(owner) as u8).collect()
    }

    /// Return the current game state
    ///
    /// Just map the enum returned from [`game::Game::state`] to a string
    pub fn get_state(&self) -> String {
        let s = match self.0.state() {
            game::State::PlayerOMove => "Player O moves",
            game::State::PlayerXMove => "Player X moves",
            game::State::PlayerOWin(_) => "Player O wins",
            game::State::PlayerXWin(_) => "Player X wins",
            game::State::CatsGame => "Tie",
        };
        s.to_string()
    }

    /// Is the game over? (i.e. are there no moves possible)
    pub fn game_over(&self) -> bool {
        match self.0.state() {
            game::State::PlayerOMove | game::State::PlayerXMove => false,
            _ => true,
        }
    }
}