// Used for handling older releases of geometry dash that do not have a windows release associated with it.
 
use crate::logic::SignatureResults;
use crate::logic::OldSignatureResults;

use crate::parsing_logic::old_should_keep_symbol;
use crate::parsing_logic as pl;
use crate::re;
// use crate::re::GET_ATTR;
use std::collections::{HashMap, HashSet};

pub fn do_old_version(
    android_symbols_file: std::path::PathBuf, 
    mut vtables: Option<std::path::PathBuf>){
    
    println!("Loading Android symbols from \"{}\"", android_symbols_file.display());
    let android_symbols = pl::old_read_lines(&android_symbols_file);
    let mut osr = OldSignatureResults::new();
    println!("[DEBUG ANDROID SYMBOLS] {:?}", android_symbols);

    
    // TODO: Warn user about empty vtable arguments and 
    // how they will be deprecated in the future. 
    // I think it's better to have a vtable argument 
    // as mandatory since most of the time the vtables.json 
    // file could be in a completely different directory or named 
    // something completely different... - Calloc
    unsafe { 
        for capture in 
            android_symbols.iter()
                .filter(|x| pl::old_should_keep_symbol(x))
                .map(|x| re::FILTER_FUNCTIONS.captures(&x))
                .filter(|m| m.is_some())
                .map(|m|m.unwrap_unchecked())
                .filter(|x|!x.get(1).unwrap_unchecked().as_str().ends_with("::")){
                
            let groups:Vec<&str> = capture.iter().filter(|c|c.is_some()).map(|c|c.unwrap_unchecked().as_str()).collect();
            
            // Should never happen but incase it does...
            if groups.len() < 2 {
                continue;
            }

            let class_name = groups[1];
            if class_name == "cocos2d"{
                continue;
            }
            println!("[DEBUG] {class_name} -> {:?}", groups);
            osr.add_func_to_class(class_name, groups[2]);
        }
    }

    if vtables.is_none(){
        println!("WARNING: (HAVING NO VTABLES FILE WILL BE DEPRECATED IN THE FUTURE) defaulting to \"vtables.json\"");
        vtables.replace("vtables.json".into());
    }

    let virtuals: HashMap<String, serde_json::Value> = pl::read_vtables_json_file(&vtables.unwrap());
    let mut virtual_tables : HashMap<String, Vec<Vec<String>>> = HashMap::new();
    for (k, v) in &virtuals{
        let vals:Vec<Vec<String>> = v.as_array()
            .unwrap()
            .iter()
            .map(|x|x
                .as_array()
                .unwrap()
                .iter()
                .map(|x|x
                    .as_str()
                    .unwrap()
                    .to_string()
                ).collect()
            ).collect();
        virtual_tables.insert(k.to_string(), vals);
    }
    osr.set_vtables(virtual_tables);


    for name in virtuals.keys()
        .filter(|n| old_should_keep_symbol(&format!("{n}::init()"))){
        let tables = virtuals[name].as_array().expect("Ghidra configured your Vtables Wrong!");
        // println!("VTABLE {name} -> {:?}", tables);
        let pure_virts:HashSet<String> = tables.iter().map(|x|x.as_array().unwrap().iter().map(|x|x.as_str().unwrap())).flatten().filter(|x|x.starts_with("pure_virtual_")).map(String::from).collect();
        
        if pure_virts.len() > 0 {
            for p in pure_virts {
                osr.add_func_to_class(&name, &p);
            }
        }
    }

    // TODO: in the future out will get replaced with a custom directory argument 
    // along with other safety warnings about overriding previous broma scripts...
    println!("writing to out/Geometrydash.bro...");

    osr.write().save(&std::path::PathBuf::from("out/Geometrydash.bro"));
    println!("Finished");



}