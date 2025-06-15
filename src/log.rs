// cargo add chrono

use std::fs;
use std::fs::File;
use std::io::Write;

pub fn err(error_folder: &str, msg: &str) {
    eprintln!("ERROR: {msg}");

    if let Err(err) = fs::create_dir_all(error_folder) {
        eprintln!("ERROR: could not create error folder: {}", err);
        return;
    }

    let now = chrono::offset::Local::now();
    let file_name = now.format("%Y-%m-%d_%H-%M-%S-%f"); // %f - nanoseconds

    let mut f = match File::options()
        .append(true)
        .create(true)
        .open(format!("{}/{}", error_folder, file_name))
    {
        Ok(f) => f,
        Err(err) => {
            eprintln!("ERROR: could not create error file: {}", err);
            return;
        }
    };

    if let Err(err) = writeln!(&mut f, "{}", msg) {
        eprintln!("ERROR: could not write to error file: {}", err);
    }
}
