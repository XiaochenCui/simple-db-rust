use super::*;
use std::{fmt::Debug, hash::Hash};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct HeapPageID {
    pub table_id: i32,
    pub page_index: usize,
}

impl HeapPageID {}

pub trait PageID: Debug {}

impl PageID for HeapPageID {}

// pub trait Box<PageID>: PartialEq + Eq + Sized {}