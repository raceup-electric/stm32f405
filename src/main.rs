use embassy_executor::{Executor, Spawner};
use static_cell::StaticCell;

static EXECUTOR: StaticCell<Executor> = StaticCell::new();

#[embassy_executor::task]
async fn main_taks(spawner: Spawner){
}

fn main() {
    let executor = EXECUTOR.init(Executor::new());
    executor.run(|spawner|{
        spawner.spawn(main_taks(spawner)).unwrap();
    });
}
