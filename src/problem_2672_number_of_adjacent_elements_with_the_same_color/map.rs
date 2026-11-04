pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn color_the_array(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut map = vec![0; n.cast_unsigned() as usize].into_boxed_slice();
        let mut same_pairs = 0;

        queries
            .into_iter()
            .map(|query| {
                let [index, color] = query.try_into().ok().unwrap();
                let index = index.cast_unsigned() as usize;
                let left = map.get(index.wrapping_sub(1)).copied().unwrap_or(0);
                let right = map.get(index + 1).copied().unwrap_or(0);
                let middle = &mut map[index];

                if *middle != 0 {
                    same_pairs -= i32::from(*middle == left);
                    same_pairs -= i32::from(*middle == right);
                }

                *middle = color;
                same_pairs += i32::from(*middle == left);
                same_pairs += i32::from(*middle == right);

                same_pairs
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn color_the_array(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        Self::color_the_array(n, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
