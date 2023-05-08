use std::env; 
use std::fs::File;
use std::io::prelude::*;

use repl;
fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() 
    {
        2 =>
        {
            match args[1].as_str() 
            {
                "help" =>
                {
                    println!{"\n\n************ CRABDB ***************"};
                    println!{"Use the following format "};
                    println!{"crabdb [USERNAME] mode [MODE]"};
                    println!{"USERNAME :"};
                    println!{"    -root (default)"};
                    println!{"MODE "};
                    println!{"    -sql"};
                    println!{"    -nosql"};
                    println!{"************ CRABDB ***************\n\n"};

                }
                _ =>
                {
                    let helpstring = "cargo run help";
                    println!("Error! use \"{}\".",helpstring);
                }
            }
        }
        4 => 
        {
            match args[1].as_str()
            {
                "root" =>
                {
                    match args[2].as_str()
                    {
                        "mode" =>
                        {
                            match args[3].as_str()
                            {
                                "sql" => 
                                {
                                    let mut file = File::create("current.txt");
                                    match &mut file 
                                    {
                                        Ok(file_unwraped) =>
                                        {
                                            match file_unwraped.write_all(b"DEFAULT") 
                                            {
                                                Ok(_) =>{}
                                                Err(errorstatement) =>
                                                {
                                                    println!("{}",errorstatement);
                                                }
                                            }
                                        }
                                        Err(errorstatement) =>
                                        {
                                            println!("{}",errorstatement);
                                        }
                                    }
                                    // println!("sql mode of execution");
                                    match repl::repl::replfunction() 
                                    {
                                        Ok(_okstatement) =>{}
                                        Err(errorstatement) =>
                                        {
                                            println!("{}",errorstatement);
                                        }
                                    }
                                }
                                "nosql" =>
                                {
                                    println!("Nosql mode of execution");
                                }
                                _ =>
                                {
                                    let helpstring = "carbdb help";
                                    println!("Error! use \"{}\".",helpstring)
                                }
                            }
                            
                        }
                        _ =>
                        {
                            let helpstring = "carbdb help";
                            println!("Error! use \"{}\".",helpstring)
                        }
                    }
                }
                _ =>
                {
                    let helpstring = "carbdb help";
                            println!("Error! use \"{}\".",helpstring)
                }
            }
        }
        _ =>
        {
            let helpstring = "crabdb help";
            println!("Error! use \"{}\".",helpstring);
        }
    } 
}
