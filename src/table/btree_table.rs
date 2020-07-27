use super::*;
use super::table::*;
use std::sync::RwLock;

#[derive(Debug)]
pub struct BTreeTable {
    pub table_id: i32,
    pub file: Arc<Mutex<File>>,
    pub row_scheme: Arc<RowScheme>,
}

impl Table for BTreeTable {
    fn get_id(&self) -> i32 {
        self.table_id
    }

    fn get_row_scheme(&self) -> Arc<RowScheme> {
        Arc::clone(&self.row_scheme)
    }
}

impl BTreeTable {
    pub fn new(file_path: &str, row_scheme: RowScheme) -> BTreeTable {
        let file = File::open(file_path).unwrap();
        BTreeTable {
            table_id: 0,
            file: Arc::new(Mutex::new(file)),
            row_scheme: Arc::new(row_scheme),
        }
    }
}

pub fn create_random_btree_table(
    columns: i32,
    rows: i32,
    max_value: i32,
    _column_specification: HashMap<i32, i32>,
    new_cells: &mut Vec<Vec<i32>>,
) -> BTreeTable {
    debug!("rows count: {}", rows);

    // generate cells
    // let mut new_cells: Vec<Vec<i32>> = Vec::new();
    for _ in 0..rows {
        let mut row_cells: Vec<i32> = Vec::new();
        for _ in 0..columns {
            let value = rand::thread_rng().gen_range(1, max_value);
            row_cells.push(value);
        }
        new_cells.push(row_cells);
    }

    // write cells to a readable file
    let path = "./readable.txt";
    let mut file = File::create(path).unwrap();
    for row_cells in new_cells.iter() {
        for value in row_cells {
            file.write_fmt(format_args!("{} ", value));
        }
        file.write(b"\n");
    }

    // init table
    let row_scheme = simple_int_row_scheme(columns, "");
    let table_path = "./heap.db";
    let mut file = File::create(table_path).unwrap();
    let table: BTreeTable = BTreeTable::new(table_path, row_scheme);

    // add to catalog
    let table_pointer = Arc::new(RwLock::new(table));
    Database::add_table(Arc::clone(&table_pointer), "table", "");

    // // write root page

    // BTreeTable{}
    table
}
