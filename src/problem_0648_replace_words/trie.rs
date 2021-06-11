pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

#[derive(Default)]
struct Node {
    children: [Option<Box<Self>>; 26],
    has_value: bool,
}

impl Solution {
    fn get_shortest_root<'a>(mut node: &Node, word: &'a str) -> &'a str {
        let mut i = 1;

        for c in word.bytes() {
            if let Some(child) = node.children[usize::from(c - b'a')].as_deref() {
                if child.has_value {
                    return &word[..i];
                }

                node = child;
                i += 1;
            } else {
                break;
            }
        }

        word
    }

    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut root = Node::default();

        for word in dictionary {
            let mut node = &mut root;

            for c in word.bytes() {
                node = node.children[usize::from(c - b'a')].get_or_insert_with(Box::default);
            }

            node.has_value = true;
        }

        let mut iter = sentence.split(' ').map(|word| Self::get_shortest_root(&root, word));
        let mut result = String::with_capacity(sentence.len());

        result.push_str(iter.next().unwrap());

        for word in iter {
            result.push(' ');
            result.push_str(word);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        Self::replace_words(dictionary, sentence)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
