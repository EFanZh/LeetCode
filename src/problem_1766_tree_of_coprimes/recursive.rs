pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

struct Context<'a> {
    nums: &'a [i32],
    graph: &'a [Vec<u32>],
    ancestors: [(u32, u32); 50],
    result: Vec<i32>,
}

impl Solution {
    fn is_coprime(mut x: u32, mut y: u32) -> bool {
        while y != 0 {
            let z = x % y;

            x = y;
            y = z;
        }

        x == 1
    }

    fn dfs(context: &mut Context, depth: u32, parent: u32, node: u32) {
        let num = context.nums[node as usize] as u32;
        let mut max_depth = 0;
        let mut max_index = u32::MAX;

        for (ancestor_num, &(ancestor_depth, ancestor_index)) in (1..).zip(&context.ancestors) {
            if Self::is_coprime(ancestor_num, num) && ancestor_depth > max_depth {
                max_depth = ancestor_depth;
                max_index = ancestor_index;
            }
        }

        context.result[node as usize] = max_index as _;

        let old_ancestor = mem::replace(&mut context.ancestors[num as usize - 1], (depth, node));

        for &child in &context.graph[node as usize] {
            if child != parent {
                Self::dfs(context, depth + 1, node, child);
            }
        }

        context.ancestors[num as usize - 1] = old_ancestor;
    }

    pub fn get_coprimes(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let mut graph = vec![Vec::new(); n].into_boxed_slice();

        for edge in edges {
            let [from, to]: [_; 2] = edge.try_into().ok().unwrap();
            let from = from as u32;
            let to = to as u32;

            graph[from as usize].push(to);
            graph[to as usize].push(from);
        }

        let mut context = Context {
            nums: &nums,
            graph: &graph,
            ancestors: [(0, 0); 50],
            result: vec![0; n],
        };

        Self::dfs(&mut context, 1, u32::MAX, 0);

        context.result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_coprimes(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> Vec<i32> {
        Self::get_coprimes(nums, edges)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
