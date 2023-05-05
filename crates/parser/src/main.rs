use serde::{Deserialize, Serialize}; 
use std::fs::File;
use std::io::prelude::*;

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
fn main ()
{
    let mut databasearrayobj = databases_array::new();
    let inputstring = String::from("Users ");
    let  mut  databaseobj = databases::new(inputstring);
    databasearrayobj.adddatabase( &databaseobj);
    databaseobj.addtable(String::from("omer"));
    databaseobj.addtable(String::from("jauhar"));
    databaseobj.addtable(String::from("khan"));
    databaseobj.addtable(String::from("meow"));
    databaseobj.describetablesftn();
    databasearrayobj.printdatabase();
}