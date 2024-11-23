// Used for handling older releases of geometry dash that do not have a windows release associated with it.
 
use crate::logic::OldSignatureResults;

use crate::parsing_logic::old_read_lines;


pub fn do_old_version(andorid_symbols_file: std::path::PathBuf, vtables:Option<std::path::PathBuf>){
    
    let andorid_symbols = old_read_lines(&andorid_symbols_file);


}