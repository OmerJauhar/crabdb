use repl::replmodule::replfunction;
fn main()
{
    let a = 32 ; 
    match replfunction() 
    {
        Ok(meow) =>
        {
            println!("The code is running fine {:?}",meow) ; 
        }
        Err(meowmeow) => 
        {
            println!("The code is not running fine {:?}",meowmeow) ; 
        }
    }

}