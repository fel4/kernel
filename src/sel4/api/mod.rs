use core::marker::PhantomData;

mod cnode;
// syscalls
pub fn sel4_Send() {}
pub fn sel4_NBSend() {}
pub fn sel4_Call() {} // sel4_Send() + sel4_Recv()
pub fn sel4_Recv() {}
pub fn sel4_Reply() {}
pub fn sel4_ReplyRecv() {} // sel4_Reply() + // sel4_Recv()
pub fn sel4_NBRecv() {}
pub fn sel4_Yield() {}

// kernel objects
pub use cnode::*;
pub struct ThreadControlBlock {}
pub struct Endpoint {}
pub struct Notification {}
pub struct VirtualAddressSpace {}
pub struct Interrupt {}
pub struct UntypedMemory {}

pub struct CapabilityPointer<T> {
    internal_type: PhantomData<T>,
}

pub type CPTR<T> = CapabilityPointer<T>;
pub type Word = u32;

pub struct MemoryRequest {
    type: Word,
    size_bits: Word
}

impl UntypedMemory {
    pub fn Retype(&mut self, req: MemoryRequest, root: CPTR<CNode>, node_index: Word, node_depth: Word, num_objects: Word) {
        unimplemented!();
    }
}