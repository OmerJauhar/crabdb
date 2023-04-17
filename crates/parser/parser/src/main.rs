use sqlparser::dialect::AnsiDialect;
// use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
fn parserftn() -> ()
{
    // let a = 32 ; 
    let sql_string = "SELECT a, b
       FROM table_1 ";

    let sql_dialect = AnsiDialect{} ; // using the ansi dialect 

    let ast = Parser::parse_sql(&sql_dialect,sql_string); //ast : abstract syntax tree 
    match ast {
        Ok(vecmatch) =>
        {
            // let a = 0 ; 
            println!("The values of the vector is {:?}",vecmatch) ; 
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



fn main()
{
    println!("This is main function ") ; 
    parserftn() ; 
}