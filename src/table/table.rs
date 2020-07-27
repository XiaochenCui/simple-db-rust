use crate::{page::Page, row::RowScheme};
use std::{io, sync::Arc};

pub trait Table: Sync + Send {
    fn get_id(&self) -> i32;
    fn get_row_scheme(&self) -> Arc<RowScheme>;
    fn read_page(&mut self, page_id: usize) -> Result<Arc<dyn Page>, io::Error>;
}