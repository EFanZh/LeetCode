pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::str;

const MAX_NAME_LENGTH: usize = 23;

#[derive(Clone)]
struct Name {
    buffer: [u8; MAX_NAME_LENGTH],
    length: u8,
}

impl Name {
    fn new(name: &[u8]) -> Self {
        let n = name.len();

        assert!(n <= MAX_NAME_LENGTH);

        let mut result = Self {
            buffer: [0; MAX_NAME_LENGTH],
            length: n as _,
        };

        result.buffer[..n].copy_from_slice(name);

        result
    }

    fn as_slice(&self) -> &[u8] {
        &self.buffer[..usize::from(self.length)]
    }
}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for Name {}

impl Hash for Name {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
        self.length.hash(state);
    }
}

impl Solution {
    pub fn get_folder_names(names: Vec<String>) -> Vec<String> {
        use std::io::Write;

        let mut names = names;
        let mut used = HashMap::<Name, u32>::new();

        for name in &mut names {
            let stack_name = Name::new(name.as_bytes());

            match used.entry(stack_name.clone()) {
                Entry::Occupied(entry) => {
                    let mut temp_stack_name = stack_name.clone();
                    let mut i = *entry.get();
                    let mut base_length = usize::from(temp_stack_name.length);

                    temp_stack_name.buffer[base_length] = b'(';

                    base_length += 1;

                    loop {
                        let mut writer = &mut temp_stack_name.buffer[base_length..];

                        write!(writer, "{i})").unwrap();

                        temp_stack_name.length = (MAX_NAME_LENGTH - writer.len()) as _;

                        match used.entry(temp_stack_name.clone()) {
                            Entry::Occupied(_) => i += 1,
                            Entry::Vacant(entry) => {
                                entry.insert(1);

                                break;
                            }
                        }
                    }

                    used.insert(stack_name, i);

                    name.push_str(
                        str::from_utf8(&temp_stack_name.buffer[base_length - 1..usize::from(temp_stack_name.length)])
                            .unwrap(),
                    );
                }
                Entry::Vacant(entry) => {
                    entry.insert(1);
                }
            }
        }

        names
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_folder_names(names: Vec<String>) -> Vec<String> {
        Self::get_folder_names(names)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
