pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn dfs(graph: &[u32; 26], group: u8, node: u8, mappings: &mut [u8; 26]) {
        let mut neighbors = graph[usize::from(node)];

        while neighbors != 0 {
            let neighbor = neighbors.trailing_zeros() as u8;

            if let mapping @ 0 = &mut mappings[usize::from(neighbor)] {
                *mapping = group;

                Self::dfs(graph, group, neighbor, mappings);
            }

            neighbors &= neighbors - 1;
        }
    }

    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let mut graph = [0_u32; 26];

        for (lhs, rhs) in s1.bytes().zip(s2.bytes()) {
            graph[usize::from(lhs) - usize::from(b'a')] |= 1 << (rhs - b'a');
            graph[usize::from(rhs) - usize::from(b'a')] |= 1 << (lhs - b'a');
        }

        let mut mappings = [0; 26];

        for node in 0..26 {
            if let mapping @ 0 = &mut mappings[usize::from(node)] {
                let group = b'a' + node;

                *mapping = group;

                Self::dfs(&graph, group, node, &mut mappings);
            }
        }

        let mut result = base_str.into_bytes();

        for c in &mut result {
            *c = mappings[usize::from(*c) - usize::from(b'a')];
        }

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        Self::smallest_equivalent_string(s1, s2, base_str)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
