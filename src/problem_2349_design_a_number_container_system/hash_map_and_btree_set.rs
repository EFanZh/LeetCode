// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::Entry;
use std::collections::{BTreeSet, HashMap};

pub struct NumberContainers {
    index_to_number: HashMap<i32, i32>,
    number_to_indices: HashMap<i32, BTreeSet<i32>>,
}

impl NumberContainers {
    fn new() -> Self {
        Self {
            index_to_number: HashMap::new(),
            number_to_indices: HashMap::new(),
        }
    }

    fn change(&mut self, index: i32, number: i32) {
        if let Some(old_number) = self.index_to_number.insert(index, number) {
            self.number_to_indices.get_mut(&old_number).unwrap().remove(&index);
        }

        match self.number_to_indices.entry(number) {
            Entry::Occupied(occupied_entry) => {
                occupied_entry.into_mut().insert(index);
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(BTreeSet::from([index]));
            }
        }
    }

    fn find(&self, number: i32) -> i32 {
        self.number_to_indices
            .get(&number)
            .and_then(|indices| indices.first().copied())
            .unwrap_or(-1)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::NumberContainers for NumberContainers {
    fn new() -> Self {
        Self::new()
    }

    fn change(&mut self, index: i32, number: i32) {
        self.change(index, number);
    }

    fn find(&self, number: i32) -> i32 {
        self.find(number)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::NumberContainers>();
    }
}
