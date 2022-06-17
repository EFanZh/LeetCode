pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let mut start = start as usize;
        let mut destination = destination as usize;

        if destination < start {
            mem::swap(&mut start, &mut destination);
        }

        let (left, rest) = distance.split_at(start);
        let (middle, right) = rest.split_at(destination - start);
        let candidate_1 = left.iter().copied().sum::<i32>() + right.iter().copied().sum::<i32>();
        let candidate_2 = middle.iter().copied().sum::<i32>();

        candidate_1.min(candidate_2)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        Self::distance_between_bus_stops(distance, start, destination)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
