pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut paths = paths;
        let mut destinations = paths.iter_mut().filter_map(Vec::pop).collect::<HashSet<_>>();

        for source in paths.into_iter().flatten() {
            destinations.remove(source.as_str());
        }

        destinations.into_iter().next().unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn dest_city(paths: Vec<Vec<String>>) -> String {
        Self::dest_city(paths)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
