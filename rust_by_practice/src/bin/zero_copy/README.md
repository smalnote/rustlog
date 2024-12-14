# Rust io::copy with Linux Zero-Copy API

## Environment

-   CPU: E5-2680v4
-   Mem: 48GB
-   OS: AlmaLinux 9
-   Host: KVM
-   Scenario: Copy 1GB file from SSD to local Unix domain socket
-   File size: 1GB
-   Unix Domain Socket Server: nc -lU /tmp/zero_copy.sock >/dev/null

## Benchmark

| API                                 | Seconds | Diff  |
| ----------------------------------- | ------- | ----- |
| C read/send                         | 907ms   | 100%  |
| C mmap/send                         | 266ms   | 29.3% |
| C sendfile                          | 131ms   | 14.4% |
| C splice/pipe                       | 158ms   | 17.4% |
| Rust std::io::copy                  | 577ms   | 63.6% |
| Rust nix::sys::sendfile::sendfile64 | 575ms   | 63.4% |

> [!NOTE]
> API splice/pipe use a pipe to connect filefd and sockfd, according to `man 2 spclie`,
> the splice function requires one of file descriptor to be pipe, result in:
> splice(filefd, pipefd[1]) and splice(pipefd[1], sockfd).

## Rust Zero-Copy Source Code

[Source Code](https://github.com/rust-lang/rust/blob/8e37e151835d96d6a7415e93e6876561485a3354/library/std/src/sys/pal/unix/kernel_copy.rs)

## Copy Disk file to Unix Domain Socket

```rust
use std::{
    env,
    fs::File,
    io::{self, Result},
    os::unix::net::UnixStream,
    time::Instant,
};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <file_path> <socket_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let socket_path = &args[2];
    let now = Instant::now();
    copy_file_to_unix_domain_socket(file_path, socket_path)?;
    let elapsed = now.elapsed();
    println!("***Metrics: time elapsed: {}ns", elapsed.as_nanos());
    Ok(())
}

fn copy_file_to_unix_domain_socket<'a>(file_path: &'a str, socket_path: &'a str) -> Result<()> {
    let mut file = File::open(file_path)?;
    let mut socket = UnixStream::connect(socket_path)?;
    let copied_len = io::copy(&mut file, &mut socket)?;
    debug_assert_eq!(copied_len, file.metadata().unwrap().len());
    Ok(())
}
```

### 分析：从 Disk File 到 Socket 的拷贝并没有走零拷贝 API

从源代码看，对于 input 到 output 的拷贝，使用到零拷贝的情况。

| Input         | Output           | Zero-copy API   |
| ------------- | ---------------- | --------------- |
| File(len > 0) | File             | copy_file_range |
| Block Device  | File             | sendfile        |
| Pipe          | File/Pipe/Socket | splice          |
| Socket/Pipe   | Pipe             | splice          |

> [!NOTE]
> 由于在执行拷贝时，是按顺序尝试 copy_file_range, sendfile, splice，尝试过程可能会改变文件
> 描述符，因此对 sendfile, splice 设置了 safe_kernel_copy 检查，要求 output 是 Pipe/Socket,
> 或 input 是 File，但是 sendfile 要求 output 是 File，导到 File -> Pipe, File -> Socket
> 不能用到 sendfile。

| ｜ Zero-copy API | Input Constraints             | Output constraints |
| ---------------- | ----------------------------- | ------------------ |
| copy_file_range  | File(len > 0)                 | File               |
| sendfile         | File(len > 0) or Block Device | File               |
| splice           | Pipe                          | \*                 |
| splice           | Socket/Pipe                   | Pipe               |

```rust
// kernel_copy.rs

// 判断是否可以用 copy_file_range，输入是文件且长度大于0，输出也是文件
if input_meta.copy_file_range_candidate(FdHandle::Input)
    && output_meta.copy_file_range_candidate(FdHandle::Output)

fn copy_file_range_candidate(&self, f: FdHandle) -> bool {
    match self {
        // copy_file_range will fail on empty procfs files. `read` can determine whether EOF has been reached
        // without extra cost and skip the write, thus there is no benefit in attempting copy_file_range
        FdMeta::Metadata(meta) if f == FdHandle::Input && meta.is_file() && meta.len() > 0 => {
            true
        }
        FdMeta::Metadata(meta) if f == FdHandle::Output && meta.is_file() => true,
        _ => false,
    }
}

// 判断是否可以用 sendfile,
// potential_sendfile_source 需要 src 是文件或文件类型是 block_device
// Block Device: HDD, SDD, etc; e.g.: /dev/sda (stat /dev/sda Access mode start with b)
// safe_kernel_copy 需要 src 是 Socket/Pipe/FIFO，或者 dst 是 File
// 可以看到 potential_sendfile_source 和 safe_kernel_copy 对于 src 的要求是冲突的，
// 实际上不可能用到 sendfile
if input_meta.potential_sendfile_source() && safe_kernel_copy(&input_meta, &output_meta)
fn potential_sendfile_source(&self) -> bool {
    match self {
        // procfs erroneously shows 0 length on non-empty readable files.
        // and if a file is truly empty then a `read` syscall will determine that and skip the write syscall
        // thus there would be benefit from attempting sendfile
        FdMeta::Metadata(meta)
            if meta.file_type().is_file() && meta.len() > 0
                || meta.file_type().is_block_device() =>
        {
            true
        }
        _ => false,
    }
}
/// Returns true either if changes made to the source after a sendfile/splice call won't become
/// visible in the sink or the source has explicitly opted into such behavior (e.g. by splicing
/// a file into a pipe, the pipe being the source in this case).
///
/// This will prevent File -> Pipe and File -> Socket splicing/sendfile optimizations to uphold
/// the Read/Write API semantics of io::copy.
///
/// Note: This is not 100% airtight, the caller can use the RawFd conversion methods to turn a
/// regular file into a TcpSocket which will be treated as a socket here without checking.
fn safe_kernel_copy(source: &FdMeta, sink: &FdMeta) -> bool {
    match (source, sink) {
        // Data arriving from a socket is safe because the sender can't modify the socket buffer.
        // Data arriving from a pipe is safe(-ish) because either the sender *copied*
        // the bytes into the pipe OR explicitly performed an operation that enables zero-copy,
        // thus promising not to modify the data later.
        (FdMeta::Socket, _) => true,
        (FdMeta::Pipe, _) => true,
        (FdMeta::Metadata(meta), _)
            if meta.file_type().is_fifo() || meta.file_type().is_socket() =>
        {
            true
        }
        // Data going into non-pipes/non-sockets is safe because the "later changes may become visible" issue
        // only happens for pages sitting in send buffers or pipes.
        (_, FdMeta::Metadata(meta))
            if !meta.file_type().is_fifo() && !meta.file_type().is_socket() =>
        {
            true
        }
        _ => false,
    }
}

// 判断是否可以用 splice
// src 和 dst 有一个是 pipe
// safe_kernel_copy 需要 src 是 Socket/Pipe/FIFO，或者 dst 不是 FIFO 或 不是 Socket
if (input_meta.maybe_fifo() || output_meta.maybe_fifo())
    && safe_kernel_copy(&input_meta, &output_meta)
```
