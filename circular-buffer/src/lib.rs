//! # Circular Buffer - 环形缓冲区
//!
//! write/read 分别满/空时返回错误；overwrite 满时覆盖最旧元素。
//!
//! ## 考点
//! - Vec<Option<T>> 避免 unsafe
//! - start/end 索引取模实现环形

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

pub struct CircularBuffer<T> {
    data: Vec<Option<T>>,
    start: usize,
    end: usize,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut data = Vec::with_capacity(capacity);
        data.resize_with(capacity, || None);
        Self {
            data,
            start: 0,
            end: 0,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.is_full() {
            return Err(Error::FullBuffer);
        }
        self.data[self.end] = Some(element);
        self.advance_end();
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.is_empty() {
            return Err(Error::EmptyBuffer);
        }
        let v = self.data[self.start]
            .take()
            .expect("buffer slot should be Some when not empty");
        self.advance_start();
        Ok(v)
    }

    pub fn clear(&mut self) {
        self.data.fill_with(|| None);
        self.start = 0;
        self.end = 0;
    }

    pub fn overwrite(&mut self, element: T) {
        if self.is_full() {
            self.advance_start();
        }
        self.data[self.end] = Some(element);
        self.advance_end();
    }

    fn is_empty(&self) -> bool {
        self.start == self.end && self.data[self.start].is_none()
    }

    fn is_full(&self) -> bool {
        self.start == self.end && self.data[self.start].is_some()
    }

    fn advance_start(&mut self) {
        self.start = (self.start + 1) % self.data.len();
    }

    fn advance_end(&mut self) {
        self.end = (self.end + 1) % self.data.len();
    }
}
