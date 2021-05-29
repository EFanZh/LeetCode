pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::rc::Rc;

#[derive(Default, Clone)]
struct Node {
    children: [Option<Rc<Node>>; 26],
}

impl Solution {
    pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
        let mut root = Node::default();

        for c in s.bytes().rev() {
            let mut new_root = root.clone();

            new_root.children[usize::from(c - b'a')] = Some(Rc::new(root));

            root = new_root;
        }

        let mut result = String::new();

        for word in dictionary {
            if (word.len() == result.len() && word < result) || word.len() > result.len() {
                let mut node = &root;
                let mut iter = word.bytes();

                loop {
                    if let Some(c) = iter.next() {
                        if let Some(child) = node.children[usize::from(c - b'a')].as_deref() {
                            node = child;
                        } else {
                            break;
                        }
                    } else {
                        result = word;

                        break;
                    }
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
        Self::find_longest_word(s, dictionary)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
