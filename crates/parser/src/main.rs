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
        Ok(vecmatch) =>
        {
            // let a = 0 ; 
            println!("The values of the vector is {:?}",vecmatch) ; 

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