use crate::{parsing_logic::is_static_func, re};
use std::collections::{HashMap, BTreeMap};
use lazy_regex::regex_replace;



// Seems to be a boolean in the original script so I made an enum to help with that so an array check is not required...
enum Group {
    INDEX(i32),
    VTINDEX(i32)
}

#[derive(Clone, Debug)]
struct FuncTup(String, i32);

trait SignatureResults {
    fn new() -> Self;
    fn best_effort_guess<'c, 'n>(&self, class_name:&'c str, name:&'n str) -> String;
    fn vtable_index_for_func<'n, 'f>(&self, name:&'n str, func_sig:&'f str) -> Option<i32>;
    #[inline]
    fn is_virtual<'n, 'f>(&self, name:&'n str, func_sig:&'f str) -> bool{
        self.vtable_index_for_func(name, func_sig).is_some()
    }

    fn group_for_function<'c, 'f>(&self,  class_name:&'c str, func_sig:&'f str) -> Group{
        if func_sig.starts_with(&format!("{class_name}(")) || func_sig.starts_with("~"){
            return Group::INDEX(-3);
        }
        if is_static_func(&class_name, &func_sig){
            return Group::INDEX(-2);
        }

        match self.vtable_index_for_func(&class_name, &func_sig){
            Some(vi) => {return Group::VTINDEX(vi);}
            None => {return Group::INDEX(-2)}
        }
    }

    fn reorder_funcs<'c, 'f>(&self, class_name:&'c str, funcs:&'f Vec<String>) -> BTreeMap<i32, Vec<Vec<String>>>{
        
        /*
         * HashMap is an unoredered container, and keeps the items 
         * in an arbitrary order. You cannot define an order. 
         * If you want sorted iteration, use BTreeMap . 
         * If you need insertion order, you can use the IndexMap 
         * type from the indexmap crate. 11 jan 2022 
         * 
         * SEE: https://stackoverflow.com/questions/70667002/can-i-iterate-in-order-on-hashmapusize-mystruct
         */
        
        let mut first:BTreeMap<i32, Vec<FuncTup>> = BTreeMap::new();
        let mut group = 0;
        let mut order = 0;
        for sig in funcs{
           
            match self.group_for_function(class_name, &sig){
                Group::INDEX(x) => {
                    group = x;
                },
                Group::VTINDEX(x) => {
                    order = x;
                }
            }
            // TODO: Remove Clones in when first release is considered stable...
            if first.get(&group).is_none(){
                let ft = FuncTup{0:sig.clone(), 1:order.clone()};
                first.entry(group.clone()).and_modify(|v|v.push(ft.clone())).or_insert(vec![ft]);
            }
        }

        // second algorythm
    
        let mut new_funcs:BTreeMap<i32, Vec<Vec<String>>> = BTreeMap::new();
        for (k, v) in first{
            let mut inner_dict:BTreeMap<i32, Vec<String>> = BTreeMap::new();
            for func_pos in v{
                inner_dict.entry(func_pos.1).and_modify(|v|v.push(func_pos.0.clone())).or_insert(vec![func_pos.0]);
            }

            // TODO: Remove Cloning...
            for mut values in inner_dict{
                values.1.sort();
                new_funcs.entry(k).and_modify(|x|x.push(values.1.clone())).or_insert(vec![values.1]);
            }
        }
        return new_funcs;
    }
}


// everything has the same lifecycle, no need to create
// more than one variable for it...

/// Old signature results carries android symbols and ghidra vtables
pub struct OldSignatureResults {
    vtables:HashMap<String, Vec<Vec<String>>>,
    classes:HashMap<String, Vec<String>>
}


impl SignatureResults for OldSignatureResults {
    fn new() -> Self {
        Self { vtables: HashMap::new(), classes: HashMap::new() }
    }

    fn best_effort_guess<'c, 'n>(&self, class_name:&'c str, name:&'n str) -> String {
        if name.starts_with(&format!("{}(", class_name)) || name.starts_with('~') {
            return name.to_string();
        }
        // My additions CCTouch / CCEvent Callbacks...
        if (name.starts_with("ccTouchBegan(") || name.starts_with("ccTouchMoved(") || name.starts_with("ccTouchEnded(") || name.starts_with("ccTouchCancelled(")) && name.contains("(cocos2d::CCTouch*, cocos2d::CCEvent*)"){
            return format!("void {}(cocos2d::CCTouch* pTouch, cocos2d::CCEvent* pEvent)", name.split_once('(').unwrap().0, );
        }

        if name.starts_with("create(") || (name.starts_with("shared") && is_static_func(class_name, &name)){
            return format!("{class_name}* {name}");
        }

        if name.starts_with("init("){
            return format!("bool {name}");
        }

        if re::ON_CCOBJECT.is_match(&name){
            let func_name = name.split_once('(',).unwrap().0;
            return format!("void {func_name}(cocos2d::CCObject* sender)");
        }

        // cocos is skipped since were using the old version and it's very risky even in a 
        // reverse engineering scenario where we don't have any idea what robtop decided 
        // he was going to fuck around with.
        
        if re::SET_ATTR.is_match(&name){
            return format!("void {name}");
        }
        if re::IS_ATTR.is_match(&name){
            return format!("bool {name}");
        }

        if re::GET_ATTR.is_match(&name){
            // We need to find the secondary function so we can find it's return type...
            let get_name = name.to_string().clone();
            let set_name = get_name.replacen('g', "s", 1).split_once("(").unwrap().0.to_string();
            let cls_table= &self.classes[class_name];
            for func in cls_table{
                if func.starts_with(&set_name){
                    return regex_replace!(r#"set[A-Z]\w+\(([^\,\)]+)\)"#, func, |_, v | format!("{v} {name}")).to_string();
                }
            }
            /* FALLBACK!!! */
        }

        return format!("TodoReturn {}", name.to_string());
    }

    fn vtable_index_for_func<'n, 'f>(&self, name:&'n str, func_sig:&'f str) -> Option<i32> {
        let short_cpy = name.to_string();
        if self.vtables.get(&short_cpy).is_none(){
            return None;
        }
        let table = &self.vtables[&short_cpy];
        for t in table{
            for idx in 0..t.len(){
                if t[idx] == func_sig {
                    return Some(idx.try_into().unwrap());
                }
            };
        }
        return None;
    }



}





// pub fn old_filter_android_symbols(symbols:Vec<String>){
//     symbols.iter().filter(|x| re::SYMBOLS_FILTER.is_match(&x)).map(f)
// }