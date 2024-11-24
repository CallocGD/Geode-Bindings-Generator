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

use cli::{BindingCommand, Cli};

use old_versions::do_old_version;

#[allow(dead_code, unused_variables)]
fn main() {
    let cli = <Cli as clap::Parser>::parse();

    match cli.command {
        // Newer updates (1.9 and later) (basically everything after the first windows release goes here...)
        BindingCommand::New {
            andorid_symbols,
            cocos2d_symbols,
            vtables,
        } => {
            println!("Not Implemented Yet but will be in the future...")
        }
        BindingCommand::Old {
            andorid_symbols,
            vtables,
        } => do_old_version(andorid_symbols, vtables),
    }
    // println!("Hello, world!");
}
