use std::hash::Hash;
use std::ops::Add;

/// Vague definition of an MDP
pub trait MDP<A, S>
where
	A: Action,
	S: State<A>
{
	/// Determines a reward based on current state, action, and next state
	fn reward(&self, state: &S, action: &A, next_state: &S) -> f64;

	/// Transition function from one state to another, given a state and action
	/// Returns a map of next states and their probabilities
	fn transition(&self, state: &S, action: &A) -> Vec<(S, f64)>;

	/// How good is this state?
	/// Does not allow revisiting states
	fn utility(&self, state: S, gamma: f64, visited: &mut Vec<S>) -> f64 {
		let actions = state.possible_actions();

		// Bellman equation time
		set_max(actions, |action| {
			// Go through each action, and find the highest valued one
			let next_states = self.transition(&state, &action);

			set_sum(next_states, |(next_state, probability)| {
				// Add up each possible value of next states, weighted by their
				// probability, given an action
				let reward = self.reward(&state, &action, &next_state);

				visited.push(state);
				// println!("At state {state:?}");

				if next_state.is_final() {
					probability * reward
				} else if visited.contains(&next_state) {
					// Remove this from the list since it isn't in our path anymore
					if let Some(pos) = visited.iter().position(|x| *x == state) {
						visited.remove(pos);
					}
					probability * reward
				} else {
					let next_utility = gamma * self.utility(next_state, gamma, visited);
					probability * (reward + next_utility)
				}
			}).unwrap()
		}).unwrap()
	}
}

/// Action trait
pub trait Action: Copy + Eq + Hash + std::fmt::Debug {}

/// State trait
pub trait State<A: Action>: Copy + Eq + std::fmt::Debug {
	/// Whether or not this state is final
	fn is_final(&self) -> bool;

	/// Gets a set of possible actions from this state
	fn possible_actions(&self) -> Vec<A>;
}

/// Finds the element of the set which causes the highest function value
///
/// * `set` - A set of elements to find the max valued member of
/// * `value` - The function to determine the value of an individual element
/// * returns - Some(element) if there are any elements, None otherwise
fn set_max<E, F, V>(set: Vec<E>, mut value: F) -> Option<V>
where
	F: FnMut(E) -> V,
	V: Copy + PartialOrd
{
	let mut max = None;

	for element in set {
		// Find the value of this element
		let v = value(element);

		// Branch based on if a max is set
		match max {
			Some(max_v) => {
				// A max is set
				if v > max_v {
					// A new max was found
					max = Some(v);
				}
			},
			None => {
				// If no max is set, set it
				max = Some(v);
			}
		}
	}

	max
}

/// Sums up the value of each element in this set, given some value function
///
/// * `set` - A set of elements with some value
/// * `value` - The function to determine the value of an individual element
/// * returns - Some(value) if there are any elements, None otherwise
fn set_sum<E, F, V>(set: Vec<E>, mut value: F) -> Option<V>
where
	F: FnMut(E) -> V,
	V: Add<Output = V>
{
	let mut sum = None;

	for element in set {
		// Find the value of this element
		let v = value(element);

		// Branch based on if sum is set yet
		match sum {
			Some(s) => {
				// Sum is set
					sum = Some(s + v);
			},
			None => {
				// If no sum is set
				// Set it to the value of the current element
				sum = Some(v);
			}
		}
	}

	sum
}
