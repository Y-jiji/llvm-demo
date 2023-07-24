use clap::*;
use clap::ArgAction::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct CliArg {
    /// Display generated llvm ir
    #[arg(long = "display-llvmir", action=SetTrue, default_value_t=false)]
    pub display_llvmir: bool,
    /// Display parsed abstract syntax tree. 
    #[arg(long = "display-parsed", action=SetTrue, default_value_t=false)]
    pub display_parsed: bool,
    /// Display input string. 
    #[arg(long = "display-inputs", action=SetTrue, default_value_t=false)]
    pub display_inputs: bool,
}
