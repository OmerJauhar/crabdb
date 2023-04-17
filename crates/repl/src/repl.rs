pub mod replmodule {

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
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
        let a = 43 ; 
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                match rl.add_history_entry(line.as_str())
                {
                    Ok(meow) => 
                    {
                        println!("The code ran okayish {:?}",meow) ; 
                    }
                    Err(error)=>
                    {
                        println!("The code ran not okaish {:?}",error) ; 
                    }
                }
                println!("Line: {}", line);
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

}