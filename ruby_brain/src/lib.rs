#[macro_use]
extern crate ruru;
extern crate brain;

use ruru::{Class, Object, RString};
use brain::prelude;

use std::io::{self, stdin, stdout};

methods!(
    RString,
    itself,

    fn execute() -> RString {
        prelude::execute(&itself.to_string(), stdin(), stdout());
        itself
    }
);

#[no_mangle]
pub extern fn initialize() {
    Class::from_existing("String").define(|itself| {
        itself.def("execute", execute);
    });
}
