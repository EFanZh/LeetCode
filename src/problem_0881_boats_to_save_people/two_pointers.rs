pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut people = people;

        people.sort_unstable();

        let mut result = 0;
        let mut iter = people.iter().copied();

        'outer: while let Some(left) = iter.next() {
            loop {
                result += 1;

                if let Some(right) = iter.next_back() {
                    if left + right <= limit {
                        break;
                    }
                } else {
                    break 'outer;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        Self::num_rescue_boats(people, limit)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
