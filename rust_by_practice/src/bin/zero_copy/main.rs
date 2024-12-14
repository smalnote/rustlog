use std::{
    env,
    fs::File,
    io::{self, Result},
    os::unix::net::UnixStream,
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
