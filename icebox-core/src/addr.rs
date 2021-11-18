use super::mask;
use core::ops::Sub;
use core::ops::SubAssign;
use core::{fmt, ops::Add};

#[derive(Debug, Clone, Copy, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
#[cfg_attr(feature = "python", derive(pyo3::FromPyObject))]
#[repr(transparent)]
pub struct GuestPhysAddr(pub u64);

impl fmt::LowerHex for GuestPhysAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::UpperHex for GuestPhysAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Add<u64> for GuestPhysAddr {
    type Output = GuestPhysAddr;

    fn add(self, rhs: u64) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Add<i64> for GuestPhysAddr {
    type Output = Self;

    fn add(self, rhs: i64) -> Self {
        let (res, o) = self.0.overflowing_add(rhs as u64);

        if cfg!(debug_assertions) && (o ^ (rhs < 0)) {
            panic!("attempt to add with overflow");
        }

        Self(res)
    }
}

impl Sub<u64> for GuestPhysAddr {
    type Output = GuestPhysAddr;

    fn sub(self, rhs: u64) -> Self::Output {
        Self(self.0 - rhs)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
#[cfg_attr(feature = "python", derive(pyo3::FromPyObject))]
#[repr(transparent)]
pub struct GuestVirtAddr(pub u64);

impl GuestVirtAddr {
    #[inline]
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub const fn pml4e(self) -> u64 {
        (self.0 >> 39) & mask(9)
    }

    #[inline]
    pub const fn pdpe(self) -> u64 {
        (self.0 >> 30) & mask(9)
    }

    #[inline]
    pub const fn pde(self) -> u64 {
        (self.0 >> 21) & mask(9)
    }

    #[inline]
    pub const fn pte(self) -> u64 {
        (self.0 >> 12) & mask(9)
    }

    /// Offset for normal pages (4Ko)
    #[inline]
    pub const fn page_offset(self) -> u64 {
        self.0 & mask(12)
    }

    /// Offset for large pages (2Mo)
    #[inline]
    pub const fn large_page_offset(self) -> u64 {
        self.0 & mask(21)
    }

    /// Offset for huge pages (1Go)
    #[inline]
    pub const fn huge_page_offset(self) -> u64 {
        self.0 & mask(30)
    }
}

impl Add<u64> for GuestVirtAddr {
    type Output = GuestVirtAddr;

    fn add(self, rhs: u64) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Add<i64> for GuestVirtAddr {
    type Output = GuestVirtAddr;

    fn add(self, rhs: i64) -> Self::Output {
        let (res, o) = self.0.overflowing_add(rhs as u64);

        if cfg!(debug_assertions) && (o ^ (rhs < 0)) {
            panic!("attempt to add with overflow");
        }

        Self(res)
    }
}

impl Sub<GuestVirtAddr> for GuestVirtAddr {
    type Output = i64;

    fn sub(self, rhs: GuestVirtAddr) -> i64 {
        self.0.overflowing_sub(rhs.0).0 as i64
    }
}

impl Sub<u64> for GuestVirtAddr {
    type Output = Self;

    fn sub(self, rhs: u64) -> Self {
        Self(self.0 - rhs)
    }
}

impl SubAssign<u64> for GuestVirtAddr {
    fn sub_assign(&mut self, rhs: u64) {
        self.0 -= rhs;
    }
}

impl fmt::LowerHex for GuestVirtAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::UpperHex for GuestVirtAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(transparent)]
pub struct MmPte(pub u64);

impl MmPte {
    /// Normal pages (4Ko)
    #[inline]
    pub const fn page_frame(self) -> GuestPhysAddr {
        GuestPhysAddr(self.0 & (mask(36) << 12))
    }

    /// Large pages (2Mo)
    #[inline]
    pub const fn large_page_frame(self) -> GuestPhysAddr {
        GuestPhysAddr(self.0 & (mask(31) << 21))
    }

    /// Huge pages (1Go)
    #[inline]
    pub const fn huge_page_frame(self) -> GuestPhysAddr {
        GuestPhysAddr(self.0 & (mask(22) << 30))
    }

    #[inline]
    pub const fn is_valid(self) -> bool {
        self.0 & 1 != 0
    }

    #[inline]
    pub const fn is_large(self) -> bool {
        self.0 & (1 << 7) != 0
    }
}

impl fmt::LowerHex for MmPte {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::UpperHex for MmPte {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
