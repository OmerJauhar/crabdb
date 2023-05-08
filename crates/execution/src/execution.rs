pub mod executionmodule 
{
use std::{collections::{BTreeMap, HashMap}};
use serde::{Deserialize, Serialize};
// use sqlparser::ast::{Statement::CreateTable, ColumnDef};
// use sql_jr_parser::Column; // See part 1 for this type def. Just Column name and sql data type (string or int)
// NOTE: For now just a mapping of col name => data as a str. Will change later
/// A row stored in a table
/// \
// pub struct ident 
// {
    // pub value : String, 
    // pub quote_style:Option<char>
// } 

#[derive(Debug,Clone , Serialize, Deserialize)]
pub struct Column 
{
   pub name :String
}
type StoredRow = HashMap<String, String>;
/// List of Column info
type ColumnInfo = Vec<Column>;


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Table {
    /// row id to row
    rows: BTreeMap<usize, StoredRow>,
    /// Column info for all columns in the table
    columns: ColumnInfo,
}

impl Table {
    // Create a table with the given Column definitions
    pub fn new( columns:Vec<Column> ) -> Self {
        
        Self {
            rows: BTreeMap::new(),
            columns,
        }
    }
    /// Insert values (a row) into the table
    ///
    /// assumes the values are in the same order of the columns passed to create
    pub fn insert(&mut self, values: Vec<String>) {
        
        let id = self
        .rows
        .last_key_value()
        .map_or(0, |(max_id, _)| max_id + 1);   
        let row: StoredRow = values
            .into_iter()
            .zip(self.columns.iter())
            // .map(|(v, col)| (col.name.to_owned(), v))
            .map(|(v, col)| (col.name.to_owned(), v))
            .collect();
        self.rows.insert(id, row);
    }
    pub fn selectftn(&mut self , columns: Vec<String> , table_name : String) -> ()
    {
        println!("+----------------------------------++");
        println!("| Table:  {:.15}                ||",table_name.clone());
        let mut i = 0 ; 
        while(i < columns.len())
        {
            print!("+----------------+");
            i +=1 ;
        }
        println!();
        for i in columns.clone()
        {
            if i == "*"
            {
                break ;
            }
            print!("{}|",format!("|{:^15.15} ",i));
        }

        let mut boolvar = false ; 
        let mut meowvar = false;
        let mut printcounter = 1 ;
        // println!("printing the values");
        for (_key , value) in self.rows.clone()
        {
            for i in columns.clone()
            {
                
                for (key1,value1) in value.clone()
                {
                    if i == "*"
                    {
                        self.printtable(table_name.clone());
                        boolvar = true ; 
                        break ; 
                    }
                    else
                    {
                        if i == key1{
                            if meowvar ==false
                            {
                                println!();
                                println!("+----------------+");
                                meowvar = true ; 
                            }
                            print!("{}|",format!("|{:^15.15} ",value1));
                            // print!("+----------------+");
                        }
                    }
                }
                if boolvar
                {
                    break ;
                }
                if printcounter < columns.len()
                {
                    printcounter +=1 ; 
                }
                else  {
                    println!();
                    let mut i = 0 ; 
                    while(i < printcounter)
                    {
                        print!("+----------------+");
                        i +=1 ;
                    }
                    println!();
                    printcounter = 1 ;
                }
                
            }   
            if boolvar
            {
                break;
            }
        }
    }
    pub fn printtable (&mut self ,table_name : String)
    {
        for (_key , value) in self.rows.clone()
        {
            for i  in value.clone()
            {
                    print!("+----------------+");
            }  
            println!();
            for (key1,value1) in value.iter()
            {
                    // print!("+----------+");
                    let printpadded =  format!("|{:^15.15} ",key1);
                    print!("{}|",printpadded);
                    // print!("+----------+");
            }   

            println!();
            for (key1,value1) in value.iter()
            {
                    print!("{}|",format!("|{:^15.15} ",value1));
            }   
            println!();
            for i in value.clone()
            {
                    print!("+----------------+");
            }  
            println!();
        }
        
        // for (_key , value) in self.rows.clone()
        // {
        //     println!();
        //     for (key1,value1) in value.iter()
        //     {
        //             print!("{}|",format!("|{:^15.15} ",value1));
        //     }   
        //     println!();
        // }
        
    }

        
    // pub fn iter(&self) -> std::collections::btree_map::Iter<usize, Row> {
    //     self.rows.iter()
    // }
}



}