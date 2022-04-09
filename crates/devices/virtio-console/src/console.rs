use std::io::Write;
use std::ops::Deref;
use virtio_queue::DescriptorChain;
use vm_memory::{Bytes, GuestMemory};

/// Console struct
pub struct Console {

}

impl Console {

}
/// process transmitq chain
pub fn process_transmitq_chain<M, T>(desc_chain: &mut DescriptorChain<M>, output: &mut T) -> usize
where
    M: Deref,
    M::Target: GuestMemory,
    T: Write,
{
    let mut len = 0;
    while let Some(desc) = desc_chain.next() {
        len = len + desc_chain.memory().write_to(
            desc.addr(),
            output,
            desc.len() as usize,
        ).unwrap();
        let _ = output.flush();
    }

    len
}