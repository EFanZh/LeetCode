pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;
use std::mem;

struct Context<'a> {
    graph: Box<[(&'a mut u32, Vec<u32>)]>,
    values: HashSet<u32>,
}

impl Solution {
    fn dfs(context: &mut Context, node: u32) {
        let (value, children) = &mut context.graph[node as usize];

        context.values.insert(**value);

        **value = 1;

        for child in mem::take(children) {
            Self::dfs(context, child);
        }
    }

    fn helper(parents: Vec<u32>, nums: Vec<u32>) -> Vec<u32> {
        let mut nums = nums;

        if let Some(mut node) = nums.iter().position(|&num| num == 1) {
            let mut context = Context {
                graph: nums.iter_mut().map(|num| (num, Vec::new())).collect(),
                values: HashSet::from([1]),
            };

            for (child, &parent) in (0..).zip(&parents) {
                if let Some((_, children)) = context.graph.get_mut(parent as usize) {
                    children.push(child);
                }
            }

            for child in mem::take(&mut context.graph[node].1) {
                Self::dfs(&mut context, child);
            }

            let mut min_missing = 2;

            loop {
                while context.values.contains(&min_missing) {
                    min_missing += 1;
                }

                *context.graph[node].0 = min_missing;

                let parent = parents[node] as usize;

                if let Some((target, siblings)) = context.graph.get_mut(parent) {
                    context.values.insert(**target);

                    for sibling in mem::take(siblings) {
                        if sibling != node as u32 {
                            Self::dfs(&mut context, sibling);
                        }
                    }

                    node = parent;
                } else {
                    break;
                }
            }
        } else {
            nums.fill(1);
        }

        nums
    }

    pub fn smallest_missing_value_subtree(parents: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        Self::helper(
            parents.into_iter().map(|x| x as _).collect(),
            nums.into_iter().map(|x| x as _).collect(),
        )
        .into_iter()
        .map(|x| x as _)
        .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn smallest_missing_value_subtree(parents: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        Self::smallest_missing_value_subtree(parents, nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
