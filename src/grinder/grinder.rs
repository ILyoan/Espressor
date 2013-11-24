#[link(name = "grinder",
       package_id = "grinder",
       vers = "0.1-pre")];

#[crate_type = "lib"];

extern mod ast;

pub mod reader;
pub mod lexer;
pub mod parser;
mod token;
mod util;
