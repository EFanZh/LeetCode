pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn dfs(arr: &[i32], d: usize, node: usize, cache: &mut [u32]) -> u32 {
        let candidate = cache[node];

        if candidate == 0 {
            let height = arr[node];
            let mut result = 0;

            for (i, &left_height) in arr.iter().enumerate().take(node).rev().take(d) {
                if left_height < height {
                    result = result.max(Self::dfs(arr, d, i, cache));
                } else {
                    break;
                }
            }

            for (i, &right_height) in arr.iter().enumerate().skip(node + 1).take(d) {
                if right_height < height {
                    result = result.max(Self::dfs(arr, d, i, cache));
                } else {
                    break;
                }
            }

            result += 1;

            cache[node] = result;

            result
        } else {
            candidate
        }
    }

    pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        let n = arr.len();
        let d = d as usize;
        let mut cache = vec![0; n];
        let mut result = 0;

        for node in 0..n {
            result = result.max(Self::dfs(&arr, d, node, &mut cache));
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        Self::max_jumps(arr, d)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
