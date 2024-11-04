use embedded_can::{Frame,Id};

pub fn init_can() -> Result<(),()>{
    todo!()
}

const DATA_BUFFER_SIZE : usize = 8;

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
        let mut buff = [0;DATA_BUFFER_SIZE];
        for i in 0..ld{
            buff[i] = data[i];
        }
        Some(Self{id:id.into(), dlc: ld,data:buff})
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
        self.id.into()
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
    pub fn send(&self) -> Result<(),()>{
        todo!();
    }

    pub fn recv(&self) ->CanBase{
        todo!();
    }
}
