use super::*;

// java: simpledb.systemtest.BTreeScanTest#testSmall
#[test]
fn test_small() {
    setup();

    let column_sizes = [1, 2, 3, 4, 5];
    let row_sizes = [0, 1, 2, 511, 512, 513, 1023, 1024, 1025, 4096 + 1000];

    for column_size in &column_sizes {
        for row_size in &row_sizes {
            validate_sacn(*column_size, *row_size);
        }
    }
}

fn validate_sacn(columns: i32, rows: i32) {
}