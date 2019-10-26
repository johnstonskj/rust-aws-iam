/*!
One-line description.

More detailed description, with

# Overview

# Usage

*/

// ------------------------------------------------------------------------------------------------
// Preamble
// ------------------------------------------------------------------------------------------------

/*
  The following is good hygiene, setting these flags to be warnings
  in the root of the project enables them for all modules. If you are
  going to be adding API documentation, then also add the additional
  flag `missing_docs`.
*/
#![warn(missing_debug_implementations, unused_extern_crates, rust_2018_idioms)]

#[macro_use]
extern crate lazy_static;

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod io;

pub mod model;
