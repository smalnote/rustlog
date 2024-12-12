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
