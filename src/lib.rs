/*!
One-line description.

More detailed description, with

# Overview

# Usage

*/

// ------------------------------------------------------------------------------------------------
// Preamble
// ------------------------------------------------------------------------------------------------

#![warn(
    missing_debug_implementations,
    missing_docs,
    unused_extern_crates,
    rust_2018_idioms
)]

#[macro_use]
extern crate lazy_static;

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod constants;

pub mod io;

pub mod model;

pub mod service;
