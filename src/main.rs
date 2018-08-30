//
// Carina Alpha
//

#![feature(nll)] // non-lexical lifetime
#![feature(in_band_lifetimes, nll, nested_method_call)] // simplify lifetimes
#![feature(try_trait, catch_expr)] // error handling
#![feature(crate_in_paths, crate_visibility_modifier, non_modrs_mods)] // module handling
#![feature(decl_macro, proc_macro)] // macro improvements
#![feature(arbitrary_self_types)] // additional self method arguments
#![feature(generators, generator_trait)] // generators/coroutines
#![feature(fn_traits, unboxed_closures)] // function-like type traits
#![feature(never_type)] // new types
#![feature(const_fn)] // const functions
#![feature(const_generics)] // const generics

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_upper_case_globals)]

extern crate failure;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use failure::{Error};

fn main() -> Result<(), Error> {
    println!("Hello, world!");

    Ok(())
}
