use std::env::current_dir;
use std::fs::create_dir_all;
use std::path::PathBuf;

fn main() {
    let current_dir = current_dir();

    let mut target_path = match current_dir {
        Ok(path) => path,
        Err(e) => panic!("Error getting current directory: {}", e),
    };

    target_path.push("dir");

    match create_dir(&target_path) {
        Ok(path) => println!("Created {:?}", path),
        Err(e) => println!("Fail {:?}", e),
    }   
    
}

fn create_dir (target: &PathBuf) -> Result<String, std::fmt::Error> {
    match create_dir_all(target) {
        Ok(_) => Ok(format!("{}", target.to_string_lossy())),
        Err(e) => panic!("Error creating: {:?}", e),
    }
}

// fn read_file(path: &str) -> Result<String, io::Error> {
//     let mut file = File::open(path)?; // Аuto return on Error
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     Ok(contents)
// }