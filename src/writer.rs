// use std::fmt::Write;

pub struct BromaWriter {
    pub code: String,
}

impl BromaWriter {
    pub(crate) fn new() -> Self {
        // Immeditately turn off clang formatting...
        // Give a few lines for anything required by other devs in the prolouge ...
        Self {
            code: "// clang-format off\n\n\n".to_string(),
        }
    }

    pub(crate) fn declare_class<'a>(&mut self, class_name: &'a str) {
        std::fmt::write(
            &mut self.code,
            format_args!("[[link(android)]]\nclass {} ", class_name),
        )
        .expect("Format Error in BromaWriter::declare_class");
        self.code += "{\n"
    }

    #[inline]
    pub(crate) fn write<'a>(&mut self, code: &'a str) {
        self.code += code;
    }

    #[inline]
    pub(crate) fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) {
        std::fmt::write(&mut self.code, args).expect("Format Error in BromaWriter::write_fmt");
    }

    #[inline]
    pub(crate) fn close_class_declaration(&mut self) {
        // Give a few lines of wiggle-room after class declaration finishes
        // for other developers to be able to document things...
        self.code += "}\n\n"
    }

    pub(crate) fn save<'p>(self, path: &'p std::path::PathBuf) {
        std::fs::write(&path, self.code).expect(&format!(
            "writing error when trying to write code to: {}",
            path.display()
        ));
    }
}
