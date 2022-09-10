use std::fmt;
use std::process::exit;

// Allows to display a variant with the format {:?}
#[derive(Debug)]
// Contains all possible errors in our tool
pub enum Errcode {}

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
