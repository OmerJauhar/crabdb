use sqlparser::ast::Statement;
use sqlparser::dialect::AnsiDialect;
// use sqlparser::dialect::GenericDialect;
use syn::Expr;
use syn::parse_str;
use sqlparser::parser::Parser;
pub fn parserftn() -> ()
{
    // let a = 32 ; 
    let sql_string = "SELECT a, b, c
       FROM table_1 ";

    let sql_dialect = AnsiDialect{} ; // using the ansi dialect 

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

                }
                Statement::Insert { or, into, table_name, columns, overwrite, source, partitioned, after_columns, table, on, returning }  =>
                {
                    println!("Inside insert table") ; 
                    
                }
                _ =>
                {
                    println!("this is the default arm");
                    // println!("{:?}",start_index);
                }
            }


        }
        Err(errormatch) => 
        {
            println!("The Error is {:?}",errormatch) ; 
        }
        
    }

}



fn main()
{
    println!("This is main function ") ; 
    parserftn() ; 
}