pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn edge_score(edges: Vec<i32>) -> i32 {
        let mut sums = vec![0_u32; edges.len()].into_boxed_slice();

        (0..).zip(edges).for_each(|(from, to)| sums[to as u32 as usize] += from);

        let mut result = 0;
        let mut max_sum = 0;

        (0..).zip(&*sums).for_each(|(i, &sum)| {
            if sum > max_sum {
                max_sum = sum;
                result = i;
            }
        });

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn edge_score(edges: Vec<i32>) -> i32 {
        Self::edge_score(edges)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
