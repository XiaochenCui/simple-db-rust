use std::fmt::Debug;

// Debug for log
pub trait PageID: Debug {
    // serialize to a str (used as hashmap key)
    fn serialize(&self) -> &str;

    fn get_table_id(&self) -> i32;
    fn get_page_index(&self) -> i32;
}

// deserialize from a str
pub fn deserialize(s: &str) -> Result<Box<dyn PageID>, ParseError> {
    // get page_id type
    let iter = s.split_ascii_whitespace();
    let id_type = iter.next().unwrap();

    // assume HeapPageID
    let table_id = iter.next().unwrap().parse::<i32>().unwrap();
    let page_index = iter.next().unwrap().parse::<i32>().unwrap();
    let pid = HeapPageID {
        table_id,
        page_index,
    };
    Ok(Box::new(pid))
}

struct ParseError;

#[derive(Debug)]
pub struct HeapPageID {
    pub table_id: i32,
    pub page_index: i32,
}

impl HeapPageID {}

impl PageID for HeapPageID {
    fn serialize(&self) -> &str {
        &format!(
            "HeapPageID table_id:{} page_index:{}",
            self.table_id, self.page_index
        )
    }

    fn get_table_id(&self) -> i32 {
        self.table_id
    }

    fn get_page_index(&self) -> i32 {
        self.page_index
    }
}
