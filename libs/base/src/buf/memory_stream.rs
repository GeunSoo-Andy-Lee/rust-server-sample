
pub struct MemoryStream {
    buf: Vec<u8>,
}

impl MemoryStream {
    pub fn new() -> Self {
        Self {
            buf: Vec::<u8>::new()
        }
    }

    pub fn clear(&mut self) {

    }

    pub fn write<T>(&mut self, data: T) {

    }
}