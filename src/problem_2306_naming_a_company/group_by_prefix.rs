pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

impl Solution {
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        let mut groups = [(); 26].map(|()| HashSet::<u64>::new());

        for idea in ideas {
            let mut iter = idea.bytes();
            let prefix = usize::from(iter.next().unwrap());
            let suffix = iter.fold(0, |result, c| (result << 7) | u64::from(c));

            groups[prefix - usize::from(b'a')].insert(suffix);
        }

        let mut result = 0;
        let mut iter = groups.iter();

        while let Some(left) = iter.next() {
            if !left.is_empty() {
                iter.clone().for_each(|right| {
                    let intersections = left.intersection(right).count();

                    result += (left.len() - intersections) * (right.len() - intersections);
                });
            }
        }

        (result * 2) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn distinct_names(ideas: Vec<String>) -> i64 {
        Self::distinct_names(ideas)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
