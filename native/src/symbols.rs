use crate::error;
use alloc::boxed::Box;
use core::ffi::{CStr, c_char, c_int};
use vmc::SymbolsIndexer;

#[derive(Default)]
pub struct Symbols(pub SymbolsIndexer);

#[unsafe(no_mangle)]
pub extern "C" fn symbols_new() -> Box<Symbols> {
    Default::default()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symbols_load_from_bytes(
    indexer: &mut Symbols,
    name: *const c_char,
    data: *const u8,
    len: usize,
) -> c_int {
    error::wrap_unit(|| unsafe {
        let data = core::slice::from_raw_parts(data, len);
        let name = CStr::from_ptr(name).to_str()?.into();
        indexer.0.load_from_bytes(name, data)?;
        Ok(())
    })
}

#[cfg(feature = "std")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn symbols_load_from_file(
    indexer: &mut Symbols,
    path: *const c_char,
) -> c_int {
    error::wrap_unit(|| {
        let path = unsafe { CStr::from_ptr(path) };
        indexer.0.load_from_file(path.to_str()?)?;
        Ok(())
    })
}

#[cfg(feature = "std")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn symbols_load_dir(indexer: &mut Symbols, path: *const c_char) -> c_int {
    error::wrap_unit(|| {
        let path = unsafe { CStr::from_ptr(path) };
        indexer.0.load_dir(path.to_str()?)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn symbols_free(indexer: Option<Box<Symbols>>) {
    drop(indexer)
}
