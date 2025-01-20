#![no_std]
#![no_builtins]
#![crate_type = "staticlib"]
#![allow(warnings)]

extern crate alloc;

use alloc::{string::String, vec::Vec};
use bincode::{Decode, Encode};

#[derive(Debug, Clone, Encode, Decode)]
pub enum Clipboard {
    Empty,
    Text(String),
    Image(Vec<u8>),
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct ToKeyboard {
    pub cpu_usage: u8,
    pub memory_usage: u8,
    pub processes: u16,
    pub clipboard: Clipboard,
}
