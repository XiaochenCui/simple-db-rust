use crate::bufferpool::BufferPool;
use crate::row::RowScheme;
use crate::table::table::*;

use std::collections::HashMap;

use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub static PAGE_SIZE: usize = 4096;

pub struct Database {
    catalog: Arc<RwLock<Catalog>>,
    buffer_pool: Arc<RwLock<BufferPool>>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            catalog: Arc::new(RwLock::new(Catalog::new())),
            buffer_pool: Arc::new(RwLock::new(BufferPool::new())),
        }
    }

    pub fn get_catalog(&self) -> RwLockReadGuard<Catalog> {
        self.catalog.try_read().unwrap()
    }

    pub fn get_buffer_pool(&self) -> RwLockWriteGuard<BufferPool> {
        self.buffer_pool.try_write().unwrap()
    }

    pub fn get_write_catalog(&self) -> RwLockWriteGuard<Catalog> {
        self.catalog.try_write().unwrap()
    }

    pub fn get_write_buffer_pool(&self) -> RwLockWriteGuard<BufferPool> {
        self.buffer_pool.try_write().unwrap()
    }

    pub fn add_table(table: Arc<RwLock<dyn Table>>, _table_name: &str, _primary_key: &str) {
        // add table to catolog
        // add a scope to release write lock (release lock at function return)
        let mut catlog = Database::global().get_write_catalog();
        catlog.add_table(Arc::clone(&table), "table", "");
    }
}

pub struct Catalog {
    table_id_table_map: HashMap<i32, Arc<RwLock<dyn Table>>>,
}

impl Catalog {
    fn new() -> Catalog {
        Catalog {
            table_id_table_map: HashMap::new(),
        }
    }

    pub(crate) fn get_row_scheme(&self, table_id: i32) -> Arc<RowScheme> {
        let t = self.table_id_table_map.get(&table_id);
        match t {
            Some(t) => t.try_read().unwrap().get_row_scheme(),
            None => panic!(""),
        }
    }

    pub(crate) fn add_table(
        &mut self,
        table: Arc<RwLock<Table>>,
        _table_name: &str,
        _primary_key: &str,
    ) {
        self.table_id_table_map
            .insert(table.try_read().unwrap().get_id(), Arc::clone(&table));
    }
}
