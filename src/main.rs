use std::env;
use std::os::raw::c_char;
use std::ffi::CString;

extern {
    fn D_DoomMain();
    fn M_FindResponseFile();
    static mut myargv: *const *const c_char;
    static mut myargc: u32;
}

fn main() {
   // create a vector of zero terminated strings
   let args = std::env::args()
       .map(|arg| CString::new(arg).unwrap() )
       .collect::<Vec<CString>>();
   // convert the strings to raw pointers
   let c_args = args
       .iter()
       .map(|arg| arg.as_ptr())
       .collect::<Vec<*const c_char>>();
   unsafe {
       myargc = c_args.len() as u32;
       myargv = c_args.as_ptr();
       M_FindResponseFile();
       D_DoomMain();
   }
}
