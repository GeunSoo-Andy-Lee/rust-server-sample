pub trait BufferedStreamBase {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;

    // getter
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn is_full(&self) -> bool {
        self.max_size() == self.len()
    }

    fn limit(&self) -> usize {
        self.max_size() - self.len()
    }

    fn vacant_len(&self) -> usize {
        self.capacity() - self.tail()
    }

    fn len(&self) -> usize {
        self.tail() - self.head()
    }

    fn max_size(&self) -> usize;
    fn tail(&self) -> usize;
    fn head(&self) -> usize;
    fn capacity(&self) -> usize;

    // setter
    fn set_max_size(&mut self, value: usize);
    /// # Safety
    unsafe fn reserve(&mut self, capacity: usize);
    /// # Safety
    unsafe fn remove_front(&mut self, value: usize);
    /// # Safety
    unsafe fn set_len(&mut self, value: usize);
}

pub trait BufferedStreamReader: BufferedStreamBase {
    fn read(&self, buf: &mut [u8], len: usize) -> Option<usize>;
    fn peek(&self, buf: &mut [u8], len: usize) -> Option<usize>;
    /// # Safety
    unsafe fn as_slice(&self, pos: usize, len: usize) -> Option<&[u8]>;
    // unsafe fn seek(&self, len: usize) -> Result<&[u8], ()>;
}

pub trait BufferedStreamWriter: BufferedStreamBase {
    fn write(&mut self, buf: &[u8]) -> Option<usize>;
    /// # Safety
    unsafe fn as_slice_mut(&mut self, pos: usize, len: usize) -> Option<&mut [u8]>;
    // unsafe fn write_for_offset(&mut self, buf: &[u8]) -> Result<usize, ()>;
}
