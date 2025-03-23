#[derive(PartialEq, Debug)]
pub enum Exception {
    //IllegalInstruction(u32),
    LoadAccessFault(u64),
    StoreAMOAccessFault(u64),
}
