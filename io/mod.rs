use prelude::*;

use core::ptr;
use traits::Std;

pub mod error;
pub use self::error::{Result, Error, ErrorKind};

/// A type used to conditionally initialize buffers passed to `Read` methods.
#[unstable(feature = "read_initializer", issue = "42788")]
#[derive(Debug)]
pub struct Initializer(bool);

impl Initializer {
    /// Returns a new `Initializer` which will zero out buffers.
    #[unstable(feature = "read_initializer", issue = "42788")]
    #[inline]
    pub fn zeroing() -> Initializer {
        Initializer(true)
    }

    /// Returns a new `Initializer` which will not zero out buffers.
    ///
    /// # Safety
    ///
    /// This may only be called by `Read`ers which guarantee that they will not
    /// read from buffers passed to `Read` methods, and that the return value of
    /// the method accurately reflects the number of bytes that have been
    /// written to the head of the buffer.
    #[unstable(feature = "read_initializer", issue = "42788")]
    #[inline]
    pub unsafe fn nop() -> Initializer {
        Initializer(false)
    }

    /// Indicates if a buffer should be initialized.
    #[unstable(feature = "read_initializer", issue = "42788")]
    #[inline]
    pub fn should_initialize(&self) -> bool {
        self.0
    }

    /// Initializes a buffer if necessary.
    #[unstable(feature = "read_initializer", issue = "42788")]
    #[inline]
    pub fn initialize(&self, buf: &mut [u8]) {
        if self.should_initialize() {
            unsafe { ptr::write_bytes(buf.as_mut_ptr(), 0, buf.len()) }
        }
    }
}

struct Guard<'a> { buf: &'a mut Vec<u8>, len: usize }

pub trait Read<STD: Std> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, STD>;

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize, STD> {
        read_to_end::<_, STD>(self, buf)
    }

    #[unstable(feature = "read_initializer", issue = "42788")]
    #[inline]
    unsafe fn initializer(&self) -> Initializer {
        Initializer::zeroing()
    }
}

// This uses an adaptive system to extend the vector when it fills. We want to
// avoid paying to allocate and zero a huge chunk of memory if the reader only
// has 4 bytes while still making large reads if the reader does have a ton
// of data to return. Simply tacking on an extra DEFAULT_BUF_SIZE space every
// time is 4,500 times (!) slower than this if the reader has a very small
// amount of data to return.
//
// Because we're extending the buffer with uninitialized data for trusted
// readers, we need to make sure to truncate that if any of this panics.
fn read_to_end<R: Read<STD> + ?Sized, STD: Std>(r: &mut R, buf: &mut Vec<u8>) -> Result<usize, STD> {
    let start_len = buf.len();
    let mut g = Guard { len: buf.len(), buf: buf };
    let ret;
    loop {
        if g.len == g.buf.len() {
            unsafe {
                g.buf.reserve(32);
                let capacity = g.buf.capacity();
                g.buf.set_len(capacity);
                r.initializer().initialize(&mut g.buf[g.len..]);
            }
        }

        match r.read(&mut g.buf[g.len..]) {
            Ok(0) => {
                ret = Ok(g.len - start_len);
                break;
            }
            Ok(n) => g.len += n,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
            Err(e) => {
                ret = Err(e);
                break;
            }
        }
    }

    ret
}
