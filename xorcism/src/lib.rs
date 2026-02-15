//! # Xorcism - XOR 流适配器
//!
//! 用密钥字节循环 XOR 数据。munge_in_place 原地修改，munge 返回迭代器。状态ful：密钥位置在调用间保持。
//!
//! ## 考点
//! - 生命周期、AsRef、Borrow
//! - 迭代器、泛型

use std::borrow::Borrow;

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    key: &'a [u8],
    pos: usize,
}

fn next_key_byte(key: &[u8], pos: &mut usize) -> u8 {
    let b = key[*pos];
    *pos += 1;
    if *pos >= key.len() {
        *pos = 0;
    }
    b
}

impl<'a> Xorcism<'a> {
    pub fn new<Key>(key: &'a Key) -> Xorcism<'a>
    where
        Key: AsRef<[u8]> + ?Sized,
    {
        Xorcism {
            key: key.as_ref(),
            pos: 0,
        }
    }

    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        for d in data.iter_mut() {
            *d ^= next_key_byte(self.key, &mut self.pos);
        }
    }

    pub fn munge<'b, Data, B>(&'b mut self, data: Data) -> impl 'b + Iterator<Item = u8>
    where
        Data: IntoIterator<Item = B>,
        <Data as IntoIterator>::IntoIter: 'b,
        B: Borrow<u8>,
    {
        let key = self.key;
        let pos = &mut self.pos;
        data.into_iter()
            .map(move |d| *d.borrow() ^ next_key_byte(key, pos))
    }
}

#[cfg(feature = "io")]
use std::io::{Read, Write};

#[cfg(feature = "io")]
impl<'a> Xorcism<'a> {
    pub fn writer<W: Write>(self, writer: W) -> Writer<'a, W> {
        Writer {
            xorcism: self,
            writer,
        }
    }

    pub fn reader<R: Read>(self, reader: R) -> Reader<'a, R> {
        Reader {
            xorcism: self,
            reader,
        }
    }
}

#[cfg(feature = "io")]
pub struct Writer<'a, W> {
    xorcism: Xorcism<'a>,
    writer: W,
}

#[cfg(feature = "io")]
impl<W: Write> Write for Writer<'_, W> {
    fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
        let munged: Vec<_> = self.xorcism.munge(data).collect();
        self.writer.write_all(&munged)?;
        Ok(data.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

#[cfg(feature = "io")]
pub struct Reader<'a, R> {
    xorcism: Xorcism<'a>,
    reader: R,
}

#[cfg(feature = "io")]
impl<R: Read> Read for Reader<'_, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.reader.read(buf)?;
        self.xorcism.munge_in_place(&mut buf[..n]);
        Ok(n)
    }
}
