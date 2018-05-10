//
// Carina Alpha
// 

#![feature(inclusive_range_syntax)] // range syntax additions
#![feature(target_feature, cfg_target_feature)] // target feature branching
#![feature(match_default_bindings, match_beginning_vert)] // simplify matching
#![feature(underscore_lifetimes, in_band_lifetimes, nll, nested_method_call)] // simplify lifetimes
#![feature(universal_impl_trait, conservative_impl_trait, dyn_trait)] // impl trait
#![feature(copy_closures, clone_closures)] // closures enhancement
#![feature(try_trait, termination_trait, catch_expr)] // error handling
#![feature(use_nested_groups, crate_in_paths, crate_visibility_modifier, non_modrs_mods)] // module handling
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

#[macro_use] extern crate pest;
#[macro_use] extern crate pest_derive;
#[macro_use] extern crate failure;
#[macro_use] extern crate log;
extern crate simplelog;
extern crate unicode_segmentation as unicode;
extern crate time;
extern crate rand;

mod preprocessor;

use pest::Parser;
#[cfg(debug_assertions)]
const grammar: &str = include_str!("./carina-alpha.pest");
#[derive(Parser)]
#[grammar = "./carina-alpha.pest"]
struct CarinaParser;

const code: &str = include_str!("../resources/sample1.ca");

fn main() {
	simplelog::SimpleLogger::init(simplelog::LogLevelFilter::Debug, simplelog::Config::default()).ok();
	debug!("Let's Go!");



	let test1 = r#"
lets go!
	indented 1
		indented 2

		indented 2 as well!

	double indent following:
			double indent! but should only have one indent
		this should have one indent following one closing indent
hey!
	"#;
	let test2 = r#"
yep.
		noo!!!
	foo
	mama mia

	lel meguka
	ha!

lets test a triple:
			harhar!
		oy vey
	ohno

ayylmao :: yo!
	foo: 1
	b: 
		lel kappa pride
gachi
		gasm"#;

	debug!("{}", preprocessor::preprocess_source(test2));
	let np = time::PreciseTime::now();
	for _ in 0..100000 {
		let i = rand::random::<bool>();
		if i {
			preprocessor::preprocess_source(test1);
		}
		else {
			preprocessor::preprocess_source(test2);
		}
		// preprocessor::preprocess_source(test1);
	}
	let na = time::PreciseTime::now();
	info!("preprocess took {:?}", np.to(na));

	// let parser = CarinaParser::parse_str(Rule::function, r#"fn abc: T -> G'"#).unwrap_or_else(|e| panic!("{}", e));
	// for pair in parser {
	// 	println!("Rule:    {:?}", pair.as_rule());
    //     println!("Span:    {:?}", pair.clone().into_span());
    //     println!("Text:    {}",   pair.clone().into_span().as_str());
	// 	for inner_pair in pair.into_inner() {
    //         println!("{:?}", inner_pair);
    //     }
	// }
}


#[test]
fn grammar_block_empty() {
	parses_to! {
		parser: CarinaParser,
		input: "\x1B\x12 \x1B\x14",
		rule: Rule::test_block_empty,
		tokens: [
			blockstart(0,2),
			blockend(3,5)
		]
	}
}


// /*
// 	integer literals tests
// */
// #[test]
// fn literal_int_1n() {
// 	parses_to! {
// 		parser: CarinaParser,
// 		input: "5",
// 		rule: Rule::int,
// 		tokens: [
// 			int(0, 1, [])
// 		]
// 	}
// }
// #[test]
// fn literal_int_5n() {
// 	parses_to! {
// 		parser: CarinaParser,
// 		input: "59338",
// 		rule: Rule::int,
// 		tokens: [
// 			int(0, 5, [])
// 		]
// 	}
// }
// #[test]
// fn literal_int_signed() {
// 	parses_to! {
// 		parser: CarinaParser,
// 		input: "+10",
// 		rule: Rule::int,
// 		tokens: [
// 			int(0, 3, [
// 				plus(0, 1)
// 			])
// 		]
// 	}
// }
// /*
// 	floating point literals tests
// */
// #[test]
// fn literal_float_point_0n() {
// 	parses_to! {
// 		parser: CarinaParser,
// 		input: "10.",
// 		rule: Rule::float,
// 		tokens: [
// 			float(0, 3, [
// 				int(0, 2)
// 			])
// 		]
// 	}
// }
// #[test]
// fn literal_float_point_5n() {
// 	parses_to! {
// 		parser: CarinaParser,
// 		input: "5.55555",
// 		rule: Rule::float,
// 		tokens: [
// 			float(0, 7, [
// 				int(0, 1),
// 				int(2, 7)
// 			])
// 		]
// 	}
// }
// #[test]
// fn literal_float_exp() {
// 	parses_to! {
// 		parser: CarinaParser,
// 		input: "28e30",
// 		rule: Rule::float,
// 		tokens: [
// 			float(0, 5, [
// 				int(0, 2),
// 				exp(2, 5, [
// 					int(3, 5)
// 				])
// 			])
// 		]
// 	}
// }
// #[test]
// fn literal_float_point_exp() {
// 	parses_to! {
// 		parser: CarinaParser,
// 		input: "2.e30",
// 		rule: Rule::float,
// 		tokens: [
// 			float(0, 5, [
// 				int(0, 1),
// 				exp(2, 5, [
// 					int(3, 5)
// 				])
// 			])
// 		]
// 	}
// }
// #[test]
// fn literal_float_point_exp_sign() {
// 	parses_to! {
// 		parser: CarinaParser,
// 		input: "00.e+2",
// 		rule: Rule::float,
// 		tokens: [
// 			float(0, 6, [
// 				int(0, 2),
// 				exp(3, 6, [
// 					plus(4, 5),
// 					int(5, 6)
// 				])
// 			])
// 		]
// 	}
// }
// /*
// 	string literal tests
// */
// #[test]
// fn literal_string_line() {
// 	parses_to! {
// 		parser: CarinaParser,
// 		input: r#"""string!!"#,
// 		rule: Rule::string,
// 		tokens: [
// 			string(0, 10, [
// 				string_line(0, 10, [
// 					eol(10, 10)
// 				])
// 			])
// 		]
// 	}
// }

// #[test]
// fn literal_string_inline() {
// 	parses_to! {
// 		parser: CarinaParser,
// 		input: r#""st\"ng!!""#,
// 		rule: Rule::string,
// 		tokens: [
// 			string(0, 10, [
// 				string_inline(0, 10, [
// 					string_inline_raw(1, 3),
// 					string_escape(3, 5),
// 					string_inline_raw(5, 9)
// 				])
// 			])
// 		]
// 	}
// }
// #[test]
// fn literal_ident() {
// 	parses_to! {
// 		parser: CarinaParser,
// 		input: "lel_marisa",
// 		rule: Rule::ident,
// 		tokens: [
// 			ident(0, 10)
// 		]
// 	}
// }
// #[test]
// fn literal_ident_1() {
// 	parses_to! {
// 		parser: CarinaParser,
// 		input: "i",
// 		rule: Rule::ident,
// 		tokens: [
// 			ident(0, 1)
// 		]
// 	}
// }

// #[test]
// fn function_simple() {
// 	parses_to! {
// 		parser: CarinaParser,
// 		input: r#"fn abc: T -> G'"#,
// 		rule: Rule::function,
// 		tokens: [
// 			function(0, 15, [
// 				ident(3, 6),
// 				decl_var(6, 7),
// 				ident(8, 9),
// 				return_specify(10, 12),
// 				ident(13, 14),
// 				breakeval(14, 15)
// 			])
// 		]
// 	}
// }
