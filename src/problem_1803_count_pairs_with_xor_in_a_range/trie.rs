pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

struct Node {
    count: u16,
    zero: u16,
    one: u16,
}

impl Solution {
    fn count_less_than(nodes: &[Node], node: &Node, high: u16, num: u16) -> u16 {
        let mut node = node;
        let mut result = 0;
        let mut probe = 1 << 14;

        loop {
            let (zero_index, one_index) = if num & probe == 0 {
                (node.zero, node.one)
            } else {
                (node.one, node.zero)
            };

            let child_index = if high & probe == 0 {
                zero_index
            } else {
                result += nodes.get(usize::from(zero_index)).map_or(0, |node| node.count);

                one_index
            };

            if let Some(child) = nodes.get(usize::from(child_index)) {
                node = child;
            } else {
                break;
            }

            probe >>= 1;
        }

        result
    }

    fn trie_insert(nodes: &mut Vec<Node>, node: &mut Node, num: u16) {
        let mut node = node;
        let mut probe = 1 << 14;
        let mut nodes_count = nodes.len() as u16;

        loop {
            node.count += 1;

            let child_index = if num & probe == 0 {
                &mut node.zero
            } else {
                &mut node.one
            };

            let child_index = if *child_index < nodes_count {
                *child_index
            } else {
                let new_child_index = nodes_count;

                *child_index = new_child_index;
                nodes_count += 1;

                nodes.push(Node {
                    count: 0,
                    zero: u16::MAX,
                    one: u16::MAX,
                });

                new_child_index
            };

            node = &mut nodes[usize::from(child_index)];

            probe >>= 1;

            if probe == 0 {
                break;
            }
        }

        node.count += 1;
    }

    pub fn count_pairs(nums: Vec<i32>, low: i32, high: i32) -> i32 {
        let low = low as u16;
        let high = high as u16;
        let mut nodes = Vec::new();

        let mut root = Node {
            count: 0,
            zero: u16::MAX,
            one: u16::MAX,
        };

        let mut result = 0;

        for num in nums {
            let num = num as u16;

            result += u32::from(
                Self::count_less_than(&nodes, &root, high + 1, num) - Self::count_less_than(&nodes, &root, low, num),
            );

            Self::trie_insert(&mut nodes, &mut root, num);
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_pairs(nums: Vec<i32>, low: i32, high: i32) -> i32 {
        Self::count_pairs(nums, low, high)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
