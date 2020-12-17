use {
    std::{
        fs::File,
        io,
    },
    byteorder::{
        LittleEndian,
        ReadBytesExt as _,
    },
    clipboard_win::{
        Unicode,
        set_clipboard,
    },
    derive_more::From,
    error_code::SystemError,
};

#[derive(Debug, From)]
enum Error {
    HomeNotFound,
    Io(io::Error),
    Version,
}

fn main_inner() -> Result<String, Error> {
    let mut stats_file = File::open(
        directories::UserDirs::new()
            .ok_or(Error::HomeNotFound)?
            .home_dir()
            .join("AppData")
            .join("LocalLow")
            .join("Innersloth")
            .join("Among Us")
            .join("playerStats2")
    )?;
    let version = stats_file.read_u8()?;
    if version != 3 { return Err(Error::Version) }
    let mut buf = String::default();
    loop {
        let stat = match stats_file.read_u32::<LittleEndian>() {
            Ok(stat) => stat,
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(Error::Io(e)),
        };
        buf += &format!("{}\n", stat);
    }
    Ok(buf)
}

fn main() -> Result<(), SystemError> {
    match main_inner() {
        Ok(buf) => set_clipboard(Unicode, buf),
        Err(e) => set_clipboard(Unicode, format!("error: {:?}", e)),
    }
}
