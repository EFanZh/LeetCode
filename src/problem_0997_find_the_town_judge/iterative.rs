pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let n = n as u16;
        let mut degrees = vec![0_u16; usize::from(n)];

        for item in trust {
            let [from, to]: [_; 2] = item.try_into().unwrap();
            let (from, to) = (from as usize - 1, to as usize - 1);

            degrees[from] = u16::MAX;

            let degree = &mut degrees[to];

            *degree = degree.saturating_add(1);
        }

        (1..)
            .zip(degrees)
            .find_map(|(i, degree)| (degree == n - 1).then(|| i))
            .unwrap_or(-1)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        Self::find_judge(n, trust)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
