use crate::{
    Architecture, Memory, MemoryAccessResult, PhysicalAddress, TranslationResult, VirtualAddress,
    VmResult, arch, mem::MemoryMap,
};

pub fn default_read_virtual_memory<B: Backend + ?Sized>(
    backend: &B,
    mmu_addr: PhysicalAddress,
    addr: VirtualAddress,
    buf: &mut [u8],
) -> TranslationResult<()> {
    let addr = backend.virtual_to_physical(mmu_addr, addr)?;
    backend.read_physical(addr, buf)?;
    Ok(())
}

pub trait Backend: Memory + arch::HasVcpus {
    #[inline]
    fn read_virtual_memory(
        &self,
        mmu_addr: PhysicalAddress,
        addr: VirtualAddress,
        buf: &mut [u8],
    ) -> TranslationResult<()> {
        default_read_virtual_memory(self, mmu_addr, addr, buf)
    }

    #[inline]
    fn read_value_virtual<T: bytemuck::Pod>(
        &self,
        mmu_addr: PhysicalAddress,
        addr: VirtualAddress,
    ) -> TranslationResult<T>
    where
        Self: Sized,
    {
        let mut value = bytemuck::Zeroable::zeroed();
        self.read_virtual_memory(mmu_addr, addr, bytemuck::bytes_of_mut(&mut value))?;
        Ok(value)
    }

    #[inline]
    fn virtual_to_physical(
        &self,
        mmu_addr: PhysicalAddress,
        addr: VirtualAddress,
    ) -> TranslationResult<PhysicalAddress> {
        self.arch().virtual_to_physical(self, mmu_addr, addr)
    }

    #[inline]
    fn find_kernel_pgd(
        &self,
        use_per_cpu: bool,
        additional: &[VirtualAddress],
    ) -> VmResult<PhysicalAddress> {
        self.arch()
            .find_kernel_pgd(self, self, use_per_cpu, additional)?
            .ok_or_else(|| "could not find kernel page directory".into())
    }

    #[inline]
    fn find_in_kernel_memory(
        &self,
        mmu_addr: PhysicalAddress,
        needle: &[u8],
    ) -> MemoryAccessResult<Option<VirtualAddress>> {
        self.arch().find_in_kernel_memory(self, mmu_addr, needle)
    }

    #[inline]
    fn iter_in_kernel_memory<'a, 'b>(
        &'a self,
        mmu_addr: PhysicalAddress,
        needle: &'b [u8],
    ) -> KernelSearchIterator<'a, 'b, Self>
    where
        Self: Sized,
    {
        KernelSearchIterator {
            backend: self,
            mmu_addr,
            finder: memchr::memmem::Finder::new(needle),
            base_search_addr: self.arch().kernel_base(),
            buffer: alloc::vec![0; (2 << 20) + needle.len()],
        }
    }
}

impl<B: Backend + ?Sized> Backend for alloc::sync::Arc<B> {
    #[inline]
    fn read_virtual_memory(
        &self,
        mmu_addr: PhysicalAddress,
        addr: VirtualAddress,
        buf: &mut [u8],
    ) -> TranslationResult<()> {
        (**self).read_virtual_memory(mmu_addr, addr, buf)
    }

    #[inline]
    fn virtual_to_physical(
        &self,
        mmu_addr: PhysicalAddress,
        addr: VirtualAddress,
    ) -> TranslationResult<PhysicalAddress> {
        (**self).virtual_to_physical(mmu_addr, addr)
    }

    #[inline]
    fn find_kernel_pgd(
        &self,
        use_per_cpu: bool,
        additional: &[VirtualAddress],
    ) -> VmResult<PhysicalAddress> {
        (**self).find_kernel_pgd(use_per_cpu, additional)
    }

    #[inline]
    fn find_in_kernel_memory(
        &self,
        mmu_addr: PhysicalAddress,
        needle: &[u8],
    ) -> MemoryAccessResult<Option<VirtualAddress>> {
        (**self).find_in_kernel_memory(mmu_addr, needle)
    }
}

#[derive(Debug)]
pub struct RuntimeBackend<B>(pub B);

impl<B: Backend> Memory for RuntimeBackend<B> {
    #[inline]
    fn memory_mappings(&self) -> &[MemoryMap] {
        self.0.memory_mappings()
    }

    #[inline]
    fn is_valid(&self, addr: PhysicalAddress, size: usize) -> bool {
        self.0.is_valid(addr, size)
    }

    #[inline]
    fn read_physical(&self, addr: PhysicalAddress, buf: &mut [u8]) -> MemoryAccessResult<()> {
        self.0.read_physical(addr, buf)
    }

