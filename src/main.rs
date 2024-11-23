

// command strtucture
mod cli;
// static regexs and one replace function
mod re;
// vtable logic and memory related logic
mod logic;
// parsing logic / file-reading / some signature logic  
mod parsing_logic;

mod old_versions;

// broma write and file saver...
mod writer;


use cli::{Cli, BindingCommand};







fn main() {
    let cli = <Cli as clap::Parser>::parse();
    
    match cli.command {
        // Newer updates (1.9 and later) (basically everything after the first windows release goes here...)
        BindingCommand::New { andorid_symbols, cocos2d_symbols, vtables } => (),
        BindingCommand::Old { andorid_symbols, vtables } => (),
    }
    // println!("Hello, world!");
}
