use std::io::{Read, Result, Write};

// 泛型参数 R 代表任何实现了 Read trait 的类型（Reader）
// 可以是文件、网络连接、内存缓冲区等任何读取器
pub struct ReadStats<R> {
    wrapped: R,           // 被包装的读取器，类型为 R
    bytes_read: usize,   // 统计：总读取字节数
    read_count: usize,   // 统计：读取操作次数
}

// R: Read 是 trait bound，确保 R 必须实现 Read trait
// 这样编译器就知道 self.wrapped.read() 方法是可用的
impl<R: Read> ReadStats<R> {
    // 构造函数：接收任何实现了 Read 的类型作为参数
    // R 的具体类型由编译器根据传入参数自动推断
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            wrapped,        // 存储被包装的读取器
            bytes_read: 0,   // 初始化统计计数器
            read_count: 0,  // 初始化统计计数器
        }
    }

    // 返回被包装对象的引用，类型为 &R
    // 允许外部访问底层的读取器
    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    // 返回总读取字节数
    pub fn bytes_through(&self) -> usize {
        self.bytes_read
    }

    // 返回读取操作次数
    pub fn reads(&self) -> usize {
        self.read_count
    }
}

// 为 ReadStats<R> 实现 Read trait，使其本身也成为读取器
// 这是装饰器模式的核心：包装器实现相同的接口
impl<R: Read> Read for ReadStats<R> {
    // 代理读取操作到被包装的读取器，同时收集统计信息
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        // 调用被包装读取器的 read 方法
        // 由于 R: Read，编译器保证这个方法存在
        let bytes_read = self.wrapped.read(buf)?;
        
        // 更新统计信息：只有操作成功时才统计
        self.bytes_read += bytes_read;  // 累加读取字节数
        self.read_count += 1;          // 增加操作次数
        
        // 返回读取的字节数，保持透明性
        Ok(bytes_read)
    }
}

// 泛型参数 W 代表任何实现了 Write trait 的类型（Writer）
// 可以是文件、网络连接、内存缓冲区等任何写入器
pub struct WriteStats<W> {
    wrapped: W,             // 被包装的写入器，类型为 W
    bytes_written: usize,   // 统计：总写入字节数
    write_count: usize,     // 统计：写入操作次数
}

// W: Write 是 trait bound，确保 W 必须实现 Write trait
// 这样编译器就知道 self.wrapped.write() 和 flush() 方法是可用的
impl<W: Write> WriteStats<W> {
    // 构造函数：接收任何实现了 Write 的类型作为参数
    // W 的具体类型由编译器根据传入参数自动推断
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            wrapped,          // 存储被包装的写入器
            bytes_written: 0,  // 初始化统计计数器
            write_count: 0,    // 初始化统计计数器
        }
    }

    // 返回被包装对象的引用，类型为 &W
    // 允许外部访问底层的写入器
    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    // 返回总写入字节数
    pub fn bytes_through(&self) -> usize {
        self.bytes_written
    }

    // 返回写入操作次数
    pub fn writes(&self) -> usize {
        self.write_count
    }
}

// 为 WriteStats<W> 实现 Write trait，使其本身也成为写入器
// 这是装饰器模式的核心：包装器实现相同的接口
impl<W: Write> Write for WriteStats<W> {
    // 代理写入操作到被包装的写入器，同时收集统计信息
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        // 调用被包装写入器的 write 方法
        // 由于 W: Write，编译器保证这个方法存在
        let bytes_written = self.wrapped.write(buf)?;
        
        // 更新统计信息：只有操作成功时才统计
        self.bytes_written += bytes_written;  // 累加写入字节数
        self.write_count += 1;               // 增加操作次数
        
        // 返回写入的字节数，保持透明性
        Ok(bytes_written)
    }

    // 代理刷新操作到被包装的写入器
    // 刷新操作不涉及数据写入，所以不更新统计信息
    fn flush(&mut self) -> Result<()> {
        // 直接调用被包装写入器的 flush 方法
        // 保持透明性，不添加额外逻辑
        self.wrapped.flush()
    }
}
