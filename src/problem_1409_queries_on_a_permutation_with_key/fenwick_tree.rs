pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn add(tree: &mut [u16], mut index: usize, diff: u16) {
        while let Some(value) = tree.get_mut(index) {
            *value = value.wrapping_add(diff);
            index |= index + 1;
        }
    }

    fn sum_less_than(tree: &[u16], mut x: usize) -> u16 {
        let mut result = 0;

        while let Some(&value) = tree.get(x.wrapping_sub(1)) {
            result += value;
            x &= x - 1;
        }

        result
    }

    pub fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
        let mut queries = queries;
        let n = queries.len();
        let m = m as u32 as usize;
        let mut buffer = vec![0; n + m * 2].into_boxed_slice();
        let (value_indices, tree) = buffer.split_at_mut(m);

        for (index, slot) in (n + 1..).zip(&mut *value_indices) {
            Self::add(tree, index, 1);
            *slot = index as _;
        }

        let mut new_index = n;

        for slot in &mut queries {
            let query_index_in_tree = &mut value_indices[*slot as u32 as usize - 1];

            *slot = i32::from(Self::sum_less_than(tree, usize::from(*query_index_in_tree)));

            Self::add(tree, usize::from(*query_index_in_tree), u16::MAX);
            Self::add(tree, new_index, 1);

            *query_index_in_tree = new_index as _;

            new_index -= 1;
        }

        queries
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
        Self::process_queries(queries, m)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
