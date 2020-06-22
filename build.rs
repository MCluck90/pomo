use fs_extra::dir;
use std::env;
use std::path::Path;

fn main() {
    let project_path = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let src_path = Path::new(project_path.to_str().unwrap()).join("resources");
    let src_path = src_path.to_str().unwrap();
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let dest_path = dest_path.to_str().unwrap();

    println!("{:?}", src_path);
    println!("{:?}", dest_path);
    let mut options = dir::CopyOptions::new();
    options.overwrite = true;
    match dir::copy(src_path, dest_path, &options) {
        Ok(_) => {}
        Err(error) => panic!("Failed to copy resources. {:?}", error),
    }
}
