use repl::replmodule::replfunction;
fn main()
{
    match replfunction() 
    {
        Ok(meow) =>
        {
            println!("The code is running fine {:?}",meow) ; 
            let a = 342 ; 
        }
        Err(meowmeow) => 
        {
            let abs = 43 ; 
            println!("The code is not running fine {:?}",meowmeow) ; 
        }
    }

}
