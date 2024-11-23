// use std::fmt::Write;

pub struct BromaWriter {
    pub code: String
}

impl BromaWriter {
    pub (crate) fn new() -> Self{
        Self{code:"// clang-format off\n".to_string()}
    }

    pub (crate) fn declare_class<'a>(&mut self, class_name: &'a str){
        std::fmt::write(&mut self.code, format_args!("[[link(android)]]\nclass {} ", class_name))
        .expect("Format Error in BromaWriter::declare_class");
        self.code += "{\n"
    }

    #[inline]
    pub (crate) fn write<'a>(&mut self, code: &'a str){
        self.code += code;
    }

    #[inline]
    pub (crate) fn write_fmt(&mut self, args:std::fmt::Arguments<'_>){
        std::fmt::write(&mut self.code, args).expect("Format Error in BromaWriter::write_fmt");
    }

    #[inline]
    pub(crate) fn close_class_declaration(&mut self){
        self.code += "}\n\n"
    }

    pub (crate) fn save<'p>(self, path:&'p std::path::PathBuf){
        std::fs::write(&path, self.code).expect(
            &format!("writing error when trying to write code to: {}", path.display())
        );
    }

}