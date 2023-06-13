pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn most_visited(n: i32, rounds: Vec<i32>) -> Vec<i32> {
        let start = *rounds.first().unwrap();
        let end = *rounds.last().unwrap();
        let mut result = rounds;

        result.clear();

        if start <= end {
            result.extend(start..=end);
        } else {
            result.extend((1..=end).chain(start..=n));
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn most_visited(n: i32, rounds: Vec<i32>) -> Vec<i32> {
        Self::most_visited(n, rounds)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
