pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::mem;

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut names = names;

        let mut sorting = names
            .iter_mut()
            .zip(heights)
            .map(|(name, height)| (height, mem::take(name)))
            .collect::<Vec<_>>();

        sorting.sort_unstable_by_key(|&(height, _)| Reverse(height as u32));

        for (target, (_, mut name)) in names.iter_mut().zip(sorting) {
            mem::swap(target, &mut name);
        }

        names
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        Self::sort_people(names, heights)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
