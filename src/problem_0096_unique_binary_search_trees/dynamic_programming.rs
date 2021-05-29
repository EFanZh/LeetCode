pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let mut cache = vec![0; n + 1];

        cache[0] = 1;

        for i in 1..=n {
            let mut count = 0;

            for root in 0..i {
                let right_size = i - 1 - root;

                count += cache[root] * cache[right_size];
            }

            cache[i] = count;
        }

        cache[n]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_trees(n: i32) -> i32 {
        Self::num_trees(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
