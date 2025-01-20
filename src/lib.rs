#![no_std]
#![no_builtins]
#![crate_type = "staticlib"]
#![allow(warnings)]

extern crate alloc;

use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Clipboard {
    Empty,
    Text(String),
    Image(Vec<u8>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToKeyboard {
    pub cpu_usage: u8,
    pub memory_usage: u8,
    pub processes: u16,
    pub clipboard: Clipboard,
}
