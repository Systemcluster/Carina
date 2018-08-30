//
// Carina Alpha
//

#![feature(rust_2018_preview, used)] // rust 2018
#![feature(nll)] // non-lexical lifetime
#![feature(in_band_lifetimes)] // simplify lifetimes
#![feature(try_trait, try_blocks)] // error handling
#![feature(decl_macro)] // macro improvements
#![feature(arbitrary_self_types)] // additional self method arguments
#![feature(generators, generator_trait)] // generators/coroutines
#![feature(fn_traits, unboxed_closures)] // function-like type traits
#![feature(never_type)] // new types
#![feature(const_fn)] // const functions
#![feature(label_break_value)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_upper_case_globals)]


#[global_allocator] static Allocator: std::alloc::System = std::alloc::System;


#[macro_use] extern crate pest;
#[macro_use] extern crate pest_derive;
#[macro_use] extern crate failure;
#[macro_use] extern crate simplelog;
extern crate ring;

use failure::{Error};

fn main() -> Result<(), Error> {
    println!("Hello, world!");



    Ok(())
}
