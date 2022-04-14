mod grid_world;

#[cfg(test)]
mod tests {
	use value_iteration::MDP;

	use crate::grid_world::*;
    #[test]
    fn utilities() {
		let mdp = GridWorld::new();
		let space = Space::new(1);

		let mut visited = Vec::new();
		visited.push(space);
		let utility = mdp.utility(space, GAMMA, &mut visited);
		assert_eq!(utility, 0.0);
    }
}
