use sqlparser::dialect::AnsiDialect;
use sqlparser::parser::Parser;
pub fn parserftn(sql_string:&str) -> ()
{
    // let a = 32 ; 
    // let sql_string = "SELECT a, b
    //    FROM table_1 ";

    let sql_dialect = AnsiDialect{} ; // using the ansi dialect 

    let ast = Parser::parse_sql(&sql_dialect,sql_string); //ast : abstract syntax tree 
    match ast {
        Ok(vecmatch) =>
        {
            // let a = 0 ; 
            println!("{:?}",vecmatch) ; 
            // for statements in vecmatch.iter()
            // {
                // 
                // println!(" {:?} \n",statements) ;
                // match statements.parse_statement() 
                // {
                    // Ok(statementmatch) => 
                    // {
                        // println!("The value of the statement is {:?}",statementmatch) ; 
                    // }
                    // Err(statementerror) =>
                    // {
                        // println!("The following error was generated {:?}",statementerror) ; 
                    // }
                // }  
            // }
        }
        Err(errormatch) => 
        {
            println!("The Error is {:?}",errormatch) ; 
        }
        
    }

}

