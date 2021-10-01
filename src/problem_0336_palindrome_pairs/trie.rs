pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

#[derive(Default)]
struct Node {
    children: [Option<Box<Node>>; 26],
    value: Option<i32>,
}

impl Solution {
    fn is_palindrome(s: &[u8]) -> bool {
        let mut iter = s.iter();

        while let Some(lhs) = iter.next() {
            if let Some(rhs) = iter.next_back() {
                if lhs != rhs {
                    return false;
                }
            } else {
                break;
            }
        }

        true
    }

    fn find_index(mut node: &Node, word: impl Iterator<Item = u8>) -> Option<i32> {
        for c in word {
            if let Some(next) = node.children[usize::from(c - b'a')].as_deref() {
                node = next;
            } else {
                return None;
            }
        }

        node.value
    }

    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut trie = Node::default();

        for (i, word) in (0..).zip(words.iter().map(String::as_bytes)) {
            let mut node = &mut trie;

            for c in word {
                node = node.children[usize::from(c - b'a')].get_or_insert_with(Box::default);
            }

            node.value = Some(i);
        }

        let mut result = Vec::new();

        for (i, word) in (0..).zip(words.iter().map(String::as_bytes)) {
            if let Some(j) = Self::find_index(&trie, word.iter().rev().copied()) {
                if j > i {
                    result.push(vec![i, j]);
                    result.push(vec![j, i]);
                }
            }

            for length in 0..word.len() {
                let (left, right) = word.split_at(length);

                if Self::is_palindrome(right) {
                    if let Some(j) = Self::find_index(&trie, left.iter().rev().copied()) {
                        result.push(vec![i, j]);
                    }
                }

                let (left, right) = word.split_at(word.len() - length);

                if Self::is_palindrome(left) {
                    if let Some(j) = Self::find_index(&trie, right.iter().rev().copied()) {
                        result.push(vec![j, i]);
                    }
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        Self::palindrome_pairs(words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
