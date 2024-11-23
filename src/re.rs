/// Regexes For Making Geode Bindings
use lazy_regex::*;
// use lazy_regex::regex_replace_all;
// use std::borrow::Cow;


// https://crates.io/crates/lazy-regex

// Followed-Bys are not allowed in lazy-regex or so far 
// in rust so we need to make a few changes in this module so it would all work as expected of...

// GLOBALS 
pub static FILTER_FUNCTIONS:Lazy<Regex> = lazy_regex!(r"((?:cocos2d::)?(\w+::)*\w+)::([\w~]+\(.*\))");

pub static TYPEINFO_VTABLE_ETC:Lazy<Regex> = lazy_regex!(r"(?:typeinfo|vtable|thunk|guard variable)");

pub static OLD_JNI_INTERALS_CHECK: Lazy<Regex> = lazy_regex!(r#"^(?:_JNIEnv|internal|tinyxml2|cocos2d|ObjectDecoder|ObjectDecoderDelegate|pugi|__cxx|__gnu_cxx|std|llvm|tk|xml_|MD5)"#);

pub static NEW_JNI_INTERNALS_CHECK : Lazy<Regex> = lazy_regex!(r#"^(?:_JNIEnv|internal|tinyxml2|cocos2d|DS_Dictionary|ObjectDecoder|ObjectDecoderDelegate|pugi|__cxx|__gnu_cxx|std|fmt|llvm|tk|xml_|MD5|rtsha1)"#);

pub static SYMBOLS_FILTER: Lazy<Regex> = lazy_regex!(r#"((\w+::)*\w+)::([\w~]+\(.*\))"#);

pub static ON_CCOBJECT: Lazy<Regex> = lazy_regex!(r#"^on[\w]+\(cocos2d::CCObject\*\)"#);

pub static SET_ATTR: Lazy<Regex> = lazy_regex!(r#"^set[A-Z]"#);

pub static IS_ATTR: Lazy<Regex> = lazy_regex!(r#"^is[A-Z]"#);

// My addition vs generate.mjs
pub static GET_ATTR: Lazy<Regex> = lazy_regex!(r#"^get[A-Z]"#);



// Example use and help only...
// #[macro_export]
// macro_rules! vec {
//     ( $( $x:expr ),* ) => {
//         {
//             let mut temp_vec = Vec::new();
//             $(
//                 temp_vec.push($x);
//             )*
//             temp_vec
//         }
//     };
// }
// macro_rules! format_args {
//     ($fmt:expr) => {{ /* compiler built-in */ }};
//     ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
// }


/// Used to help with replacing strings through multiple 
/// regexes... This is not meant to be used publically only this module and thats it...
macro_rules! multi_single_replace {

    ($text:tt, $($re:literal, $repl:expr,)*) =>{
        $(
            $text = regex_replace_all!($re, &$text, $repl).to_string();
        )*
    }
}

/// Cleanses up function signatures. 
/// 
/// In Javascript it's this 
/// ```js 
/// function cleanFunctionSig(sig) {
///    return sig
///    .replace(/__thiscall |__cdecl /g, '')
///    .replace(/public: |private: |protected: /g, '')
///    .replace(/enum |class |struct /g, '')
///    .replace(/\(void\)/, '()')
///    .replace(/ &/g, '&')
///    .replace(/ \*/g, '*')
///    .replace(/\)const /g, ') const')
///    .replace(/,(?!\s)/g, ', ')
///    .replace(/std::basic_string<char, std::char_traits<char>, std::allocator<char> ?>/g, 'gd::string')
///    .replace(/std::string/g, 'gd::string')
///    .replace(
///        /std::set<(.*?), std::less<(.*?)>, std::allocator<(.*?)> ?>/g,
///        v => `gd::set<${v.match(/(?<=std::set<)(.*?)(?=,)/)[0]}>`
///    )
/// ```
/// 
/// 
/// In Python it's this because in it's version I shoved everything into a constant variable for speed...
/// 
/// ```python 
/// def cleanFunctionSig(sig: str):
///     for r, sub in REPLACEMENTS.items():
///     sig = re.sub(r, sub, sig)
/// return sig
/// ```
/// 
/// This is like a modified version of both of 
/// those with some added modifications to some 
/// of the regular expression to make up for the 
/// loss of some functionaility in rust
/// 
pub fn clean_function_sig(sig:String) -> String {
    let mut new_sig = sig;

    // I spent 3 hours of suffering with this one, you're welcome...
    multi_single_replace!(new_sig, 
        "^(public: |private: |protected: |enum |class |struct |__thiscall |__cdecl )"i, "",
        " &", "&",
        r" *", "*",
        r#"\(void\)"#, "()",
        r#"\)"#, ") const",
        r#"(?:,)[^\s]"#, |_| ", ",
        // string replacements
        r#"std::basic_string<char, std::char_traits<char>, std::allocator<char> ?>|std::string"#, "gd::string",
        // Let you all know i spend 2 hours trying to find out some shit turns out my problem was |c| <- this, and format!().to_string()
        // Macro didn't like it. turns out it wasn't the macro's fault it was that this | group0, group1, group2| 
        // that explains this awful fukery to me better...
        r#"std::set<(.*?), std::less<(?:.*?)>, std::allocator<(?:.*?)> ?>"#, |_, c| format!("gd::set<{}>", c),
        r#"std::vector<(.*?), std::allocator<(.*?)> ?>"#, |_, t, _| format!("gd::vector<{}>", t),
        r#"std::_Tree_const_iterator<std::_Tree_val<std::_Tree_simple_types<cocos2d::CCObject\*> ?> ?>"#, "cocos2d::CCSetIterator",
        r#"std::map<(.*?), (.*?), std::less<(?:.*?)>, std::allocator<std::pair<(?:.*?), (?:.*?)> ?> ?>"#, |_, k, v| format!("gd::map<{}, {}>", k, v),
        r#"std::unordered_map<(.*?), std::pair<double, double>, .*?> ?> ?> ?>"#, |_, k| format!("gd::unordered_map<{}, gd::pair<double, double>>", k),
        r#"std::unordered_map<(.*?), (.*?), .*?> ?> ?>"#, |_, k, v| format!("gd::unordered_map<{}, {}>", k, v),
        r#"unsinged long long"#, "uint64_t",
        r#"void \(cocos2d::CCObject::\*\)\(cocos2d::CCObject\*\)"#,"cocos2d::SEL_MenuHandler",
        r#"void \(cocos2d::CCObject::\*\)\(\)"#, "cocos2d::SEL_CallFunc",
        r#"void \(cocos2d::CCObject::\*\)\(cocos2d::CCNode\*\)"#,"cocos2d::SEL_CallFuncN",
        r#"void \(cocos2d::CCObject::\*\)\(cocos2d::CCObject\*\)"#,"cocos2d::SEL_CallFuncO",
        r#"void \(cocos2d::CCObject::\*\)\(cocos2d::CCEvent\*\)"#,"cocos2d::SEL_EventHandler",
        r#"int \(cocos2d::CCObject::\*\)\(cocos2d::CCObject\*\)"#,"cocos2d::SEL_Compare",
        r#"void \(cocos2d::CCObject::\*\)\(cocos2d::extension::CCHttpClient\*, cocos2d::extension::CCHttpResponse\*\)"#,"cocos2d::extension::SEL_HttpResponse",
        r#"void \(cocos2d::CCObject::\*\)\(float\)"#,"cocos2d::SEL_SCHEDULE",
        r#"cocos2d::_ccColor3B"#, "cocos2d::ccColor3B",
        r#"cocos2d::_ccColor4B"#, "cocos2d::ccColor4B",
        r#"cocos2d::_ccColor4F"#, "cocos2d::ccColor4F",
        r#"cocos2d::_ccVertex2F"#, "cocos2d::_ccVertex2F",
        r#"cocos2d::_ccVertex3F"#, "cocos2d::_ccVertex3F",
        // NOTE: Trailing comma was to allow me to use lambdas in the firstplace...
        // I also didn't want to deal with a seperate library for poc_macros since I am still a beigger - Calloc
        // Like they all say: "If it aint broke, don't fix it..."
        r#"cocos2d::_ccHSVValue"#, "cocos2d::ccHSVValue",
    );

    return new_sig.to_string();
}




