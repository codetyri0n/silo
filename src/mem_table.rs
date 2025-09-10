use crossbeam_skiplist::SkipMap;
use bytes::Bytes;
use std::*;
use std::sync::atomic::AtomicUsize;

pub struct MemTable {
    data : SkipMap<Bytes, Bytes>,
    size : AtomicUsize,
    entry_count : AtomicUsize,
}

impl MemTable {
    pub fn new() -> Self {
        MemTable {
            data: SkipMap::new(),
            size: AtomicUsize::new(0),
            entry_count: AtomicUsize::new(0),
        }
    }

    pub fn get(&self, key: &Bytes) -> Option<Bytes> {
        if let Some(entry) = self.data.get(key) {
            Some(entry.value().clone())
        } else {
            None
        }
    }

    pub fn put(&mut self, key : Bytes, value : Bytes) -> Option<Bytes> {
        let key_len = key.len();
        let value_len = value.len();
        
        // First, check if the key already exists to determine if this is an insert or update
        let existing_entry = self.data.get(&key);
        
        // Now insert the new key-value pair (this overwrites if key exists)
        self.data.insert(key, value.clone());
        
        match existing_entry {
            Some(prev_entry) => {
                let prev_value = prev_entry.value();
                let prev_value_len = prev_value.len();
                let size_diff = value_len as isize - prev_value_len as isize;

                if size_diff > 0 {
                    self.size.fetch_add(size_diff as usize, std::sync::atomic::Ordering::Relaxed);
                } else {
                    self.size.fetch_sub(size_diff.unsigned_abs(), std::sync::atomic::Ordering::Relaxed);
                }
                
                // Return the previous value that was overwritten
                Some(prev_value.clone())
            }
            None => {
                // INSERT case: new key
                let new_size = key_len + value_len;

                self.size.fetch_add(new_size, std::sync::atomic::Ordering::Relaxed);
                self.entry_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

                None
            }
        }
    }

}



