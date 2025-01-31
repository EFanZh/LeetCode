pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

struct Node {
    count: u32,
    children: [u32; 26],
}

impl Solution {
    pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        let mut allocator = Vec::<Node>::with_capacity(words.len() * 2);
        let mut allocator_len = 0;
        let mut root = [u32::MAX; 26];

        for word in &words {
            let mut node = &mut root;

            for c in word.bytes() {
                let c = usize::from(c - b'a');
                let mut child_index = node[c] as usize;

                if child_index < allocator_len {
                    allocator[child_index].count += 1;
                } else {
                    node[c] = allocator_len as _;
                    child_index = allocator_len;

                    allocator.push(Node {
                        count: 1,
                        children: [u32::MAX; 26],
                    });

                    allocator_len += 1;
                };

                node = &mut allocator[child_index].children;
            }
        }

        words
            .into_iter()
            .map(|word| {
                let mut count = 0;
                let mut node = &root;

                for c in word.into_bytes() {
                    let child = &allocator[node[usize::from(c) - usize::from(b'a')] as usize];

                    count += child.count;
                    node = &child.children;
                }

                count as _
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        Self::sum_prefix_scores(words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
