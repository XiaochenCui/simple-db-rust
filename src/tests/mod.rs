mod scan_test;

use crate::database::*;
use crate::sequential_scan::SequentialScan;
use crate::table::*;
use crate::transaction_id::*;

use std::collections::HashMap;
use std::io::Write;
use std::panic;
use std::sync::Arc;
use std::sync::Once;
use std::sync::RwLock;

use env_logger::Builder;
use log::{debug, info};

static INIT: Once = Once::new();

fn setup() {
    INIT.call_once(init_log);

    Database::global().get_buffer_pool().clear();
}

fn init_log() {
    let mut builder = Builder::from_default_env();

    builder
        .format_timestamp_secs()
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} - {}] [{}:{}] {}",
                record.level(),
                record.target(),
                record.file().unwrap(),
                record.line().unwrap(),
                record.args()
            )
        })
        .init();
}

mod heap_table_test {}
