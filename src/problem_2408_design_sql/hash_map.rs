// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

struct Table {
    columns: u32,
    prev_row_id: u32,
    rows: HashMap<u32, Vec<String>>,
}

#[expect(clippy::upper_case_acronyms, reason = "required")]
pub struct SQL {
    tables: HashMap<String, Table>,
}

impl SQL {
    fn new(names: Vec<String>, columns: Vec<i32>) -> Self {
        let mut tables = HashMap::with_capacity(names.len());

        tables.extend(names.into_iter().zip(columns.into_iter().map(|columns| Table {
            columns: columns.cast_unsigned(),
            prev_row_id: 0,
            rows: HashMap::new(),
        })));

        Self { tables }
    }

    fn ins(&mut self, name: String, row: Vec<String>) -> bool {
        if let Some(table) = self
            .tables
            .get_mut(name.as_str())
            .filter(|table| row.len() == table.columns as usize)
        {
            table.prev_row_id += 1;
            table.rows.insert(table.prev_row_id, row);

            true
        } else {
            false
        }
    }

    fn rmv(&mut self, name: String, row_id: i32) {
        if let Some(table) = self.tables.get_mut(name.as_str()) {
            table.rows.remove(&row_id.cast_unsigned());
        }
    }

    fn sel(&self, name: String, row_id: i32, column_id: i32) -> String {
        self.tables
            .get(name.as_str())
            .and_then(|table| table.rows.get(&row_id.cast_unsigned())?.get((column_id - 1) as usize))
            .map_or("<null>", String::as_str)
            .to_string()
    }

    fn exp(&self, name: String) -> Vec<String> {
        let mut result = Vec::new();

        if let Some(table) = self.tables.get(name.as_str()) {
            let mut rows = table.rows.iter().map(|(&id, row)| (id, row)).collect::<Vec<_>>();

            rows.sort_unstable_by_key(|&(id, _)| id);

            result.extend(rows.into_iter().map(|(id, row)| {
                let mut result = id.to_string();

                result.reserve_exact(row.len() + row.iter().map(String::len).sum::<usize>());

                for value in row {
                    result.push(',');
                    result.push_str(value);
                }

                result
            }));
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::SQL for SQL {
    fn new(names: Vec<String>, columns: Vec<i32>) -> Self {
        Self::new(names, columns)
    }

    fn ins(&mut self, name: String, row: Vec<String>) -> bool {
        self.ins(name, row)
    }

    fn rmv(&mut self, name: String, row_id: i32) {
        self.rmv(name, row_id);
    }

    fn sel(&self, name: String, row_id: i32, column_id: i32) -> String {
        self.sel(name, row_id, column_id)
    }

    fn exp(&self, name: String) -> Vec<String> {
        self.exp(name)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::SQL>();
    }
}
