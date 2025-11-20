pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::mem;

impl Solution {
    fn helper(queries: Vec<Vec<i32>>, mut set_color: impl FnMut(i32, i32) -> i32) -> Vec<i32> {
        let mut color_counts = HashMap::new();

        queries
            .into_iter()
            .map(|query| {
                let [index, color] = query.try_into().ok().unwrap();
                let old_color = set_color(index, color);

                if color != old_color {
                    if let Entry::Occupied(entry) = color_counts.entry(old_color) {
                        if *entry.get() == 0 {
                            entry.remove();
                        } else {
                            *entry.into_mut() -= 1;
                        }
                    }

                    match color_counts.entry(color) {
                        Entry::Occupied(occupied_entry) => *occupied_entry.into_mut() += 1,
                        Entry::Vacant(vacant_entry) => {
                            vacant_entry.insert(0);
                        }
                    }
                }

                color_counts.len() as _
            })
            .collect()
    }

    pub fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let limit = limit.cast_unsigned() as usize;

        if limit < queries.len() {
            let mut colors = vec![0; limit + 1].into_boxed_slice();

            Self::helper(queries, |index, color| {
                mem::replace(&mut colors[index.cast_unsigned() as usize], color)
            })
        } else {
            let mut colors = HashMap::new();

            Self::helper(queries, |index, color| colors.insert(index, color).unwrap_or(0))
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        Self::query_results(limit, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
