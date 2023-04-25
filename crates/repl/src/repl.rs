
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use parser::parser::parserftn ;
 
//main function will return a Result Enum 
pub fn replfunction() -> Result<()> {
    // `()` can be used when no completer is required
    let mut rl = DefaultEditor::new()?;
    #[cfg(feature = "with-file-history")]
    //rl.load history return a result enum 
    // err checks if the result is err or ok()
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        // let a = 43 ; 
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                println!("{}",line);
                match rl.add_history_entry(line.as_str())
                {
                    Ok(meow) => 
                    {
                        // println!("{:?}",meow) ; 
                    }
                    Err(error)=>
                    {
                        println!("{:?}",error) ; 
                    }
                }
                println!("Line: {}", line);
                parserftn(&line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    #[cfg(feature = "with-file-history")]
    rl.save_history("history.txt");
    Ok(())
}

