use crate::{page::Page, row::RowScheme};
use std::{io, sync::Arc};

pub trait Table<'a> {
    fn get_id(&self) -> i32;
    fn get_row_scheme(&self) -> Arc<RowScheme>;
    fn read_page(&'a mut self, page_id: i32) -> Result<Arc<dyn Page>, io::Error>;
}