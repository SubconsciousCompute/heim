use heim_runtime::{self as runtime, SyncRuntime};
use heim_common::prelude::*;
use heim_disk as disk;

fn main() -> Result<()> {
    let mut rt = runtime::new().unwrap();
    for io_cnt in rt.block_collect(disk::io_counters()) {
        dbg!(io_cnt);
    }

    println!("\n\n--- Per physical disk ---\n");

    for io_cnt in rt.block_collect(disk::io_counters_physical()) {
        dbg!(io_cnt);
    }

    Ok(())
}
