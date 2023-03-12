use core::slice;
use std::alloc::{alloc, dealloc, Layout};
use std::cell::Cell;
use std::ptr;

use super::base::{BufferedStreamBase, BufferedStreamReader, BufferedStreamWriter};

pub struct BufferedStream {
    ptr: *mut u8,
    capacity: usize,
    head: Cell<usize>, // atomic
    tail: Cell<usize>, // atomic
    max_size: usize,
}

impl BufferedStreamBase for BufferedStream {
    fn new() -> Self {
        Self::new_in(super::constant::DEFAULT_BUFFER_MIN_SIZE)
    }

    fn with_capacity(capacity: usize) -> Self {
        Self::new_in(capacity)
    }

    fn capacity(&self) -> usize {
        self.capacity
    }

    fn head(&self) -> usize {
        self.head.get()
    }

    fn tail(&self) -> usize {
        self.tail.get()
    }

    fn max_size(&self) -> usize {
        self.max_size
    }

    fn set_max_size(&mut self, value: usize) {
        let max_size = {
            if value < super::constant::DEFAULT_BUFFER_MIN_SIZE {
                super::constant::DEFAULT_BUFFER_MIN_SIZE
            } else {
                value
            }
        };

        self.max_size = max_size;
    }

    /// # Safety
    unsafe fn reserve(&mut self, capacity: usize) {
        let cap = self.capacity + capacity;
        if cap > self.max_size {
            return;
        }

        // 새로운 버퍼 생성
        let ptr = unsafe {
            let layout = Layout::from_size_align_unchecked(cap, std::mem::size_of::<u8>());
            alloc(layout) as *mut u8
        };

        // 기존 버퍼 내용 고대로 새로운 버퍼에 복사
        ptr::copy(self.ptr, ptr, self.capacity());
        dealloc(
            self.ptr as *mut u8,
            Layout::from_size_align_unchecked(self.capacity(), std::mem::size_of::<u8>()),
        );

        // ptr 수정
        self.set_ptr(ptr);
        // capacity 변경
        self.set_capacity(cap);
    }

    // as_slice 사용한 이후에 head 땡기는 용도
    /// # Safety
    unsafe fn remove_front(&mut self, value: usize) {
        self.head.set(value);
    }

    // as_slice_mut 해서 사용 하기 전에 tail 미리 땡기는 용도
    /// # Safety
    unsafe fn set_len(&mut self, value: usize) {
        self.tail.set(value);
    }
}

impl BufferedStreamReader for BufferedStream {
    fn read(&self, buf: &mut [u8], len: usize) -> Option<usize> {
        let result = self.peek(buf, len)?;
        self.set_head(self.head() + len);
        Some(result)
    }

    fn peek(&self, buf: &mut [u8], len: usize) -> Option<usize> {
        // 읽을 byte 보다 stream buffer에 적게 들어 있거나, read buffer size 가 읽으려는 byte 보다 작으면 에러
        if len > self.len() || buf.len() < len {
            return None;
        }

        unsafe {
            ptr::copy(self.ptr.add(self.head()), buf.as_mut_ptr(), len);
        }
        Some(len)
    }

    unsafe fn as_slice(&self, pos: usize, len: usize) -> Option<&[u8]> {
        if self.head() > pos || self.tail() < (pos + len) {
            return None;
        }

        let result = slice::from_raw_parts(self.ptr.add(pos), len);
        Some(result)
    }

    // // required before set read offset
    // unsafe fn seek(&self, len: usize) -> Result<&[u8], ()> {
    //     // tail - read_offset
    //     if len > (self.origin.tail() - self.offset()) {
    //         Err(())
    //     } else {
    //         Ok(slice::from_raw_parts(
    //             self.origin.get_mut().ptr.add(self.offset()),
    //             len,
    //         ))
    //     }
    // }
}

impl BufferedStreamWriter for BufferedStream {
    fn write(&mut self, buf: &[u8]) -> Option<usize> {
        // grow 와 compaction 을 해도 부족한 경우 에러
        if (self.capacity() - self.tail()) < buf.len() {
            self.handle_grow(buf.len())?;
        }

        unsafe {
            ptr::copy(buf.as_ptr(), self.ptr.add(self.tail()), buf.len());
        }

        self.set_tail(self.tail() + buf.len());

        Some(buf.len())
    }

