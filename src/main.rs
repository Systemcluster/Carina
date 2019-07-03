#![feature(arbitrary_self_types)]
#![feature(associated_type_defaults)]
#![feature(associated_type_bounds)]
#![feature(async_await)]
#![feature(bind_by_move_pattern_guards)] 
#![feature(box_patterns, box_syntax)]
#![feature(c_variadic)]
#![feature(concat_idents)]
#![feature(const_compare_raw_pointers)]
#![feature(const_fn, const_fn_union, const_generics, const_panic,
	const_raw_ptr_deref, const_raw_ptr_to_usize_cast, const_transmute)]
#![feature(core_intrinsics)]
#![feature(default_type_parameter_fallback)]
#![feature(decl_macro)]
#![feature(doc_alias, doc_cfg, doc_keyword, doc_masked, doc_spotlight, external_doc)]
#![feature(exclusive_range_pattern, exhaustive_patterns)]
#![feature(existential_type)]
#![feature(extern_types)]
#![feature(fundamental)]
#![feature(generators)]
#![feature(generic_associated_types)]
#![feature(impl_trait_in_bindings)]
#![feature(in_band_lifetimes)]
#![feature(infer_static_outlives_requirements)]
#![feature(label_break_value)]
#![feature(naked_functions)]
#![feature(never_type)]
#![feature(nll)]
#![feature(non_ascii_idents)]
#![feature(non_exhaustive)]
#![feature(optimize_attribute)]
#![feature(optin_builtin_traits)]
#![feature(overlapping_marker_traits)]
#![feature(panic_runtime)]
#![feature(platform_intrinsics)]
#![feature(plugin, plugin_registrar, rustc_private)]
#![feature(precise_pointer_size_matching)]
#![feature(proc_macro_hygiene)]
#![feature(re_rebalance_coherence)]
#![feature(repr_simd, repr128)]
#![feature(rustc_attrs)]
#![feature(simd_ffi)]
#![feature(slice_patterns)]
#![feature(specialization)]
#![feature(structural_match)]
#![feature(thread_local)]
#![feature(trace_macros)]
#![feature(trait_alias)]
#![feature(trivial_bounds)]
#![feature(try_blocks)]
#![feature(type_alias_enum_variants)]
#![feature(type_ascription)]
#![feature(unboxed_closures)]
#![feature(unsized_locals, unsized_tuple_coercion)]
#![feature(untagged_unions)]

#![feature(await_macro)]
#![feature(clamp)]
#![feature(coerce_unsized)]
// #![feature(concat_idents_macro)]
#![feature(const_cstr_unchecked, const_int_conversion,
	// const_needs_drop, const_ptr_nonnull,
	const_saturating_int_methods, const_slice_len, const_str_as_bytes, const_str_len, const_string_new,
	const_type_id, const_vec_new)]
#![feature(error_iter, error_type_id)]
#![feature(euclidean_division)]
#![feature(exact_size_is_empty)]
#![feature(extra_log_consts)]
// #![feature(fix_error)]
#![feature(fn_traits)]
#![feature(gen_future)]
#![feature(generator_trait)]
#![feature(hash_raw_entry)]
#![feature(ip)]
#![feature(is_sorted)]
#![feature(iter_once_with)]
#![feature(linked_list_extras)]
#![feature(manually_drop_take)]
#![feature(map_entry_replace, map_get_key_value)]
#![feature(maybe_uninit_array, maybe_uninit_ref, maybe_uninit_slice)]
#![feature(pattern)]
#![feature(range_is_empty)]
#![feature(result_map_or_else)]
#![feature(shrink_to)]
#![feature(slice_concat_ext, slice_iter_mut_as_slice, slice_partition_at_index, slice_partition_dedup)]
#![feature(trusted_len)]
#![feature(try_reserve)]
#![feature(try_trait)]
#![feature(unicode_version)]
#![feature(unsize)]
#![feature(vec_drain_as_slice, vec_remove_item, vec_resize_default)]
#![feature(wait_timeout_until, wait_until)]
#![feature(weak_counts)]
#![feature(weak_ptr_eq)]
#![feature(wrapping_next_power_of_two)]


#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_upper_case_globals)]
#![allow(clippy::useless_format)]


///
/// Carina Programming Language Interpreter
///


#[global_allocator] 
static Allocator: std::alloc::System = std::alloc::System;


#[macro_use] extern crate regex;
#[macro_use] extern crate log;
#[macro_use] extern crate serde;


use log::*;
use env_logger;
use pretty_env_logger;
use async_log;
use chrono::*;
use structopt::StructOpt;
use color_backtrace;


#[derive(StructOpt, Debug)]
#[structopt(
	name = "Carina",
	about = "Carina language compiler",
	author = "Christian Sdunek <me@systemcluster.me>",
	version = "0.0.1")]
struct Opt {
	#[structopt(short = "d", long = "debug")]
	debug: bool,
	#[structopt(parse(from_os_str))]
	input: std::path::PathBuf,
}


mod parser;
mod parser2;

fn main() {
	color_backtrace::install();
	let opt = Opt::from_args();

	let level_default = log::LevelFilter::Info;
	let level: log::LevelFilter = std::env::var("LOG_LEVEL").map(|v|str::parse(&v))
		.unwrap_or(Ok(level_default))
		.unwrap_or(level_default);
	pretty_env_logger::formatted_timed_builder()
		.write_style(env_logger::WriteStyle::Always)
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
	}
	else if let Some(t) = time_elapsed.num_microseconds() {
		time_seconds = t as f64 / 1000.0 / 1000.0;
	}
	else {
		time_seconds = time_elapsed.num_milliseconds() as f64 / 1000.0;
	}
	if success {
		info!("✔ elapsed time: {:.5}s", time_seconds);
	}
	else {
		error!("✘ elapsed time: {:.5}s", time_seconds);
	}
}
