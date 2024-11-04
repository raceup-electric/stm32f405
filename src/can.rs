use embedded_can::{Frame,Id};
use core::sync::atomic::{AtomicBool,Ordering};


#[allow(unused)]

const DATA_BUFFER_SIZE : usize = 8;
static INITDONE: AtomicBool = AtomicBool::new(false);

#[derive(Debug)]
pub struct CanBase {
    id: Id,
    dlc: usize,
    data: [u8;DATA_BUFFER_SIZE],
}

impl Frame for CanBase{
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        let ld = data.len();
        if ld > DATA_BUFFER_SIZE{
            return None
        }
        let mut frame = Self{id:id.into(), dlc: ld,data:[0;DATA_BUFFER_SIZE]};
        for i in 0..ld{
            frame.data[i] = data[i];
        }
        Some(frame)
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        todo!("not implemented")
    }

    fn is_extended(&self) -> bool {
        false
    }

    fn is_remote_frame(&self) -> bool {
        todo!()
    }

    fn id(&self) -> Id {
        self.id
    }

    fn dlc(&self) -> usize {
        self.dlc
    }

    fn data(&self) -> &[u8] {
        &self.data
    }
}

#[allow(unused)]
impl CanBase {
    pub fn init_can() -> Result<(),()>{
        if INITDONE.load(Ordering::Acquire){
            return Err(())
        }
        todo!()
    }

    pub fn send(&self) -> Result<(),()>{
        if !INITDONE.load(Ordering::Acquire) {
            return Err(())
        }
        todo!();
    }

    pub fn recv() ->Result<CanBase,()>{
        if !INITDONE.load(Ordering::Acquire){
            return Err(())
        }
        todo!()
    }
}
