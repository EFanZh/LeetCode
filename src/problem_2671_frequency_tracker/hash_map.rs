// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub struct FrequencyTracker {
    // Maps each number to its frequency count minus one.
    frequencies: HashMap<i32, u32>,

    // Stores the counts of frequencies. The index represents the frequency count minus one.
    frequency_counts: Vec<u32>,
}

impl FrequencyTracker {
    fn new() -> Self {
        Self {
            frequencies: HashMap::new(),
            frequency_counts: Vec::new(),
        }
    }

    fn add(&mut self, number: i32) {
        let frequency = *match self.frequencies.entry(number) {
            Entry::Occupied(occupied_entry) => {
                let frequency = occupied_entry.into_mut();

                self.frequency_counts[*frequency as usize] -= 1;

                *frequency += 1;

                frequency
            }
            Entry::Vacant(vacant_entry) => vacant_entry.insert(0),
        } as usize;

        if let Some(count) = self.frequency_counts.get_mut(frequency) {
            *count += 1;
        } else {
            self.frequency_counts.push(1);
        }
    }

    fn delete_one(&mut self, number: i32) {
        match self.frequencies.entry(number) {
            Entry::Occupied(mut occupied_entry) => {
                let frequency = occupied_entry.get_mut();
                let old_frequency = *frequency;

                if let Some(count) = self.frequency_counts.get_mut((*frequency as usize).wrapping_sub(1)) {
                    *count += 1;
                    *frequency -= 1;
                } else {
                    occupied_entry.remove();
                }

                self.frequency_counts[old_frequency as usize] -= 1;
            }
            Entry::Vacant(_) => {}
        }
    }

    fn has_frequency(&self, frequency: i32) -> bool {
        self.frequency_counts
            .get((frequency as u32 as usize).wrapping_sub(1))
            .is_some_and(|&count| count != 0)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::FrequencyTracker for FrequencyTracker {
    fn new() -> Self {
        Self::new()
    }

    fn add(&mut self, number: i32) {
        self.add(number);
    }

    fn delete_one(&mut self, number: i32) {
        self.delete_one(number);
    }

    fn has_frequency(&self, frequency: i32) -> bool {
        self.has_frequency(frequency)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::FrequencyTracker>();
    }
}
