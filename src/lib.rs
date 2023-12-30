#![cfg_attr(test, feature(test))]

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate dirs;
extern crate glob;
extern crate nix;
extern crate structopt;
#[macro_use]
extern crate failure;
extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate backtrace;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
#[cfg(test)]
extern crate test;

#[macro_use]
pub mod macros;
pub mod bash_server;
pub mod builtins;
pub mod context_parser;
pub mod dircolor;
pub mod eval;
pub mod expand;
pub mod fuzzy;
pub mod highlight;
pub mod history;
pub mod mainloop;
pub mod parser;
pub mod path;
pub mod pattern;
pub mod process;
pub mod prompt;
pub mod shell;
pub mod theme;
pub mod utils;
pub mod variable;

lazy_static! {
    pub static ref STARTED_AT: std::time::SystemTime = std::time::SystemTime::now();
}
