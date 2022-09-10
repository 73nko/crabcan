use std::fmt;
use std::process::exit;

use crate::container::MINIMAL_KERNEL_VERSION;

// Allows to display a variant with the format {:?}
#[derive(Debug)]
// Contains all possible errors in our tool
pub enum Errcode {
    ArgumentInvalid(&'static str),
    NotSupported(u8),
    ContainerError(u8),
}

impl Errcode {
    pub fn get_retcode(&self) -> i32 {
        1 // Everything != 0 will be treated as an error
    }
}

#[allow(unreachable_patterns)]
// trait Display, allos Errcode enum to be displayed by
//   println!("{}", error);
// in this case, it calls the function "fmt", which we define the behavior below
impl fmt::Display for Errcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Errcode::NotSupported(errtype) => match errtype {
                0 => write!(
                    f,
                    "Minimal kernel version required: {}",
                    MINIMAL_KERNEL_VERSION
                ),
                1 => write!(f, "Only x86_64 architecture is supported"),
                _ => write!(f, "{:?} (unknown id)", self),
            },
            Errcode::ArgumentInvalid(element) => write!(f, "ArgumentInvalid: {}", element),
            _ => write!(f, "{:?}", self),
        }
    }
}

// Get the result from a function, and exit the process with the correct error code
pub fn exit_with_retcode(res: Result<(), Errcode>) {
    match res {
        Ok(_) => {
            log::debug!("Exit wiithout any error, returning 0");
            exit(0);
        }
        Err(e) => {
            let retcode = e.get_retcode();
            log::debug!("Errror on exit:\n\t{}\n\tReturnning {}", e, retcode);
            exit(retcode);
        }
    }
}
