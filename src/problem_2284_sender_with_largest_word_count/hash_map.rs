pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;

impl Solution {
    pub fn largest_word_count(messages: Vec<String>, senders: Vec<String>) -> String {
        let mut counts = HashMap::new();

        messages.into_iter().zip(senders).for_each(|(message, sender)| {
            let count = message.bytes().fold(1, |count, c| count + u32::from(c == b' '));

            drop(message);

            match counts.entry(sender) {
                Entry::Occupied(occupied_entry) => *occupied_entry.into_mut() += count,
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(count);
                }
            }
        });

        counts
            .into_iter()
            .map(|(sender, count)| (count, sender))
            .max()
            .unwrap()
            .1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn largest_word_count(messages: Vec<String>, senders: Vec<String>) -> String {
        Self::largest_word_count(messages, senders)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
