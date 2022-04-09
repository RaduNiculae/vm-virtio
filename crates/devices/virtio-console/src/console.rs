use std::io::Write;
use std::ops::Deref;
use virtio_queue::DescriptorChain;
use vm_memory::{Bytes, GuestMemory};

/// Console struct
pub struct Console {

}

impl Console {
    pub fn process_transmitq_chain<M>(desc_chain: &mut DescriptorChain<M>, output: T) -> usize
    where
        M: Deref,
        M::Target: GuestMemory,
        T: Write,
    {
        let len = 0;
        while let Some(desc) = desc_chain.next() {
            let len = len + desc_chain.memory().write_to(
                desc.addr(),
                output,
                desc.len() as usize,
            );
            let _ = out.flush();
        }

        len
    }
}