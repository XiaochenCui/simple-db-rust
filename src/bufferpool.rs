use crate::page::page::Page;
use crate::database::*;
use crate::page::*;
use crate::page_id::*;
use crate::permissions::Permissions;

use crate::transaction_id::TransactionID;
// use lazy_static::__Deref;
use log::debug;
use crate::page_id;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock, RwLockWriteGuard},
};

pub struct BufferPool<'value> {
    buffer: HashMap<Key, Value<'value>>,
}

// // Use trait instead of generic, because BufferPool
// // have to accommodate PageID in different types
// type Key = Box<dyn PageID>;

type Key = String;

type Value<'a> = Arc<RwLock<dyn Page<'a>>>;

impl<'a> BufferPool<'_> {
    pub fn new() -> BufferPool<'a> {
        BufferPool {
            buffer: HashMap::new(),
        }
    }

    pub fn get_page_size(&self) -> usize {
        4096
    }

    pub fn get_page(&self, key: Key) -> Option<RwLockWriteGuard<dyn Page>> {
        // deserialize key to a page_id
        debug!("get page: {:?}", key);
        let result: Option<RwLockWriteGuard<dyn Page>>;
        let pid: Box<dyn PageID>;
        match page_id::deserialize(&key) {
            Ok(v) => pid = v,
            Err(e) => return None,
        }

        // get page form buffer
        match self.buffer.get(&key) {
            Some(v) => {
                result = Some(v.try_write().unwrap());
            }
            None => {
                // if page not exist in buffer, get it from disk
                let catlog = Database::global().get_catalog();
                let mut table = catlog.get_table(pid.get_table_id());
                let result = table.read_page(pid.get_page_index());
                let page = match result {
                    Ok(p) => p,
                    Err(e) => {
                        debug!("error: {}", e);
                        return None;
                    }
                };

                // add to buffer
                self.buffer
                    .insert(key, Arc::new(RwLock::new(page)));

                return Some(
                    self.buffer
                        .get(&key)
                        .unwrap()
                        .try_write()
                        .unwrap(),
                );
            }
        };

        return result;
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}
