use value_iteration::*;

pub const GAMMA: f64 = 1.0;
const R: f64 = -0.04;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Move {
	UP,
	RIGHT,
	DOWN,
	LEFT
}

impl Action for Move {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Space {
	pub n: u8
}

impl Space {
	/// Space constructor
	pub fn new(n: u8) -> Space {
		Space {n}
	}
}

impl State<Move> for Space {
	/// Whether or not this state is final
	fn is_final(&self) -> bool {
		match self.n {
			10 => true,
			11 => true,
			_ => false
		}
	}

	/// Gets a set of possible actions from this state
	fn possible_actions(&self) -> Vec<Move> {
		let mut moves = Vec::new();
		moves.push(Move::UP);
		moves.push(Move::RIGHT);
		moves.push(Move::DOWN);
		moves.push(Move::LEFT);

		moves
	}
}

fn match_up(n: u8) -> u8 {
	match n {
		1|2|6|7|9|10 => n+1,
		_ => n
	}
}

fn match_right(n: u8) -> u8 {
	match n {
		1|5|6|7|8 => n+3,
		3|4 => n+2,
		_ => n
	}
}

fn match_down(n: u8) -> u8 {
	match n {
		2|3|7|8|10|11 => n-1,
		_ => n
	}
}

fn match_left(n: u8) -> u8 {
	match n {
		4|8|9|10|11 => n-3,
		5|6 => n-2,
		_ => n
	}
}
pub struct GridWorld {}

impl GridWorld {
	pub fn new() -> GridWorld {
		GridWorld {}
	}
}

impl MDP<Move, Space> for GridWorld {
    /// Find the reward for being in this space
	fn reward(&self, _state: &Space, _action: &Move, next_state: &Space) -> f64 {
        match next_state.n {
			10 => -1.0,
			11 => 1.0,
			_ => R
		}
    }

    fn transition(&self, state: &Space, action: &Move) -> Vec<(Space, f64)> {
        let mut states = Vec::new();

		// Go through every single possibility
		match (state.n, action) {
			(10, _) => states.push((Space::new(10), 1.0)),
			(11, _) => states.push((Space::new(11), 1.0)),
			(n, Move::UP) => {
				states.push((Space::new(match_up(n)), 0.8));
				states.push((Space::new(match_left(n)), 0.1));
				states.push((Space::new(match_right(n)), 0.1));
			},
			(n, Move::RIGHT) => {
				states.push((Space::new(match_right(n)), 0.8));
				states.push((Space::new(match_left(n)), 0.1));
				states.push((Space::new(match_down(n)), 0.1));
			},
			(n, Move::DOWN) => {
				states.push((Space::new(match_down(n)), 0.8));
				states.push((Space::new(match_right(n)), 0.1));
				states.push((Space::new(match_left(n)), 0.1));
			},
			(n, Move::LEFT) => {
				states.push((Space::new(match_left(n)), 0.8));
				states.push((Space::new(match_down(n)), 0.1));
				states.push((Space::new(match_up(n)), 0.1));
			}
		}

		states
    }
}
