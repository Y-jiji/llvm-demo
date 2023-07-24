use std::io::Read;

use rustyline::error::ReadlineError;

#[macro_use]
extern crate lalrpop_util;

pub mod syntax;
pub mod cliarg;
pub mod codgen;

/// build makes more !
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cliarg = <cliarg::CliArg as clap::Parser>::parse();
    
    println!("+--------------------------------+");
    println!("| Tips:                          |");
    println!("| `Ctrl + D` to fire a command.  |");
    println!("| `Ctrl + C` to stop the loop.   |");
    println!("+--------------------------------+");

    let mut line_reader = rustyline::DefaultEditor::new()?;
    loop {
        // --- get buffer until eof ---
        let mut buf = String::new();
        loop { match line_reader.readline("==> ") {
            Ok(line) => { buf.push_str(line.as_str()); buf.push('\n'); }
            Err(ReadlineError::Eof) => { break }
            Err(ReadlineError::Interrupted) => { println!("exit"); return Ok(()) }
            Err(e) => Err(e)?
        } }
        
        // --- print input string ---
        let inputs = buf.trim_end();
        if cliarg.display_inputs {
            println!("--- string ---");
            println!("{inputs}");
        }
        
        // --- print parsed input --- 
        match syntax::ItemsParser::new().parse(inputs) {
            Ok(parsed)  => {if cliarg.display_parsed {
                println!("--- parsed ---");
                for x in parsed { println!("{x:?}"); }
            }},
            Err(parerr) => {
                println!("--- errors ---\n{parerr}");
            }
        }
        
        // --- print generated llvm ir ---
        if cliarg.display_llvmir {
            println!("--- llvmir ---");
            println!();
        }
        
        // --- print evaluated result ---

    }
    Ok(())
}
