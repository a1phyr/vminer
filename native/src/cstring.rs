use core::{
    cmp,
    ffi::c_char,
    fmt,
    ptr::{self, NonNull},
    str,
};

pub struct Formatter {
    ptr: Option<NonNull<c_char>>,
    len: usize,
    written: usize,
}

impl Formatter {
    #[inline]
    pub unsafe fn new(ptr: *mut c_char, len: usize) -> Self {
        if len != 0 {
            Self {
                ptr: NonNull::new(ptr),
                len: len - 1,
                written: 0,
            }
        } else {
            Self {
                ptr: None,
                len: 0,
                written: 0,
            }
        }
    }

    #[inline]
    pub fn finish(self) -> usize {
        unsafe {
            if let Some(ptr) = self.ptr {
                ptr.as_ptr().write(0);
            }
        }
        self.written
    }
}

impl Drop for Formatter {
    fn drop(&mut self) {
        unsafe {
            if let Some(ptr) = self.ptr {
                ptr.as_ptr().write(0);
            }
        }
    }
}

impl fmt::Write for Formatter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let ptr = match self.ptr {
            Some(ptr) => ptr.as_ptr(),
            None => return Ok(()),
        };

        let bytes = s.as_bytes();

        let n = cmp::min(bytes.len(), self.len);

        unsafe {
            ptr::copy_nonoverlapping(bytes.as_ptr(), ptr.cast(), n);
        }

        self.ptr = unsafe { NonNull::new(ptr.add(n)) };
        self.len -= n;
        self.written += n;

        if bytes.len() > n {
            Err(fmt::Error)
        } else {
            Ok(())
        }
    }

    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
        fmt::write(self, args)
    }
}

pub unsafe fn with_formatter(
    ptr: *mut c_char,
    len: usize,
    f: impl FnOnce(&mut Formatter) -> vmc::VmResult<()>,
) -> isize {
    crate::error::wrap_usize(|| {
        let mut fmt = unsafe { Formatter::new(ptr, len) };
        f(&mut fmt)?;
        Ok(fmt.finish())
    })
}
