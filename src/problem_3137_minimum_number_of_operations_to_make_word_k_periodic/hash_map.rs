pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::num::NonZeroUsize;

impl Solution {
    pub fn minimum_operations_to_make_k_periodic(word: String, k: i32) -> i32 {
        let k = NonZeroUsize::new(k.cast_unsigned() as _).unwrap();
        let mut counts = HashMap::<_, u32>::new();

        word.as_bytes()
            .chunks_exact(k.get())
            .for_each(|chunk| match counts.entry(chunk) {
                Entry::Occupied(occupied_entry) => *occupied_entry.into_mut() += 1,
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(1);
                }
            });

        ((word.len() / k) as u32 - counts.values().copied().fold(0, u32::max)).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_operations_to_make_k_periodic(word: String, k: i32) -> i32 {
        Self::minimum_operations_to_make_k_periodic(word, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
