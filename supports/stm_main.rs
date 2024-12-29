use embassy_executor::{Executor, Spawner};
use static_cell::StaticCell;

// Import logic_main as a crate-level module
use crate::logic_main::main_task;

static EXECUTOR: StaticCell<Executor> = StaticCell::new();

fn main() {
    let executor = EXECUTOR.init(Executor::new());
    executor.run(|spawner| {
        spawner.spawn(main_task(spawner)).unwrap();
    });
}

