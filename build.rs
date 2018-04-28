
use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

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
