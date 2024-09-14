pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn dig_artifacts(n: i32, artifacts: Vec<Vec<i32>>, dig: Vec<Vec<i32>>) -> i32 {
        let n = n as u32 as usize;
        let mut grid = vec![false; n * n].into_boxed_slice();

        for position in dig {
            let [r, c] = <[_; 2]>::map(position.try_into().ok().unwrap(), |x| x as u32 as usize);

            grid[n * r + c] = true;
        }

        artifacts
            .iter()
            .map(|artifact| {
                let [r0, c0, r1, c1] =
                    <[_; 4]>::map(artifact.as_slice().try_into().ok().unwrap(), |x| x as u32 as usize);

                i32::from((r0..=r1).all(|row| grid[n * row + c0..=n * row + c1].iter().all(|&x| x)))
            })
            .sum()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn dig_artifacts(n: i32, artifacts: Vec<Vec<i32>>, dig: Vec<Vec<i32>>) -> i32 {
        Self::dig_artifacts(n, artifacts, dig)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
