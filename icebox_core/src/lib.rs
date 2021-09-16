#![cfg_attr(not(feature = "std"), no_std)]

mod addr;
pub use addr::{GuestPhysAddr, GuestVirtAddr, MmPte};

mod backend;
pub use backend::Backend;

mod error;
pub use error::{Error, MemoryAccessError, MemoryAccessResult};

mod os;
pub use os::Os;

pub const fn mask(size: u32) -> u64 {
    !(!0 << size)
}
