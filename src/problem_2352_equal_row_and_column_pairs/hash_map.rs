pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::Entry;
use std::collections::HashMap;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut counts = HashMap::new();

        for row in &grid {
            match counts.entry(&**row) {
                Entry::Occupied(occupied_entry) => *occupied_entry.into_mut() += 1,
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(1);
                }
            }
        }

        let mut result = 0;
        let n = grid.len();
        let mut buffer = Vec::with_capacity(n);

        for x in 0..n {
            buffer.extend(grid.iter().map(|row| row[x]));

            if let Some(count) = counts.get(&*buffer) {
                result += count;
            }

            buffer.clear();
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        Self::equal_pairs(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