    #[inline]
    fn search(
        &self,
        addr: PhysicalAddress,
        page_size: u64,
        finder: &memchr::memmem::Finder,
        buf: &mut [u8],
    ) -> MemoryAccessResult<Option<u64>> {
        self.0.search(addr, page_size, finder, buf)
    }
}

impl<B: Backend> arch::HasVcpus for RuntimeBackend<B> {
    type Arch = arch::RuntimeArchitecture;

    fn arch(&self) -> Self::Arch {
        self.0.arch().into_runtime()
    }

    fn vcpus_count(&self) -> usize {
        self.0.vcpus_count()
    }

    fn registers(
        &self,
        vcpu: arch::VcpuId,
    ) -> crate::VcpuResult<<Self::Arch as Architecture>::Registers> {
        self.0.registers(vcpu).map(Into::into)
    }

    fn special_registers(
        &self,
        vcpu: crate::VcpuId,
    ) -> crate::VcpuResult<<Self::Arch as Architecture>::SpecialRegisters> {
        self.0.special_registers(vcpu).map(Into::into)
    }

    fn other_registers(
        &self,
        vcpu: crate::VcpuId,
    ) -> crate::VcpuResult<<Self::Arch as Architecture>::OtherRegisters> {
        self.0.other_registers(vcpu).map(Into::into)
    }

    fn instruction_pointer(&self, vcpu: arch::VcpuId) -> crate::VcpuResult<VirtualAddress> {
        self.0.instruction_pointer(vcpu)
    }

    fn stack_pointer(&self, vcpu: arch::VcpuId) -> crate::VcpuResult<VirtualAddress> {
        self.0.stack_pointer(vcpu)
    }

    fn base_pointer(&self, vcpu: arch::VcpuId) -> crate::VcpuResult<Option<VirtualAddress>> {
        self.0.base_pointer(vcpu)
    }

    fn pgd(&self, vcpu: arch::VcpuId) -> crate::VcpuResult<PhysicalAddress> {
        self.0.pgd(vcpu)
    }

    fn kernel_per_cpu(&self, vcpu: arch::VcpuId) -> crate::VcpuResult<Option<VirtualAddress>> {
        self.0.kernel_per_cpu(vcpu)
    }
}

impl<B: Backend> Backend for RuntimeBackend<B> {
    #[inline]
    fn read_virtual_memory(
        &self,
        mmu_addr: PhysicalAddress,
        addr: VirtualAddress,
        buf: &mut [u8],
    ) -> TranslationResult<()> {
        self.0.read_virtual_memory(mmu_addr, addr, buf)
    }

    #[inline]
    fn read_value_virtual<T: bytemuck::Pod>(
        &self,
        mmu_addr: PhysicalAddress,
        addr: VirtualAddress,
    ) -> TranslationResult<T> {
        self.0.read_value_virtual(mmu_addr, addr)
    }

    #[inline]
    fn virtual_to_physical(
        &self,
        mmu_addr: PhysicalAddress,
        addr: VirtualAddress,
    ) -> TranslationResult<PhysicalAddress> {
        self.0.virtual_to_physical(mmu_addr, addr)
    }

    #[inline]
    fn find_kernel_pgd(
        &self,
        use_per_cpu: bool,
        additional: &[VirtualAddress],
    ) -> VmResult<PhysicalAddress> {
        self.0.find_kernel_pgd(use_per_cpu, additional)
    }

    #[inline]
    fn find_in_kernel_memory(
        &self,
        mmu_addr: PhysicalAddress,
        needle: &[u8],
    ) -> MemoryAccessResult<Option<VirtualAddress>> {
        self.0.find_in_kernel_memory(mmu_addr, needle)
    }
}

#[derive(Debug)]
pub struct KernelSearchIterator<'a, 'b, B: ?Sized> {
    backend: &'a B,
    finder: memchr::memmem::Finder<'b>,
    buffer: alloc::vec::Vec<u8>,
    mmu_addr: PhysicalAddress,
    base_search_addr: VirtualAddress,
}

impl<B: Backend + ?Sized> Iterator for KernelSearchIterator<'_, '_, B> {
    type Item = VmResult<VirtualAddress>;

    fn next(&mut self) -> Option<VmResult<VirtualAddress>> {
        let result = self
            .backend
            .arch()
            .find_in_kernel_memory_raw(
                &self.backend,
                self.mmu_addr,
                self.base_search_addr,
                &self.finder,
                &mut self.buffer,
            )
            .transpose()?;

        Some(match result {
            Ok(addr) => {
                self.base_search_addr = addr + 1u64;
                Ok(addr)
            }
            Err(err) => {
                self.base_search_addr += 1u64;
                Err(err.into())
            }
        })
    }
}
