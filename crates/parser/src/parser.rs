use sqlparser::ast::SetExpr;
use sqlparser::dialect::AnsiDialect;
use sqlparser::parser::Parser;
use sqlparser::ast::Statement;
use sqlparser::ast::Expr;
use sqlparser::ast::Value;
use execution ; 
// extern  crate execution; 
use execution::execution::executionmodule::Table ; 
use execution::execution::executionmodule::column;

// #[derive(Debug)]
// struct column 
// {
//     name : String
// }
pub fn parserftn(sql_string:&str) -> ()
{
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
                Statement::Query(..) =>
                {
                    println!("inside query");
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
                _ =>
                {
                    println!("this is the default arm");
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

