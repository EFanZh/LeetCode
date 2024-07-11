pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn find_root(union_find: &mut [(u32, u32)], node: u32) -> (u32, u32) {
        let state = union_find[node as usize];

        if state.0 == 0 {
            (node, state.1)
        } else {
            let result = Self::find_root(union_find, state.0 - 1);

            if state.0 != result.0 + 1 {
                union_find[node as usize].0 = result.0 + 1;
            }

            result
        }
    }

    fn union(union_find: &mut [(u32, u32)], left: u32, right: u32) {
        let left_root = Self::find_root(union_find, left);
        let right_root = Self::find_root(union_find, right);

        if left_root.0 != right_root.0 {
            let (child, parent) = if left_root.1 < right_root.1 {
                (left_root.0, right_root.0)
            } else {
                if left_root.1 == right_root.1 {
                    union_find[left_root.0 as usize].1 += 1;
                }

                (right_root.0, left_root.0)
            };

            union_find[child as usize].0 = parent + 1;
        }
    }

    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let source = source as u32;
        let destination = destination as u32;
        let mut union_find = vec![(0, 0); n as u32 as usize].into_boxed_slice();
        let mut iter = edges.into_iter();

        while Self::find_root(&mut union_find, source) != Self::find_root(&mut union_find, destination) {
            if let Some(edge) = iter.next() {
                let [from, to]: [_; 2] = edge.try_into().ok().unwrap();

                Self::union(&mut union_find, from as _, to as _);
            } else {
                return false;
            }
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        Self::valid_path(n, edges, source, destination)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
