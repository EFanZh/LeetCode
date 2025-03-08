pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut states = HashMap::new();
        let mut all_players = HashSet::new();

        for match_ in matches {
            let [winner, loser] = match_.try_into().ok().unwrap();
            let (winner, loser) = (winner as u32, loser as u32);

            all_players.extend([winner, loser]);

            match states.entry(loser) {
                Entry::Occupied(entry) => *entry.into_mut() = true,
                Entry::Vacant(entry) => {
                    entry.insert(false);
                }
            }
        }

        let mut all_players = all_players.into_iter().collect::<Box<_>>();

        all_players.sort_unstable();

        let mut no_loses = Vec::new();
        let mut one_loses = Vec::new();

        for &player in &*all_players {
            match states.get(&player) {
                None => no_loses.push(player as i32),
                Some(false) => one_loses.push(player as i32),
                Some(true) => {}
            }
        }

        vec![no_loses, one_loses]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::find_winners(matches)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
