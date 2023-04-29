use std::collections::{BTreeMap, HashMap};
use serde::{Deserialize, Serialize};
// use sql_jr_parser::Column; // See part 1 for this type def. Just column name and sql data type (string or int)
// NOTE: For now just a mapping of col name => data as a str. Will change later
/// A row stored in a table
type StoredRow = HashMap<String, String>;
/// List of column info
type ColumnInfo = Vec<Column>;
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct Table {
    /// row id to row
    rows: BTreeMap<usize, StoredRow>,
    /// Column info for all columns in the table
    columns: ColumnInfo,
}