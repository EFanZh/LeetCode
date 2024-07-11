pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut targets = vec![false; n as u32 as _];

        for edge in edges {
            let [_, to]: [_; 2] = edge.try_into().ok().unwrap();

            targets[to as u32 as usize] = true;
        }

        (0..)
            .zip(targets)
            .filter_map(|(i, used)| if used { None } else { Some(i) })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        Self::find_smallest_set_of_vertices(n, edges)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
