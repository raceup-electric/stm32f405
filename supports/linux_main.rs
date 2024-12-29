use embassy_executor::Executor;
use static_cell::StaticCell;
use logic_root::main_taks;

static EXECUTOR: StaticCell<Executor> = StaticCell::new();

fn main() {
    let executor = EXECUTOR.init(Executor::new());
    executor.run(|spawner|{
        spawner.spawn(main_taks()).unwrap();
    });
}
