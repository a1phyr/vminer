#[cfg(any(feature = "linux", feature = "windows"))]
mod pointer;

#[cfg(any(feature = "linux", feature = "windows"))]
macro_rules! pointer_defs {
    ( $( $core_ty:path = $ptr:ty; )* ) => {
        trait ToPointer<T> {
            fn to_pointer<Os, Ctx>(self, os: &Os, ctx: Ctx) -> Pointer<T, Os, Ctx>;
        }

        $(
            impl ToPointer<$ptr> for $core_ty {
                #[inline]
                fn to_pointer<Os, Ctx>(self, os: &Os, ctx: Ctx) -> Pointer<$ptr, Os, Ctx> {
                    Pointer::new(self.0, os, ctx)
                }
            }

            impl<Os, Ctx> From<Pointer<'_, $ptr, Os, Ctx>> for $core_ty {
                #[inline]
                fn from(ptr: Pointer<$ptr, Os, Ctx>) -> $core_ty {
                    $core_ty(ptr.addr)
                }
            }
        )*
    };
}

#[cfg(feature = "linux")]
pub mod linux;
#[cfg(feature = "linux")]
pub use linux::Linux;

#[cfg(feature = "windows")]
pub mod windows;
#[cfg(feature = "windows")]
pub use windows::Windows;

use alloc::{boxed::Box, string::String};
use core::fmt;
use vmc::VmResult;

pub trait Buildable<B: vmc::Backend>: Sized {
    fn quick_check(_backend: &B) -> Option<OsBuilder> {
        None
    }

    fn build(backend: B, builder: OsBuilder) -> VmResult<Self>;
}

#[derive(Debug, Default)]
pub struct OsBuilder {
    pub symbols: Option<vmc::SymbolsIndexer>,
    pub kpgd: Option<vmc::PhysicalAddress>,
    pub kaslr: Option<vmc::VirtualAddress>,
    pub version: Option<String>,
    pub loader: Option<Box<dyn SymbolLoader + Send + Sync>>,
}

impl OsBuilder {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn with_symbols(mut self, symbols: vmc::SymbolsIndexer) -> Self {
        self.symbols = Some(symbols);
        self
    }

    #[inline]
    pub fn with_kpgd(mut self, kpgd: vmc::PhysicalAddress) -> Self {
        self.kpgd = Some(kpgd);
        self
    }

    #[inline]
    pub fn with_kaslr(mut self, kaslr: vmc::VirtualAddress) -> Self {
        self.kaslr = Some(kaslr);
        self
    }

    #[inline]
    pub fn with_version(mut self, version: String) -> Self {
        self.version = Some(version);
        self
    }

    #[inline]
    pub fn build<B: vmc::Backend, Os: Buildable<B>>(self, backend: B) -> VmResult<Os> {
        Os::build(backend, self)
    }
}

#[inline]
pub fn os_builder() -> OsBuilder {
    OsBuilder::new()
}

pub trait SymbolLoader {
    fn load(&self, name: &str, id: &str) -> VmResult<Option<vmc::ModuleSymbols>>;
}

pub struct EmptyLoader;

impl SymbolLoader for EmptyLoader {
    fn load(&self, _name: &str, _id: &str) -> VmResult<Option<vmc::ModuleSymbols>> {
        Ok(None)
    }
}

impl fmt::Debug for dyn SymbolLoader + Send + Sync + '_ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SymbolLoader").finish_non_exhaustive()
    }
}
