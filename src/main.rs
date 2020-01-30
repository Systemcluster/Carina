#![allow(
	dead_code,
	unused_imports,
	non_upper_case_globals,
	incomplete_features,
	clippy::useless_format,
	clippy::toplevel_ref_arg,
	clippy::unneeded_field_pattern,
	clippy::redundant_pattern_matching,
	clippy::len_zero,
	clippy::print_literal
)]
#![feature(
	arbitrary_self_types,
	associated_type_defaults,
	associated_type_bounds,
	box_patterns,
	box_syntax,
	c_variadic,
	concat_idents,
	const_compare_raw_pointers,
	const_fn,
	const_fn_union,
	const_generics,
	const_panic,
	const_raw_ptr_deref,
	const_raw_ptr_to_usize_cast,
	const_transmute,
	core_intrinsics,
	default_type_parameter_fallback,
	decl_macro,
	doc_alias,
	doc_cfg,
	doc_keyword,
	doc_masked,
	doc_spotlight,
	external_doc,
	exclusive_range_pattern,
	exhaustive_patterns,
	extern_types,
	fundamental,
	generators,
	generic_associated_types,
	impl_trait_in_bindings,
	in_band_lifetimes,
	infer_static_outlives_requirements,
	label_break_value,
	let_chains,
	naked_functions,
	nll,
	non_ascii_idents,
	optimize_attribute,
	optin_builtin_traits,
	overlapping_marker_traits,
	panic_runtime,
	platform_intrinsics,
	plugin,
	plugin_registrar,
	rustc_private,
	precise_pointer_size_matching,
	proc_macro_hygiene,
	repr_simd,
	repr128,
	rustc_attrs,
	simd_ffi,
	specialization,
	structural_match,
	thread_local,
	trace_macros,
	trait_alias,
	trivial_bounds,
	try_blocks,
	type_alias_impl_trait,
	type_ascription,
	unboxed_closures,
	unsized_locals,
	unsized_tuple_coercion,
	untagged_unions
)]
#![feature(
	clamp,
	coerce_unsized,
	const_cstr_unchecked,
	const_int_conversion,
	const_saturating_int_methods,
	const_type_id,
	error_iter,
	error_type_id,
	exact_size_is_empty,
	extra_log_consts,
	fn_traits,
	gen_future,
	generator_trait,
	hash_raw_entry,
	ip,
	is_sorted,
	iter_once_with,
	linked_list_extras,
	map_entry_replace,
	maybe_uninit_ref,
	maybe_uninit_slice,
	pattern,
	range_is_empty,
	shrink_to,
	slice_concat_ext,
	slice_iter_mut_as_slice,
	slice_partition_at_index,
	slice_partition_dedup,
	trusted_len,
	try_reserve,
	try_trait,
	unicode_version,
	unsize,
	vec_drain_as_slice,
	vec_remove_item,
	vec_resize_default,
	wrapping_next_power_of_two
)]

///
/// Carina Programming Language Interpreter
///

#[global_allocator]
static Allocator: std::alloc::System = std::alloc::System;

#[macro_use]
extern crate regex;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

use async_log;
use chrono::*;
use color_backtrace;
use log::*;
use pretty_env_logger;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about, author)]
struct Opt {
	#[structopt(short = "d", long = "debug", help = "Prints additional debug output")]
	debug: bool,
	#[structopt(parse(from_os_str), help = "Carina source file")]
	input: std::path::PathBuf,
}

mod parser2;

fn main() {
	color_backtrace::install();
	let opt = Opt::from_args();

	let level_default = log::LevelFilter::Info;
	let level: log::LevelFilter = std::env::var("LOG_LEVEL")
		.map(|v| str::parse(&v))
		.unwrap_or(Ok(level_default))
		.unwrap_or(level_default);
	pretty_env_logger::formatted_timed_builder()
		.filter_level(level)
		.init();
	// async_log::Logger::wrap(logger, || 0)
	// 	.start(log::LevelFilter::Trace).unwrap();

	let time_start = Utc::now();
	let success: bool;
	match parser2::parse(opt.input) {
		Err(err) => {
			error!("{:?}", err);
			success = false;
		}
		Ok(info) => {
			info!("{}", info.message);
			success = true;
		}
	}
	let time_end = Utc::now();
	let time_elapsed: Duration = time_end.signed_duration_since(time_start);
	let time_seconds;
	if let Some(t) = time_elapsed.num_nanoseconds() {
		time_seconds = t as f64 / 1000.0 / 1000.0 / 1000.0;
	} else if let Some(t) = time_elapsed.num_microseconds() {
		time_seconds = t as f64 / 1000.0 / 1000.0;
	} else {
		time_seconds = time_elapsed.num_milliseconds() as f64 / 1000.0;
	}
	if success {
		info!("✔ elapsed time: {:.5}s", time_seconds);
	} else {
		error!("✘ elapsed time: {:.5}s", time_seconds);
	}
}
