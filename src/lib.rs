#![feature(maybe_uninit_ref)]
#![feature(raw)]

// always define macros in the first module defined in
// a library, or at the top of your lib.rs or main.rs file
mod macros;

mod bufferpool;
mod cell;
mod database;
mod page;
mod page_id;
mod permissions;
mod row;
mod sequential_scan;
mod table;
mod tests;
mod transaction_id;
mod util;
