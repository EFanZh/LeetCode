pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryFrom;

impl Solution {
    pub fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut graph = vec![Vec::new(); n];

        for path in paths {
            let [x, y] = <[_; 2]>::try_from(path).unwrap();
            let x = x as usize - 1;
            let y = y as usize - 1;

            graph[x].push(y);
            graph[y].push(x);
        }

        let mut result = vec![0; n];

        for (i, neighbors) in graph.iter().enumerate() {
            let mut state = 0_u8;

            for &neighbor in neighbors {
                state |= 1 << result[neighbor];
            }

            result[i] = (state >> 1).trailing_ones() as i32 + 1;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32> {
        Self::garden_no_adj(n, paths)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
