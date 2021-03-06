// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// aux-build:plugin.rs
// ignore-stage1

#![feature(proc_macro)]

#[macro_use] extern crate plugin;

#[derive(Foo, Bar)] //~ ERROR proc-macro derive panicked
struct Baz {
    a: i32,
    b: i32,
}

fn main() {}
