// use std::{collections::{BTreeMap, HashMap}, vec};
// use serde::{Deserialize, Serialize};
// use sqlparser::ast::{Statement::CreateTable, ColumnDef};
// use sql_jr_parser::Column; // See part 1 for this type def. Just column name and sql data type (string or int)
// NOTE: For now just a mapping of col name => data as a str. Will change later
/// A row stored in a table
/// \
// pub struct ident 
// {
    // pub value : String, 
    // pub quote_style:Option<char>
// } 

// #[derive(Debug,Clone , Serialize, Deserialize)]
// pub struct column 
// {
//    pub name :String
// }
// type StoredRow = HashMap<String, String>;
// /// List of column info
// type ColumnInfo = Vec<column>;
// #[derive(Debug, Clone, Default, Serialize, Deserialize)]
// pub(crate) struct Table {
//     /// row id to row
//     rows: BTreeMap<usize, StoredRow>,
//     /// Column info for all columns in the table
//     columns: ColumnInfo,
// }

// impl Table {
//     // Create a table with the given column definitions
//     pub fn new( columns:Vec<column> ) -> Self {
        
//         Self {
//             rows: BTreeMap::new(),
//             columns,
//         }
//     }
//     /// Insert values (a row) into the table
//     ///
//     /// assumes the values are in the same order of the columns passed to create
//     pub fn insert(&mut self, values: Vec<String>) {
        
//         let id = self
//         .rows
//         .last_key_value()
//         .map_or(0, |(max_id, _)| max_id + 1);   
//         let row: StoredRow = values
//             .into_iter()
//             .zip(self.columns.iter())
//             // .map(|(v, col)| (col.name.to_owned(), v))
//             .map(|(v, col)| (col.name.to_owned(), v))
//             .collect();
//         self.rows.insert(id, row);
//     }
// }


fn main()
{
}