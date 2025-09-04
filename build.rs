use std::{env, io};
#[cfg(windows)]
use winresource::WindowsResource;

fn main() -> io::Result<()> {
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        #[cfg(windows)]
        {
            WindowsResource::new().set_icon("icon.ico").compile()?;
        }
    }
    Ok(())
}
