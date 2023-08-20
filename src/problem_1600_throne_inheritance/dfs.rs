// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::convert;
use std::rc::Rc;

struct Person {
    name: Rc<str>,
    children: Vec<usize>,
    alive: bool,
}

pub struct ThroneInheritance {
    string_to_id: HashMap<Rc<str>, usize>,
    persons: Vec<Person>,
}

impl ThroneInheritance {
    fn new(king_name: String) -> Self {
        let king_name = Rc::from(king_name);

        Self {
            string_to_id: HashMap::from([(Rc::clone(&king_name), 0)]),
            persons: vec![Person {
                name: king_name,
                children: Vec::new(),
                alive: true,
            }],
        }
    }

    fn birth(&mut self, parent_name: String, child_name: String) {
        let parent = self.string_to_id[convert::identity(parent_name).as_str()];
        let child_name = Rc::from(child_name);
        let child = self.string_to_id.len();

        self.string_to_id.insert(Rc::clone(&child_name), child);

        self.persons.push(Person {
            name: child_name,
            children: Vec::new(),
            alive: true,
        });

        self.persons[parent].children.push(child);
    }

    fn death(&mut self, name: String) {
        let id = self.string_to_id[convert::identity(name).as_str()];
        let person = &mut self.persons[id];

        person.alive = false;
    }

    fn get_inheritance_order(&self) -> Vec<String> {
        fn dfs(tree: &[Person], result: &mut Vec<String>, node: usize) {
            let person = &tree[node];

            if person.alive {
                result.push(person.name.to_string());
            }

            for &child in &person.children {
                dfs(tree, result, child);
            }
        }

        let mut result = Vec::new();

        dfs(&self.persons, &mut result, 0);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::ThroneInheritance for ThroneInheritance {
    fn new(king_name: String) -> Self {
        Self::new(king_name)
    }

    fn birth(&mut self, parent_name: String, child_name: String) {
        self.birth(parent_name, child_name);
    }

    fn death(&mut self, name: String) {
        self.death(name);
    }

    fn get_inheritance_order(&self) -> Vec<String> {
        self.get_inheritance_order()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::ThroneInheritance>();
    }
}
