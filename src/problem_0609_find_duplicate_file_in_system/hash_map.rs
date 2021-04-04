pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        let mut content_to_path = HashMap::<_, Vec<_>>::new();
        let mut path_buffer = String::new();

        for path in &paths {
            let mut path_iter = path.split(' ');
            let dir = path_iter.next().unwrap();

            path_buffer.push_str(dir);
            path_buffer.push('/');

            let dir_length = path_buffer.len();

            for file in path_iter {
                let mut file_iter = file.split(|c: char| c == '(' || c == ')');
                let file_name = file_iter.next().unwrap();
                let file_content = file_iter.next().unwrap();

                path_buffer.push_str(file_name);

                content_to_path
                    .entry(file_content)
                    .and_modify(|paths| paths.push(path_buffer.clone()))
                    .or_insert_with(|| vec![path_buffer.clone()]);

                path_buffer.truncate(dir_length);
            }

            path_buffer.clear();
        }

        content_to_path
            .into_iter()
            .filter_map(|(_, value)| if value.len() < 2 { None } else { Some(value) })
            .collect()
    }
}

impl super::Solution for Solution {
    fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        Self::find_duplicate(paths)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
