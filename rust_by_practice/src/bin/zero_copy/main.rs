use std::{
    env,
    fs::File,
    io::{self, Read, Result, Write},
    os::{
        fd::AsRawFd,
        unix::{fs::MetadataExt, net::UnixStream},
    },
    time::Instant,
};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <method> <file_path> <socket_path>", args[0]);
        std::process::exit(1);
    }

    let method = &args[1];
    let file_path = &args[2];
    let socket_path = &args[3];
    let now = Instant::now();

    let file = File::open(file_path)?;
    let socket = UnixStream::connect(socket_path)?;
    match method.as_str() {
        "read_write" => read_write(file, socket)?,
        "std_io_copy" => copy_file_to_unix_domain_socket(file, socket)?,
        "libc_sendfile" => libc_sendfile(file, socket)?,
        "nix_sendfile" => nix_sendfile(file, socket)?,
        _ => panic!("unsupported method {}", method),
    }

    let elapsed = now.elapsed();
    println!("***Metrics: time elapsed: {}ms", elapsed.as_millis());
    Ok(())
}

fn read_write(file: File, socket: UnixStream) -> Result<()> {
    let mut buf: [u8; 8192] = [0; 8192];
    let mut bytes_left = file.metadata().unwrap().size() as usize;
    loop {
        let bytes_read = (&file).read(&mut buf[..])?;
        if bytes_read == 0 {
            break;
        }
        (&socket).write_all(&buf[..bytes_read])?;
        bytes_left -= bytes_read;
    }
    assert_eq!(bytes_left, 0);
    Ok(())
}

fn copy_file_to_unix_domain_socket(mut file: File, mut socket: UnixStream) -> Result<()> {
    let copied_len = io::copy(&mut file, &mut socket)?;
    assert_eq!(copied_len, file.metadata().unwrap().len());
    Ok(())
}

fn nix_sendfile(file: File, socket: UnixStream) -> Result<()> {
    let mut len_left = file.metadata().unwrap().len() as usize;
    while len_left > 0 {
        let chunk_size = std::cmp::min(len_left, 0x7ffff000);
        let copied_len = nix::sys::sendfile::sendfile64(&socket, &file, None, chunk_size)?;
        len_left -= copied_len;
    }
    assert_eq!(len_left, 0);
    Ok(())
}

fn libc_sendfile(file: File, socket: UnixStream) -> Result<()> {
    let mut len_written = 0_u64;
    let len_file = file.metadata().unwrap().size();
    while len_written < len_file {
        let chunk_size = std::cmp::min(len_file - len_written, 0x7ffff000_u64) as usize;
        match unsafe {
            libc::sendfile64(
                socket.as_raw_fd(),
                file.as_raw_fd(),
                std::ptr::null_mut(),
                chunk_size,
            )
        } {
            -1 => return Err(std::io::Error::last_os_error()),
            len_sent => len_written += len_sent as u64,
        }
    }
    assert_eq!(len_written, len_file);
    Ok(())
}
