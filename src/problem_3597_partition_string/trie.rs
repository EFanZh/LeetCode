pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

struct Node {
    children: [u32; 26],
}

impl Solution {
    pub fn partition_string(s: String) -> Vec<String> {
        let mut root = Node {
            children: [u32::MAX; 26],
        };

        let mut buffer = Vec::<Node>::new();
        let mut buffer_len = buffer.len();
        let mut result = Vec::new();
        let mut current = &mut root;
        let mut start = 0;

        for (i, c) in s.bytes().enumerate() {
            let child_index = &mut current.children[usize::from(c) - usize::from(b'a')];
            let child_index_usize = *child_index as usize;

            current = if child_index_usize < buffer_len {
                &mut buffer[child_index_usize]
            } else {
                *child_index = buffer_len as _;
                buffer_len += 1;

                buffer.push(Node {
                    children: [u32::MAX; 26],
                });

                result.push(s[start..=i].to_string());
                start = i + 1;

                &mut root
            };
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn partition_string(s: String) -> Vec<String> {
        Self::partition_string(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