    /// required before reserve
    /// # Safety
    unsafe fn as_slice_mut(&mut self, pos: usize, len: usize) -> Option<&mut [u8]> {
        if self.capacity() < (pos + len) {
            return None;
        }

        let result = slice::from_raw_parts_mut(self.ptr.add(pos), len);
        Some(result)
    }

    // // required before reserve and set write offset
    // unsafe fn write_for_offset(&mut self, buf: &[u8]) -> Result<usize, ()> {
    //     if (self.origin.capacity() - self.origin.tail()) < buf.len() {
    //         Err(())
    //     } else {
    //         ptr::copy(buf.as_ptr(), self.origin.ptr.add(self.offset()), buf.len());
    //         let new_tail = self.offset() + buf.len();
    //         if self.origin.tail() < new_tail {
    //             self.origin.set_tail(new_tail);
    //         }
    //         Ok(buf.len())
    //     }
    // }
}

impl BufferedStream {
    fn new_in(capacity: usize) -> Self {
        let ptr = unsafe {
            let layout = Layout::from_size_align_unchecked(capacity, std::mem::size_of::<u8>());
            alloc(layout) as *mut u8
        };

        Self {
            ptr,
            capacity,
            head: Cell::new(0),
            tail: Cell::new(0),
            max_size: super::constant::DEFAULT_BUFFER_MAX_SIZE,
        }
    }

    fn handle_grow(&mut self, len: usize) -> Option<()> {
        // 쓸 수 있는 최대 bytes 가 쓰려는 bytes 보다 크거나 같으면 쓸 수 있음
        if self.limit() < len {
            return None;
        }

        // 현재 쓸 수 있는 bytes 보다 쓰려는 bytes 가 많으면 capacity 늘려야함 그렇지 않다면 compaction만
        if (self.capacity() - self.len()) < len {
            self.grow(len);
        } else {
            self.compaction();
        }
        Some(())
    }

    fn grow(&mut self, len: usize) {
        let cap = self.required_capacity(len);

        // 새로운 버퍼 생성
        let ptr = unsafe {
            let layout = Layout::from_size_align_unchecked(cap, std::mem::size_of::<u8>());
            alloc(layout) as *mut u8
        };

        // 기존 버퍼 head 부터 tail - head 만큼 새로운 버퍼에 복사 후 기존 버퍼 메모리 반환
        unsafe {
            ptr::copy(self.ptr.add(self.head()), ptr, self.len());
            dealloc(
                self.ptr as *mut u8,
                Layout::from_size_align_unchecked(self.capacity(), std::mem::size_of::<u8>()),
            )
        }

        // ptr 수정
        self.set_ptr(ptr);
        // capacity 변경
        self.set_capacity(cap);
        // tail 변경
        self.set_tail(self.len());
        // head 변경
        self.set_head(0);
    }

    fn compaction(&mut self) {
        // head 부터 tail 까지 0번 인덱스에 복사
        unsafe {
            ptr::copy(self.ptr.add(self.head()), self.ptr, self.len());
        }
        // tail 변경
        self.set_tail(self.len());
        // head 변경
        self.set_head(0);
    }

    fn required_capacity(&self, len: usize) -> usize {
        let required_cap = self.capacity() + len;
        let mut cap = self.capacity();

        // 필요한 capacity 만큼 기존 capacity * 2 BUFFER_MAX_SIZE 까지
        while cap < required_cap {
            cap *= 2;

            if cap > self.max_size() {
                cap = self.max_size();
                break;
            }
        }
        cap
    }

    // setter
    fn set_head(&self, value: usize) {
        self.head.set(value);
    }

    fn set_tail(&self, value: usize) {
        self.tail.set(value);
    }

    fn set_ptr(&mut self, ptr: *mut u8) {
        self.ptr = ptr;
    }

    fn set_capacity(&mut self, value: usize) {
        self.capacity = value;
    }
}


impl Drop for BufferedStream {
    fn drop(&mut self) {
        unsafe {
            dealloc(
                self.ptr as *mut u8,
                Layout::from_size_align_unchecked(self.capacity, std::mem::size_of::<u8>()),
            )
        };
    }
}
