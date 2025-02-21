pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::{convert, str};

type Name = [u8; 10];

#[derive(Default)]
struct Node {
    children: HashMap<Name, Self>,
    signature_id: u32,
}

struct Context1 {
    signature_to_id: HashMap<Box<[(Name, u32)]>, u32>,
    id_to_duplication: Vec<bool>,
}

struct Context2<'a> {
    is_duplicated: Vec<bool>,
    path: Vec<&'a str>,
    result: Vec<Vec<String>>,
}

impl<'a> Context2<'a> {
    fn enter_path(&mut self, name: &'a Name, f: impl FnOnce(&mut Self)) {
        let length = name.iter().position(|&c| c == 0).unwrap_or(name.len());
        let name = str::from_utf8(&name[..length]).unwrap();

        self.path.push(name);

        f(self);

        self.path.pop();
    }

    fn add_path_to_result(&mut self) {
        self.result
            .push(self.path.iter().copied().map(str::to_string).collect());
    }
}

impl Solution {
    fn find_duplicates(context: &mut Context1, node: &mut Node) -> u32 {
        let signature_id = if node.children.is_empty() {
            u32::MAX
        } else {
            let mut signature = node
                .children
                .iter_mut()
                .map(|(&name, node)| (name, Self::find_duplicates(context, node)))
                .collect::<Box<_>>();

            signature.sort_unstable_by_key(|&(name, _)| name);

            match context.signature_to_id.entry(signature) {
                Entry::Occupied(entry) => {
                    let id = *entry.get();

                    context.id_to_duplication[id as usize] = true;

                    id
                }
                Entry::Vacant(entry) => {
                    let id = context.id_to_duplication.len() as _;

                    entry.insert(id);
                    context.id_to_duplication.push(false);

                    id
                }
            }
        };

        node.signature_id = signature_id;

        signature_id
    }

    fn collect_results<'a>(context: &mut Context2<'a>, nodes: &'a HashMap<Name, Node>) {
        for (name, child) in nodes {
            if let Some(&is_duplicated) = context.is_duplicated.get(child.signature_id as usize) {
                if !is_duplicated {
                    context.enter_path(name, |context| {
                        context.add_path_to_result();

                        Self::collect_results(context, &child.children);
                    });
                }
            } else {
                context.enter_path(name, Context2::add_path_to_result);
            }
        }
    }

    pub fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut root = Node::default();

        for path in &paths {
            let mut node = &mut root;

            for component in path {
                let mut new_component = [0_u8; 10];

                new_component[..component.len()].copy_from_slice(component.as_bytes());

                node = node.children.entry(new_component).or_insert_with(Node::default);
            }
        }

        let mut cx1 = Context1 {
            signature_to_id: HashMap::new(),
            id_to_duplication: Vec::new(),
        };

        Self::find_duplicates(&mut cx1, &mut root);

        let mut cx2 = Context2 {
            is_duplicated: convert::identity(cx1).id_to_duplication,
            path: Vec::new(),
            result: Vec::new(),
        };

        Self::collect_results(&mut cx2, &root.children);

        cx2.result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
        Self::delete_duplicate_folder(paths)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
