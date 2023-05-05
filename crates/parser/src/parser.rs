use sqlparser::ast::SetExpr;
use sqlparser::dialect::AnsiDialect;
use sqlparser::parser;
use sqlparser::parser::Parser;
use sqlparser::ast::Statement;
use sqlparser::ast::Expr;
use sqlparser::ast::Value;
use std::fs::File;
use std::io::prelude::*;
use execution ; 
// extern  crate execution; 
use execution::execution::executionmodule::Table ; 
use execution::execution::executionmodule::column;
use serde::{Deserialize, Serialize}; 


#[derive(Debug,Serialize, Deserialize,Clone)]
struct databases  
{
    name : String,
    tables : Vec<String>
}

impl databases {
    fn new(database_name :String ) -> Self{
        Self { name: (database_name), tables: (Vec::new()) }
    }
    fn addtable(&mut self,table_name : String) -> () {
        self.tables.push(table_name)
    }
    fn describetablesftn (&mut self) -> ()
    {
        println!("+---------------------------+");
        println!("| Tables                    |");
        println!("+---------------------------+");

        for i in self.tables.iter()
        {
           let printpadded =  format!("|{: <27}|",i);
           println!("{}",printpadded);
        }
        println!("+---------------------------+");

    }

}
#[derive(Debug , Serialize , Deserialize)]
struct databases_array
{
    array : Vec<databases> 
}
impl databases_array {
    fn new() -> Self{
        Self{array : Vec::new()}
    }
    fn adddatabase(&mut self , newdatabse: &databases ) -> ()
    {
        self.array.push(newdatabse.clone())
    }
    fn printdatabase(&self) -> ()
    {
        println!("+---------------------------+");
        println!("| Databases                 |");
        println!("+---------------------------+");

        for i in self.array.iter()
        {
            let printpadded = format!("|{: <27}|",i.name);
            // println!("{}",i.name);
            println!("{:}", printpadded);

        }
        println!("+---------------------------+");
    }
}

