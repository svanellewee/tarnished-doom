extern crate cc;

use std::process::Command;
use std::env;
use std::path::{Path, PathBuf};
use std::{fs, io};
use std::ffi::OsStr;
use std::fs::DirEntry;


fn build_dir<P>(path: P) -> Result<Vec<PathBuf>, io::Error> 
where P: AsRef<Path> {
    fs::read_dir(path)?
        .into_iter()
        //.map(|x| x.map(|entry| entry.path()))
        .filter(|y| {
            println!(".....-----------------------{:?}", y);
           // -Ok(DirEntry("src/main/deh_mapping.c")) 
            match y {
                &Ok(ref v) => {
                    let result = v.path().extension().unwrap() == "c";
                    println!("result == {:?} {}", v.path(), result);
                    result
                },
                _ => false
            }
        })
        .map(|x| x.map(|entry| entry.path()))
        .collect()
}



fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let path = env::current_dir().unwrap();
println!("The current directory is............................................ {}", path.display());
    cc::Build::new()
        .files(build_dir("src/main/").unwrap())
        .include("src/main")
        .include("src/textscreen")
        .include("src/doom")
        .include("src/opl")
        .include("src/pcsound")
        .include("src/midiproc")
        .include("/usr/include/SDL2")
        .pic(true)
        .flag_if_supported("-Wall")
        .flag_if_supported("-Wdeclaration-after-statement")
        .flag_if_supported("-Wredundant-decls")
        .define("_REENTRANT", None)
        .compile("YO");
    // note that there are a number of downsides to this approach, the comments
    // below detail how to improve the portability of these commands.
    //*Command::new("gcc").args(&["src/hello.c", "-c", "-fPIC", "-o"])
    //                   .arg(&format!("{}/hello.o", out_dir))
    //                   .status().unwrap();*/
    //*Command::new("gcc").args(&["src/opl/*.c", "-c", "-fPIC", "-I/usr/include/SDL2"]).status().unwrap();
    //Command::new("ar").args(&["crus", "libopl.a", "src/opl/*.o"])
     //                 .current_dir(&Path::new(&out_dir))
      //                .status().unwrap();*/
      //
    let packages = ["doom", "midiproc", "opl", "pcsound", "textscreen", "main"]; 
    for package in packages.iter() {
        println!("cargo:rustc-link-search=native=src/{}", package);
        println!("cargo:rustc-link-lib=static={}", package);
     }
    println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu/");
    println!("cargo:include=/usr/include/SDL2");
    println!("cargo:libdir=/usr/lib/x86_64-linux-gnu/");
    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rustc-link-lib=SDL2_net");
    println!("cargo:rustc-link-lib=SDL2_mixer");
}
