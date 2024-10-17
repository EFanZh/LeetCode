pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        let mut degrees = vec![0_u16; n as u32 as usize];

        for road in roads {
            let [from, to] = <[_; 2]>::map(road.try_into().ok().unwrap(), |x| x as u32 as usize);

            degrees[from] += 1;
            degrees[to] += 1;
        }

        degrees.sort_unstable();

        (1..).zip(degrees).map(|(i, x)| i * i64::from(x)).sum()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        Self::maximum_importance(n, roads)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
