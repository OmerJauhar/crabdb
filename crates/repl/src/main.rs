mod repl ;
// use parser::parser::parserftn ; 
// use parser ;  
//main function will return a Result Enum 
fn main()
{
    match repl::replfunction()
    {
        Ok(_okstatement) =>{
            
        }
        Err(errorstatement) =>{
            println!("{}",errorstatement);
        }
    }
    // let meow = "SELECT a,b FROM table_1;";
    // parserftn(meow);

}