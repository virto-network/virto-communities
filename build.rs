use std::{fs, path::PathBuf};

const STYLES_IN: &str = "public/styles";
const STYLES_OUT: &str = "public/css-out";

fn main() {
    println!("cargo:rerun-if-changed=src/styles");

    let files = fs::read_dir(STYLES_IN).expect("It should read a dir");

    _ = fs::create_dir(STYLES_OUT);

    for file in files {
        let file = file.expect("It should read a file");
        let path = file.path();
        let file_name = file.file_name();
        let raw_name = file_name
            .to_str()
            .expect("It should parse a file name")
            .strip_suffix(".scss")
            .expect("It should remove a suffix .scss");

        let out_path = PathBuf::from(STYLES_OUT).join(format!("{}.css", raw_name));

        let scss_options = grass::Options::default().style(grass::OutputStyle::Compressed);
        let css =
            grass::from_path(path, &scss_options).expect("It should convert a .scss block to css");
        fs::write(out_path, css).expect("It should write the output of a css");
    }
}
