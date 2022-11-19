// TODO: i128 and be done with it?
// create variables at runtime based on max number provided?
pub enum ColumnType {
    STRING(i32),
    BOOL(bool),
    NUMBER(i32), // TODO: distinguish between floats and ints as well as negative numbers
    BLOB(i32),
    DATE(String), // NOTE: have this be a string of a valid date
}

pub struct Column {
    name: String,
    column_type: ColumnType,
    default_value: String,
    not_null: bool,
    is_primary: bool,
    constraints: Vec<String>, // TODO: think about this one, maybe have a vec of constraints
}

impl Column {
    pub fn new(
        name: &str,
        column_tye: ColumnType,
        default_value: &str,
        not_null: bool,
        is_primary: bool,
        constraints: Option<Vec<String>>,
    ) -> Column {
        Column {
            name: name.to_string(),
            column_type: ColumnType::STRING(16),
            default_value: default_value.to_string(),
            not_null,
            is_primary,
            constraints: match constraints {
                Some(vec) => vec,
                None => Vec::new(),
            },
        }
    }
}
