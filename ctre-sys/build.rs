// use std::env;

const LIB_LIST: &[&str] = &[
    "FRC_NetworkCommunication",
    "NiFpga",
    "NiFpgaLv",
    "niriodevenum",
    "niriosession",
    "NiRioSrv",
    "RoboRIO_FRC_ChipObject",
    "visa",
//    "./lib/libCTRE_Phoenix.so",
];

fn main() {
    for lib in LIB_LIST {
        println!("cargo:rustc-link-lib=dylib={}", lib);
    }
    println!("cargo:rustc-link-lib=stdc++");
}
