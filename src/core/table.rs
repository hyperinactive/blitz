use super::column::Column;

// ok, so the table needs to have:
// name - string
// columns - new struct
// info about columns
// primary key
// indexes on this table
// the page number for the root BTree page of the table in the db file
// list of triggers for the table
#[derive(Debug)]
pub struct Table {
    name: String,
    columns: Vec<Column>, // TODO(ap): actual column struct pls
    indexes: Vec<String>, // TODO(ap): actual index struct pls
    triggers: Vec<String>,
}

impl Table {
    pub fn new(
        name: &str,
        columns: Option<Vec<Column>>,
        index: Option<Vec<String>>,
        triggers: Option<Vec<String>>,
    ) -> Table {
        Table {
            name: name.to_string(),
            columns: match columns {
                Some(cols) => cols,
                None => Vec::new() as Vec<Column>,
            },
            indexes: match index {
                Some(ind) => ind,
                None => Vec::new() as Vec<String>,
            },
            triggers: match triggers {
                Some(trig) => trig,
                None => Vec::new() as Vec<String>,
            },
        }
    }
}
