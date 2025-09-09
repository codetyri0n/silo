use crossbeam_skiplist::SkipMap;
use bytes::Bytes;
use std::*;
use std::str::Bytes;
use std::sync::atomic::AtomicUsize;

pub struct MemTable {
    data : SkipMap<Bytes, Bytes>,
    size : AtomicUsize,
    entry_count : AtomicUsize,
}

impl MemTable {
    pub fn new() -> Self {
        MemTable {
            data : SkipMap::new(),
            size : AtomicUsize::new(0),
            entry_count : AtomicUsize::new(0),
        }
    }
}

    

}