pub fn parserftn(sql_string:&str) -> ()
{
    let mut parser_databases_array = databases_array::new();
    let mut parser_database = databases::new(String::from("Default"));
    parser_databases_array.adddatabase(&parser_database);
    let mut file = File::create("person.json").unwrap();
    // let a = 32 ; 
    // let sql_string = "SELECT a, b
    //    FROM table_1 ";

    let sql_dialect = AnsiDialect{} ;
    let mut finalcurrentable : Table ;  // using the ansi dialect 

    let ast = Parser::parse_sql(&sql_dialect,sql_string); //ast : abstract syntax tree 
        match ast {
        Ok(mut vecmatch) =>
        {
            // let a = 0 ; 
            // println!("The values of the vector is {:?}",vecmatch) ; 
            let start_index = vecmatch.remove(0); 
            match start_index {
                Statement::Analyze { table_name, partitions, for_columns, columns, cache_metadata, noscan, compute_statistics } =>
                {
                    println!("inside analyze)");
                    // println!("{:?}",table_name);
                    // println!("{:?}",);
                }
                Statement::CreateTable { or_replace, temporary, external, global, if_not_exists, transient, name, columns, constraints, hive_distribution, hive_formats, table_properties, with_options, file_format, location, query, without_rowid, like, clone, engine, default_charset, collation, on_commit, on_cluster, order_by }   =>
                {
                    println!("Inside create table ") ; 
                    let mut  i = 0 ; 
                    let mut finalvector = vec![column
                    {
                        name: String::from("dummy")
                    }];
                    
                    for iter in columns{
                        if i == 0 
                        {
                            finalvector[0].name = iter.name.value ;
                            i+=1 ;
                        }
                        else   {
                            finalvector.push(column{
                                name : iter.name.value
                            });
                        }
                    }
                    println!("{:?}",finalvector);
                    finalcurrentable =Table::new(finalvector);
                    
                    

                }
                Statement::Query(..) =>
                {
                    println!("inside select query");
                    
                    // finalcurrentable.printtable();
                    
                }
                Statement::Insert { or, into, table_name, columns, overwrite, source, partitioned, after_columns, table, on, returning }  =>
                {
                    println!("Inside insert table") ;
                    let mut  i = 0 ; 
                    let mut finalvector = vec![column
                    {
                        name: String::from("dummy")
                    }];
                    
                    for iter in columns{
                        if i == 0 
                        {
                            finalvector[0].name = iter.value ;
                            i+=1 ;
                        }
                        else   {
                            finalvector.push(column{
                                name : iter.value
                            });
                        }
                    }
                    println!("{:?}",finalvector);
                    finalcurrentable =Table::new(finalvector);
                    let mut insertvector = Vec::new();
                    match *source.body
                    {
                        SetExpr::Values(finalvalues) =>
                        {

                            // println!("{:?}",finalvalues.rows[0][0]);
                            for iter in finalvalues.rows[0].iter()
                            {
                                // println!("{:?}",iter);
                                match iter 
                                {
                                    Expr::Value(valuesfinal) =>
                                    {
                                        match valuesfinal
                                        {
                                            Value::Number(numbervar,boolval) =>
                                            {
                                                insertvector.push(numbervar.to_string());
                                            }
                                            Value::SingleQuotedString(stringvar) =>
                                            {
                                                insertvector.push(stringvar.to_string());
                                            }
                                            _ =>
                                            {
                                            }
                                        }
                                    }
                                _ =>
                                {
                                }
                                }
                            }
                        }
                        _ =>
                        {
                        }
                        
                    }
                    println!("{:?}",insertvector);
                    finalcurrentable.insert(insertvector);
                    finalcurrentable.printtable();
                }
                Statement::ShowTables{extended,full,db_name,filter}=>
                {
                    println!("Inside show table");
                    
                }
                Statement::CreateDatabase { db_name, if_not_exists, location, managed_location } =>
                {
                    
                    let mut filewrite = File::open("person.json").unwrap();
                    let meow = db_name.0[0].value.clone();
                    let meowdatabase = databases::new(meow);
                    parser_databases_array.adddatabase(&meowdatabase);
                    parser_databases_array.printdatabase();
                    let serialized_parser_database_array  = serde_json::to_string(&parser_databases_array).unwrap();
                    filewrite.write_all(serialized_parser_database_array.as_bytes()).unwrap();
                }
                Statement::ShowCollation { filter } =>
                {
                    println!("meow meow")
                }
                Statement:: Use { db_name } =>
                {
                    println!("Inside use database ");
                }
                _=>
                {
                    // println!("Inside Show Databases ");
                    // let mut fileread = File::open("person.json").unwrap();
                    // let mut contents = String::new();
                    // fileread.read_to_string(&mut contents).unwrap();
                    // let read_database_array : databases_array= serde_json::from_str((&contents)).unwrap();
                    // read_database_array.printdatabase();
                    println!("Inside Show Databases ");
                    let mut fileread = File::open("person.json");
                    match &mut fileread {
                        Ok(file) =>
                        {
                            println!("Inside OK");
                            let mut contents = String::new();
                            file.read_to_string(&mut contents).unwrap();
                            println!("{}",contents);
                            println!("After Contents");
                            // let read_database_array  = serde_json::from_str((&contents)).unwrap();
                            // read_database_array.printdatabase();
                            
                        }
                        Err(errormsg ) =>
                        {
                            println!("Inside error");
                            println!("{}",errormsg);
                        }
                    }
                }
            }


        }
        Err(errormatch) => 
        {
            println!("The Error is {:?}",errormatch) ; 
        }
        
    }
    // match ast {
    //     Ok(vecmatch) =>
    //     {
    //         // let a = 0 ; 
    //         println!("{:?}",vecmatch) ; 
    //         // for statements in vecmatch.iter()
    //         // {
    //             // 
    //             // println!(" {:?} \n",statements) ;
    //             // match statements.parse_statement() 
    //             // {
    //                 // Ok(statementmatch) => 
    //                 // {
    //                     // println!("The value of the statement is {:?}",statementmatch) ; 
    //                 // }
    //                 // Err(statementerror) =>
    //                 // {
    //                     // println!("The following error was generated {:?}",statementerror) ; 
    //                 // }
    //             // }  
    //         // }
    //     }
    //     Err(errormatch) => 
    //     {
    //         println!("The Error is {:?}",errormatch) ; 
    //     }
        
    // }

}

