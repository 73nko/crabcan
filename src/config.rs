use std::ffi::CString;
use std::path::PathBuf;

use crate::errors::Errcode;

#[derive(Clone)]
pub struct ContainerOpts {
    pub path: CString,
    pub args: Vec<CString>,

    pub uid: u32,
    pub mount_dir: PathBuf,
}

impl ContainerOpts {
    pub fn new(command: String, uid: u32, mount_dir: PathBuf) -> Result<ContainerOpts, Errcode> {
        let args: Vec<CString> = command
            .split_ascii_whitespace()
            .map(|s| CString::new(s).expect("Cannot read arg"))
            .collect();

        let path = args[0].clone();

        Ok(ContainerOpts {
            path,
            args,
            uid,
            mount_dir,
        })
    }
}
