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
        "std_io_copy" => copy_file_to_unix_domain_socket(file, socket)?,
        "nix_sendfile" => nix_sendfile(file, socket)?,
        "read_write" => read_write(file, socket)?,
        "libc_sendfile" => libc_sendfile(file, socket)?,
        _ => panic!("unsupported method {}", method),
    }

    let elapsed = now.elapsed();
    println!("***Metrics: time elapsed: {}ns", elapsed.as_nanos());
    Ok(())
}

fn copy_file_to_unix_domain_socket(mut file: File, mut socket: UnixStream) -> Result<()> {
    let copied_len = io::copy(&mut file, &mut socket)?;
    debug_assert_eq!(copied_len, file.metadata().unwrap().len());
    Ok(())
}

fn nix_sendfile(file: File, socket: UnixStream) -> Result<()> {
    let size = file.metadata().unwrap().len() as usize;
    let copied_len = nix::sys::sendfile::sendfile64(socket, file, None, size)?;
    debug_assert_eq!(copied_len, size);
    Ok(())
}

fn read_write(file: File, socket: UnixStream) -> Result<()> {
    let mut buf: [u8; 4096] = [0; 4096];
    loop {
        let bytes_read = (&file).read(&mut buf[..])?;
        if bytes_read == 0 {
            break;
        }
        (&socket).write_all(&buf[..bytes_read])?
    }
    Ok(())
}

fn libc_sendfile(file: File, socket: UnixStream) -> Result<()> {
    let mut len_written = 0_u64;
    let len_file = file.metadata().unwrap().size();
    while len_written < len_file {
        let chunck_size = std::cmp::min(len_file - len_written, 0x7ffff000_u64) as usize;
        match unsafe {
            libc::sendfile64(
                socket.as_raw_fd(),
                file.as_raw_fd(),
                std::ptr::null_mut(),
                chunck_size,
            )
        } {
            -1 => return Err(std::io::Error::last_os_error()),
            len_sent => len_written += len_sent as u64,
        }
    }
    Ok(())
}
