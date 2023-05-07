use sqlparser::ast::SetExpr;
use sqlparser::dialect::AnsiDialect;
use sqlparser::parser::Parser;
use sqlparser::ast::Statement;
use sqlparser::ast::Expr;
use sqlparser::ast::Value;
use std::fmt::format;
use std::fs::File;
use std::fs ; 
use std::ops::Index;
use std::slice::SliceIndex;
use std::io::prelude::*;
use execution ; 
// extern  crate execution; 
use execution::execution::executionmodule::Table ; 
use execution::execution::executionmodule::Column;
use serde::{Deserialize, Serialize}; 

#[derive(Debug,Serialize, Deserialize,Clone)]
struct DataBases  
{
    name : String,
    tables : Vec<String>
}

impl DataBases {
    fn new(database_name :String ) -> Self{
        Self { name: (database_name), tables: (Vec::new()) }
    }
    fn addtable(&mut self,table_name : String) -> () {
        self.tables.push(table_name)
    }
    fn rmtable(&mut self , table_name: String) ->()
    {
        let index = self.tables.iter().position(|n| n == &table_name);
        match index
        {
            Some(i) => 
            {
                self.tables.remove(i);
                println!("Table dropped successfully");
            }
            None => println!("Table does not exist")
        }
    } 
    fn describetablesftn (&self) -> ()
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
struct DatabasesArray
{
    array : Vec<DataBases> 
}
impl DatabasesArray {
    fn new() -> Self{
        Self{array : Vec::new()}
    }
    fn adddatabase(&mut self , newdatabse: &DataBases ) -> ()
    {
        self.array.push(newdatabse.clone())
    }
    fn printdatabase(&self) -> ()
    {
        println!("+---------------------------+");
        println!("| DataBases                 |");
        println!("+---------------------------+");

        for i in self.array.iter()
        {
            let printpadded = format!("|{: <27}|",i.name);
            // println!("{}",i.name);
            println!("{:}", printpadded);

        }
        println!("+---------------------------+");
    }
    fn exists(&self, checkstring :String) -> bool
    {
        let mut checkvar : bool = false ; 
        for i in &self.array
        {
            if i.name == checkstring {
                 checkvar = true;
            }
        };
        checkvar
    }
}

pub fn parserftn(sql_string:&str) -> ()
{
    // let mut current_database = String::from("Default"); 
    let mut parser_databases_array = DatabasesArray::new();
    let  parser_database = DataBases::new(String::from("Default"));
    parser_databases_array.adddatabase(&parser_database);
    let filemetadata = fs::metadata("person.json");
    match filemetadata {
        Ok(_) =>
        {
            // println!("File do exists ");
        }
        Err(_) =>
        {
            println!("File is not created");
            let _file = File::create("person.json").unwrap();
        }
    }

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
                Statement::Analyze { table_name: _, partitions:_, for_columns:_, columns:_, cache_metadata:_, noscan:_, compute_statistics:_ } =>
                {
                    println!("inside analyze)");
                    // println!("{:?}",table_name);
                    // println!("{:?}",);
                }
                Statement::CreateTable { or_replace:_, temporary:_, external:_, global:_, if_not_exists:_, transient:_, name, columns, constraints:_, hive_distribution:_, hive_formats:_, table_properties:_, with_options:_, file_format:_, location:_, query:_, without_rowid:_, like:_, clone:_, engine:_, default_charset:_, collation:_, on_commit:_, on_cluster:_, order_by:_ }   =>
                {
                    let mut contents1 = String::new();
                    let mut file = File::open("current.txt");
                    let mut boolvar = false ;  
                    match &mut file 
                    {
                        Ok(file_unwrapped) =>
                        {
                            match file_unwrapped.read_to_string(&mut contents1)
                            {
                                Ok(_) => 
                                {
                                    if contents1 == String::from("DEFAULT"){
                                        println!("No Database Selected ");
                                    }
                                    else  {
                                        let mut fileread = File::open("person.json");
                                        match &mut fileread {
                                            Ok(file) =>
                                            {
                                                let mut contents = String::new();
                                                // file.read_to_string(&mut contents).unwrap();
                                                file.read_to_string(&mut contents).unwrap();

                                                let mut read_database_array :DatabasesArray = serde_json::from_str(&contents).unwrap();
                                                for  i in read_database_array.array.iter_mut()
                                                {
                                                   if (i.name == contents1)
                                                   {
                                                    //    println!("matched");
                                                       i.addtable(name.0[0].value.clone());
                                                       i.describetablesftn(); 
                                                       let mut  i = 0 ; 
                                                       let mut finalvector = vec![Column
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
                                                               finalvector.push(Column{
                                                                   name : iter.name.value
                                                               });
                                                           }
                                                       }
                                                    //    println!("{:?}",finalvector);
                                                       let currentable =Table::new(finalvector);
                                                       let table_file = name.0[0].value.clone().to_string() + ".json";
                                                    //    println!("{}",table_file);
                                                       let mut filewrite = File::create(table_file);
                                                       match &mut filewrite
                                                       {
                                                        Ok(file) =>
                                                        {
                                                            let serialized_current_table = serde_json::to_string((&currentable));
                                                            match &serialized_current_table
                                                            {
                                                             Ok(sct) =>
                                                             {
                                                                 match file.write_all(sct.as_bytes())
                                                                 {
                                                                     Ok(_) =>
                                                                     {
                                                                        //  println!("Successfull");
                                                                     }
                                                                     Err(errorstatement) => {println!("{}",errorstatement)}
     
                                                                 }
                                                             }
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
                                                       boolvar = true ;
                                                       break;
                                                   }
                                                }   
                                                if boolvar
                                                {
                                                    let mut filewrite = File::create("person.json");
                                                    file.set_len(0);
                                                    match &mut filewrite 
                                                    {
                                                        Ok(file) => 
                                                        {
                                                            let serialized_parser_database_array  = serde_json::to_string(&read_database_array);
                                                            match &serialized_parser_database_array
                                                            {
                                                                Ok(spda_string) =>
                                                                {
                                                                    match file.write_all(spda_string.as_bytes()) 
                                                                    {
                                                                        Ok(_) =>
                                                                        {
                                                                            println!("Successfull");
                                                                        }
                                                                        Err(errormsg) =>
                                                                        {   
                                                                            println!("error : {}",errormsg);
                                                                        }
                                                                    }
                                                                }
                                                                Err(_) =>
                                                                {
                                                                    println!("Error at serde_json");
                                                                }
                                                            }
                                                        }
                                                        Err(errorstatement) => { println!("{}",errorstatement)}
                                                    } 
                                                }
                                                       
                                                    
                                                

                                            }
                                            Err(errormsg ) =>
                                            {
                                                println!("Inside error");
                                                println!("{}",errormsg);
                                            }
                                        }
                                        if boolvar
                                        {
                                            
                                            
                                        }
                                    }
                                }
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
                    // println!("Inside create table ") ; 

                    // let mut  i = 0 ; 
                    // let mut finalvector = vec![Column
                    // {
                    //     name: String::from("dummy")
                    // }];
                    
                    // for iter in columns{
                    //     if i == 0 
                    //     {
                    //         finalvector[0].name = iter.name.value ;
                    //         i+=1 ;
                    //     }
                    //     else   {
                    //         finalvector.push(Column{
                    //             name : iter.name.value
                    //         });
                    //     }
                    // }
                    // println!("{:?}",finalvector);
                    // finalcurrentable =Table::new(finalvector);
                    
                    

                }
                Statement::Query(..) =>
                {
                    println!("inside select query");
                    
                    // finalcurrentable.printtable();
                    
                }
                Statement::Insert { or:_, into:_, table_name:_, columns, overwrite:_, source, partitioned:_, after_columns:_, table:_, on:_, returning:_ }  =>
                {
                    println!("Inside insert table") ;
                    let mut  i = 0 ; 
                    let mut finalvector = vec![Column
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
                            finalvector.push(Column{
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
                                            Value::Number(numbervar,_boolval) =>
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
                Statement::ShowTables{extended:_,full:_,db_name:_,filter:_}=>
                {
                    let mut contents1 = String::new();
                    let mut file = File::open("current.txt");
                    match &mut file 
                    {
                        Ok(file_unwrapped) =>
                        {
                            match file_unwrapped.read_to_string(&mut contents1)
                            {
                                Ok(_) => 
                                {
                                    if contents1 == String::from("DEFAULT"){
                                        println!("No Database Selected ");
                                    }
                                    else  {
                                        let mut fileread = File::open("person.json");
                                        match &mut fileread {
                                            Ok(file) =>
                                            {
                                                let mut contents = String::new();
                                                file.read_to_string(&mut contents).unwrap();
                                                let read_database_array :DatabasesArray = serde_json::from_str(&contents).unwrap();
                                                 for i in read_database_array.array.iter()
                                                 { 
                                                    if i.name == contents1 
                                                    {
                                                        i.describetablesftn();
                                                    }
                                                 }
                                                
                                            }
                                            Err(errormsg ) =>
                                            {
                                                println!("Inside error");
                                                println!("{}",errormsg);
                                            }
                                        }
                                    }
                                }
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
                    
                }
                Statement::Drop { object_type, if_exists:_, names, cascade:_, restrict:_, purge:_ } =>
                {
                    //drop table implementation
                    let mut  boolvar = false ; 
                    match object_type
                    {
                        Table =>
                        {
                            let  table_name = names[0].0[0].value.clone();
                            let mut contents1 = String::new();
                            let mut file = File::open("current.txt");
                            match &mut file 
                            {
                                Ok(file_unwrapped) =>
                                {
                                    match file_unwrapped.read_to_string(&mut contents1)
                                    {
                                        Ok(_) => 
                                        {
                                            if contents1 == String::from("DEFAULT"){
                                                println!("No Database Selected ");
                                            }
                                            else  {
                                                let mut fileread = File::open("person.json");
                                                match &mut fileread {
                                                    Ok(file) =>
                                                    {
                                                        let mut contents = String::new();
                                                        file.read_to_string(&mut contents).unwrap();
                                                        let mut  read_database_array :DatabasesArray = serde_json::from_str(&contents).unwrap();
                                                         for i in read_database_array.array.iter_mut()
                                                         { 
                                                            for j in i.tables.clone()
                                                            {
                                                                if j == table_name 
                                                                {
                                                                    i.rmtable(table_name.clone());
                                                                    boolvar = true ; 
                                                                    break ; 
                                                                }
                                                            }
                                                         }
                                                         if boolvar == false 
                                                         {
                                                            println!("No table {} in database {}",table_name,contents1);
                                                         }
                                                         if boolvar
                                                         {
                                                             let mut filewrite = File::create("person.json");
                                                             file.set_len(0);
                                                             match &mut filewrite 
                                                             {
                                                                 Ok(file) => 
                                                                 {
                                                                     let serialized_parser_database_array  = serde_json::to_string(&read_database_array);
                                                                     match &serialized_parser_database_array
                                                                     {
                                                                         Ok(spda_string) =>
                                                                         {
                                                                             match file.write_all(spda_string.as_bytes()) 
                                                                             {
                                                                                 Ok(_) =>
                                                                                 {
                                                                                     let file_path = table_name + ".json";
                                                                                     match fs::remove_file(file_path)
                                                                                     {
                                                                                      Ok(_) => {println!("Successfull");}
                                                                                      Err(errorstatement) => {println!("{}",errorstatement);}
                                                                                     }
                                                                                 }
                                                                                 Err(errormsg) =>
                                                                                 {   
                                                                                     println!("error : {}",errormsg);
                                                                                 }
                                                                             }
                                                                         }
                                                                         Err(_) =>
                                                                         {
                                                                             println!("Error at serde_json");
                                                                         }
                                                                     }
                                                                 }
                                                                 Err(errorstatement) => { println!("{}",errorstatement)}
                                                             } 
                                                         }
                                                     
                                                    }
                                                    Err(errormsg ) =>
                                                    {
                                                        println!("Inside error");
                                                        println!("{}",errormsg);
                                                    }
                                                }
                                            }
                                        }
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
                            
                    

                        }
                    }
                }
                Statement::CreateDatabase { db_name, if_not_exists:_, location:_, managed_location:_ } =>
                {
                    
                    let mut fileread = File::open("person.json");
                    match &mut fileread {
                        Ok(file) =>
                        {
                            let mut contents = String::new();
                            file.read_to_string(&mut contents).unwrap();
                            let mut read_database_array :DatabasesArray = serde_json::from_str(&contents).unwrap();
                            read_database_array.printdatabase();
                            if!read_database_array.exists(db_name.0[0].value.clone())
                            {
                                match file.set_len(0)
                                {
                                    Ok(file1) =>
                                    {
                                        drop(file1);
                                        let mut filewrite = File::create("person.json");
                                        match &mut filewrite 
                                        {
                                            Ok(file) =>
                                            {
                                                let meow = db_name.0[0].value.clone();
                                                let meowdatabase = DataBases::new(meow);
                                                read_database_array.adddatabase(&meowdatabase);
                                                read_database_array.printdatabase();
            
                                                let serialized_parser_database_array  = serde_json::to_string(&read_database_array);
                                                match &serialized_parser_database_array
                                                {
                                                    Ok(spda_string) =>
                                                    {
                                                        match file.write_all(spda_string.as_bytes()) 
                                                        {
                                                            Ok(_) =>
                                                            {
                                                                println!("Successfull");
                                                            }
                                                            Err(errormsg) =>
                                                            {   
                                                                println!("error : {}",errormsg);
                                                            }
                                                        }
                                                    }
                                                    Err(_) =>
                                                    {
                                                        println!("Error at serde_json");
                                                    }
                                                }
            
                                            }
                                            Err(_) =>
                                            {
                                                println!("File not opened in createdb");
                                            }
                                        }
                                    }
                                    Err(errorstatement)=>
                                    {
                                        println!("{}",errorstatement);
                                    }
                                }
                                

                            }
                            else  {
                                println!("Database Already Exists!");
                            }

                            
                        }
                        Err(errormsg ) =>
                        {
                            println!("Inside error");
                            println!("{}",errormsg);
                        }
                    }
                    
                    // // let mut fileread = OpenOptions::new()
                    // // .write(true)
                    // // .truncate(true)
                    // // .open("person.json");
                    // let mut fileread = File::open("person.json");
                    // match &mut fileread
                    // {
                    //     Ok(filew) =>
                    //     {
                    //         let mut contents = String::new(); 
                    //         filew.read_to_string(&mut contents).unwrap();
                    //         let mut read_database_array :DatabasesArray = serde_json::from_str(&contents).unwrap();
                    //         let meowdatabase = DataBases::new(db_name.0[0].value.clone());
                    //         read_database_array.adddatabase(&meowdatabase);
                    //         match filew.set_len(0)
                    //         {
                    //             Ok(_) =>
                    //             {
                    //                 let serialized_parser_database_array  = serde_json::to_string(&read_database_array);
                    //                 match &serialized_parser_database_array
                    //                 {
                    //                     Ok(spda_string) =>
                    //                     {
                    //                         match filew.write_all(spda_string.as_bytes()) 
                    //                         {
                    //                             Ok(_) =>
                    //                             {
                    //                                 println!("Successfull");
                    //                             }
                    //                             Err(errormsg) =>
                    //                             {   
                    //                                 println!("error : {}",errormsg);
                    //                             }
                    //                         }
                    //                     }
                    //                     Err(_) =>
                    //                     {
                    //                         println!("Error at serde_json");
                    //                     }
                    //                 }    
                    //             }
                    //             Err(_) =>
                    //             {
                                
                    //             }
                    //         } 

                    //     }
                    //     Err(error) =>
                    //     {
                    //         println!("Error ");
                    //     }
                    // }
                    // let mut fileread = File::open("person.json");
                    // match &mut fileread {
                    //     Ok(file) =>
                    //     {
                    //         let mut contents = String::new();
                    //         file.read_to_string(&mut contents).unwrap();
                    //         let mut read_database_array :DatabasesArray = serde_json::from_str((&contents)).unwrap();
                    //         let  meow: String = db_name.0[0].value.clone();
                    //         let  meowdatabase = DataBases::new(meow);
                    //         read_database_array.adddatabase(&meowdatabase);
                    //         file.set_len(0);
                    //         let serialized_parser_database_array  = serde_json::to_string(&read_database_array);
                    //         match &serialized_parser_database_array
                    //         {
                    //             Ok(spda_string) =>
                    //             {
                    //                 match file.write_all(spda_string.as_bytes()) 
                    //                 {
                    //                     Ok(_) =>
                    //                     {
                    //                         println!("Successfull");
                    //                     }
                    //                     Err(errormsg) =>
                    //                     {   
                    //                         println!("error : {}",errormsg);
                    //                     }
                    //                 }
                    //             }
                    //             Err(_) =>
                    //             {
                    //                 println!("Error at serde_json");
                    //             }
                    //         }    
                    //     }
                    //     Err(errormsg ) =>
                    //     {
                    //         println!("Inside error");
                    //         println!("{}",errormsg);
                    //     }
                    // }
                    
                    // let mut filewrite = File::open("person.json");
                    // let mut filewrite = File::create("person.json");
                    // match &mut filewrite 
                    // {
                    //     Ok(file) =>
                    //     {
                    //         let meow = db_name.0[0].value.clone();
                    //         let meowdatabase = DataBases::new(meow);
                    //         parser_databases_array.adddatabase(&meowdatabase);
                    //         parser_databases_array.printdatabase();
                            
                    //         let serialized_parser_database_array  = serde_json::to_string(&parser_databases_array);
                    //         match &serialized_parser_database_array
                    //         {
                    //             Ok(spda_string) =>
                    //             {
                    //                 match file.write_all(spda_string.as_bytes()) 
                    //                 {
                    //                     Ok(_) =>
                    //                     {
                    //                         println!("Successfull");
                    //                     }
                    //                     Err(errormsg) =>
                    //                     {   
                    //                         println!("error : {}",errormsg);
                    //                     }
                    //                 }
                    //             }
                    //             Err(_) =>
                    //             {
                    //                 println!("Error at serde_json");
                    //             }
                    //         }
                            
                    //     }
                    //     Err(_) =>
                    //     {
                    //         println!("File not opened in createdb");
                    //     }
                    // }

                }
                // Statement::ShowCollation { filter } =>
                // {
                //     println!("meow meow")
                // }
                Statement:: Use { db_name } =>
                {
                    let mut fileread = File::open("person.json");
                    match &mut fileread {
                        Ok(file) =>
                        {
                            let mut contents = String::new();
                            file.read_to_string(&mut contents).unwrap();
                            let read_database_array :DatabasesArray = serde_json::from_str(&contents).unwrap();
                            if read_database_array.exists(db_name.value.clone())
                            {
                                let mut file = File::create("current.txt");
                                match &mut file {
                                    Ok(file_unwrapped) =>
                                    {
                                    match file_unwrapped.set_len(0)
                                    {
                                        Ok(_) => {}
                                        Err(errorstatement) => {println!("{}",errorstatement)}
                                    }
                                    match file_unwrapped.write_all(db_name.value.clone().as_bytes())
                                    {
                                        Ok(_) => {
                                            println!("Database switched to {}",db_name.value);
                                        }
                                        Err(errorstatement) => {println!("{}",errorstatement)}
                                    }
                                    }
                                    Err(errorstatement) =>
                                    {
                                        println!("{}",errorstatement);
                                    }
                                }
                                // println!("Database switched to {}",GLOBAL_CURRENT_DB);
                            }
                            else  {
                                println!("Database does not exists.");
                                println!("Current DataBases are :");
                                read_database_array.printdatabase();
                            }
                            
                        }
                        Err(errormsg ) =>
                        {
                            println!("Inside error");
                            println!("{}",errormsg);
                        }
                    }
                }
                _=>
                {
                    // println!("Inside Show DataBases ");
                    // let mut fileread = File::open("person.json").unwrap();
                    // let mut contents = String::new();
                    // fileread.read_to_string(&mut contents).unwrap();
                    // let read_database_array : DatabasesArray= serde_json::from_str((&contents)).unwrap();
                    // read_database_array.printdatabase();
                    let mut fileread = File::open("person.json");
                    match &mut fileread {
                        Ok(file) =>
                        {
                            let mut contents = String::new();
                            file.read_to_string(&mut contents).unwrap();
                            let read_database_array :DatabasesArray = serde_json::from_str(&contents).unwrap();
                            read_database_array.printdatabase();
                            
                        }
                        Err(errormsg ) =>
                        {
                            println!("Inside error");
                            println!("{}",errormsg);
                        }
                    }
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

