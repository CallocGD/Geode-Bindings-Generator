



#[derive(clap::Parser, Debug)]
#[clap(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: BindingCommand
}

// TODO: Custom output paths unlike the original scripts...

#[derive(clap::Subcommand, Debug)]
pub enum BindingCommand {
    /// Generates bindings based on newer versions of the game...
    New {
        andorid_symbols: std::path::PathBuf,
        cocos2d_symbols: std::path::PathBuf,
        /// Vtables obtained from ghidra
        vtables:Option<std::path::PathBuf>
    },

    /// Generates Bindings for older versions of GD...
    Old {
        /// Android Symbols laid out in a textfile these should be generated with llvm-nm 
        andorid_symbols:std::path::PathBuf,
        /// Vtables obtained from ghidra
        vtables:Option<std::path::PathBuf>
    }
}
