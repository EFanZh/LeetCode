pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

#[derive(Default)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    length: usize,
    value: Option<String>,
}

impl Solution {
    fn build_trie(words: Vec<String>) -> TrieNode {
        let mut trie = TrieNode::default();

        for word in words {
            let mut node = &mut trie;

            node.length += 1;

            for character in word.bytes() {
                node = node.children[usize::from(character - b'a')].get_or_insert_with(Box::default);
                node.length += 1;
            }

            node.value = Some(word);
        }

        trie
    }

    fn dfs(board: &mut [Vec<char>], row: usize, column: usize, parent: &mut TrieNode, result: &mut Vec<String>) {
        if let Some(cell) = board.get_mut(row).and_then(|r| r.get_mut(column)) {
            if *cell != '*' {
                let child = &mut parent.children[usize::from(*cell as u8 - b'a')];

                if let Some(trie) = child.as_deref_mut() {
                    let original_num_result = result.len();

                    if let Some(value) = trie.value.take() {
                        result.push(value);
                    }

                    let saved = mem::replace(cell, '*');

                    Self::dfs(board, row.wrapping_sub(1), column, trie, result);
                    Self::dfs(board, row, column.wrapping_sub(1), trie, result);
                    Self::dfs(board, row, column + 1, trie, result);
                    Self::dfs(board, row + 1, column, trie, result);

                    if trie.length == 0 {
                        *child = None;
                    }

                    parent.length -= result.len() - original_num_result;

                    board[row][column] = saved;
                }
            }
        }
    }

    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie = Self::build_trie(words);
        let rows = board.len();
        let columns = board.first().map_or(0, Vec::len);
        let mut result = Vec::new();

        for row in 0..rows {
            for column in 0..columns {
                Self::dfs(&mut board, row, column, &mut trie, &mut result);
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        Self::find_words(board, words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
