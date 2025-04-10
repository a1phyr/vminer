#![allow(clippy::wrong_self_convention)]

use core::{fmt, marker::PhantomData};
use vmc::{VirtualAddress, VmResult};

pub trait HasLayout<L, Ctx = KernelSpace>: vmc::Os {
    fn get_layout(&self) -> VmResult<&L>;
}

pub trait HasOffset: Copy {
    type Target;

    fn from_layout(layout: vmc::symbols::StructRef) -> Self;

    fn offset(self) -> VmResult<u64>;
}

pub trait Context: Copy {
    fn read_memory<Os: vmc::Os>(
        self,
        os: &Os,
        addr: VirtualAddress,
        buf: &mut [u8],
    ) -> VmResult<()>;
}

#[derive(Clone, Copy)]
pub struct KernelSpace;

impl Context for KernelSpace {
    #[inline]
    fn read_memory<Os: vmc::Os>(
        self,
        os: &Os,
        addr: VirtualAddress,
        buf: &mut [u8],
    ) -> VmResult<()> {
        os.read_kernel_memory(addr, buf)
    }
}

#[derive(Clone, Copy)]
pub struct ProcSpace {
    proc: vmc::Process,
    pgd: vmc::PhysicalAddress,
}

impl Context for ProcSpace {
    #[inline]
    fn read_memory<Os: vmc::Os>(
        self,
        os: &Os,
        addr: VirtualAddress,
        buf: &mut [u8],
    ) -> VmResult<()> {
        if addr.is_kernel() {
            os.read_kernel_memory(addr, buf)
        } else {
            os.read_process_memory(self.proc, self.pgd, addr, buf)
        }
    }
}

pub trait Readable: Sized {
    fn read<Os: vmc::Os, Ctx: Context>(os: &Os, ctx: Ctx, addr: VirtualAddress) -> VmResult<Self>;
}

impl<T: bytemuck::Pod> Readable for T {
    #[inline]
    fn read<Os: vmc::Os, Ctx: Context>(os: &Os, ctx: Ctx, addr: VirtualAddress) -> VmResult<Self> {
        let mut value = bytemuck::Zeroable::zeroed();
        ctx.read_memory(os, addr, bytemuck::bytes_of_mut(&mut value))?;
        Ok(value)
    }
}

pub struct RawPointer<T> {
    pub addr: VirtualAddress,
    _typ: PhantomData<T>,
}

pub struct Pointer<'a, T, Os, Ctx = KernelSpace> {
    pub addr: VirtualAddress,
    pub os: &'a Os,
    pub ctx: Ctx,
    _typ: PhantomData<T>,
}

impl<T, Os, Ctx: Copy> Clone for Pointer<'_, T, Os, Ctx> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, Os, Ctx: Copy> Copy for Pointer<'_, T, Os, Ctx> {}

impl<T, U, Os, Ctx> PartialEq<Pointer<'_, U, Os, Ctx>> for Pointer<'_, T, Os, Ctx> {
    #[inline]
    fn eq(&self, other: &Pointer<U, Os, Ctx>) -> bool {
        self.addr == other.addr
    }
}

impl<T, Os, Ctx> Eq for Pointer<'_, T, Os, Ctx> {}

impl<T, Os, Ctx> fmt::Debug for Pointer<'_, T, Os, Ctx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.addr.fmt(f)
    }
}

impl<'a, T, Os, Ctx> Pointer<'a, T, Os, Ctx> {
    #[inline]
    pub fn new(addr: VirtualAddress, os: &'a Os, ctx: Ctx) -> Self {
        Pointer {
            addr,
            os,
            ctx,
            _typ: PhantomData,
        }
    }

    #[inline]
    pub fn is_null(self) -> bool {
        self.addr.is_null()
    }

    #[inline]
    pub fn map_non_null<U>(self, f: impl FnOnce(Self) -> VmResult<U>) -> VmResult<Option<U>> {
        if self.addr.is_null() {
            Ok(None)
        } else {
            Some(f(self)).transpose()
        }
    }

    #[inline]
    pub fn switch_context<N>(self, ctx: N) -> Pointer<'a, T, Os, N> {
        Pointer::new(self.addr, self.os, ctx)
    }

    #[inline]
    pub fn monomorphize(self) -> Pointer<'a, T::Mono, Os, Ctx>
    where
        T: Monomorphize,
    {
        Pointer::new(self.addr, self.os, self.ctx)
    }

    #[inline]
    pub fn cast<U>(self) -> Pointer<'a, U, Os, Ctx> {
        Pointer::new(self.addr, self.os, self.ctx)
    }
}

impl<'a, T, Os: vmc::Os, Ctx: Context> Pointer<'a, T, Os, Ctx> {
    #[inline]
    #[allow(dead_code)]
    pub fn read(self) -> VmResult<T>
    where
        T: Readable,
    {
        T::read(self.os, self.ctx, self.addr)
    }

    #[inline]
    pub fn field<U, F, P>(self, get_offset: F) -> VmResult<Pointer<'a, U, Os, Ctx>>
    where
        F: FnOnce(&T) -> P,
        P: HasOffset<Target = U>,
        Os: HasLayout<T, Ctx>,
    {
        if self.addr.is_null() {
            return Err(vmc::VmError::deref_null_ptr());
        }

        let offset = get_offset(self.os.get_layout()?).offset()?;
        Ok(Pointer::new(self.addr + offset, self.os, self.ctx))
    }

    #[inline]
    pub fn read_field<U, F, P>(self, get_offset: F) -> VmResult<U>
    where
        F: FnOnce(&T) -> P,
        P: HasOffset<Target = U>,
        U: Readable,
        Os: HasLayout<T, Ctx>,
    {
        if self.addr.is_null() {
            return Err(vmc::VmError::deref_null_ptr());
        }

        let offset = get_offset(self.os.get_layout()?).offset()?;
        U::read(self.os, self.ctx, self.addr + offset)
    }

    #[inline]
    pub fn read_pointer_field<U, F, P>(self, get_offset: F) -> VmResult<Pointer<'a, U, Os, Ctx>>
    where
        F: FnOnce(&T) -> P,
        P: HasOffset<Target = RawPointer<U>>,
        Os: HasLayout<T, Ctx>,
    {
        if self.addr.is_null() {
            return Err(vmc::VmError::deref_null_ptr());
        }

        let offset = get_offset(self.os.get_layout()?).offset()?;
        let addr = VirtualAddress::read(self.os, self.ctx, self.addr + offset)?;
        Ok(Pointer::new(addr, self.os, self.ctx))
    }

    #[inline]
    pub fn switch_to_userspace(
        self,
        proc: vmc::Process,
    ) -> VmResult<Pointer<'a, T, Os, ProcSpace>> {
        let pgd = self.os.process_pgd(proc)?;
        let proc_space = ProcSpace { proc, pgd };
        Ok(self.switch_context(proc_space))
    }
}

pub trait Monomorphize {
    type Mono;
}
