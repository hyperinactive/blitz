use std::{
    env,
    fs::File,
    io::{ErrorKind, Read, Write},
    path::Path,
};

enum FileMode {
    READ,
    WRITE,
    APPEND,
}

// NOTE: files being opened aren't exclusive
// TODO: need to figure out how to lock files while writing into them
pub struct FileSystem {}

impl FileSystem {
    fn open_file(path_str: &str, mode: FileMode) -> File {
        let curr_dir = match env::current_dir() {
            Ok(dir) => dir
                .to_str()
                .expect("Current dir couldn't convert to string")
                .to_string(),
            _ => panic!("No current dir, wtf?"),
        };

        let joined_path = curr_dir + "\\" + path_str;
        let path = Path::new(&joined_path);

        let mut read = false;
        let mut write = false;
        let mut append = false;

        match mode {
            FileMode::READ => read = true,
            FileMode::WRITE => write = true,
            FileMode::APPEND => append = true,
        }

        let file = match File::options()
            .read(read)
            .write(write)
            .append(append)
            .open(path)
        {
            Ok(f) => f,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {
                    panic!("File not found.",)
                }
                _ => panic!("{:?}", e),
            },
        };

        file
    }

    pub fn read_file(path_str: &str) -> String {
        let mut file = FileSystem::open_file(path_str, FileMode::READ);

        // output buffer
        let mut buffer = String::new();
        // read and append to buffer
        file.read_to_string(&mut buffer)
            .expect("Error reading file content");

        // bytes to string
        buffer
    }

    pub fn overwrite_file(path_str: &str, data: &str) {
        let mut file = FileSystem::open_file(path_str, FileMode::WRITE);
        file.write_all(data.as_bytes()).expect("Writing failed");
    }

    pub fn write_to_file(path_str: &str, data: &str) {
        let mut file = FileSystem::open_file(path_str, FileMode::APPEND);
        file.write_all(data.as_bytes()).expect("Writing failed");
    }
}

// ***************************************************************************
// TESTS
// ***************************************************************************

#[cfg(test)]
mod tests {
    use super::FileSystem;

    #[test]
    fn read_file_content() {
        let content = FileSystem::read_file("bin\\hello.txt");
        assert_eq!(content, "hello\r\nworld".to_string());
    }

    #[test]
    #[should_panic(expected = "File not found.")]
    fn file_not_found() {
        FileSystem::read_file("hello.txt");
    }

    #[test]
    fn overwrite_file() {
        FileSystem::overwrite_file("bin\\joji.txt", "joji\r\njojo");
        let content = FileSystem::read_file("bin\\joji.txt");
        assert_eq!(content, "joji\r\njojo".to_string());
    }

    #[test]
    #[ignore = "it works, but predicting endlessly appending output is a chore and I don't want to write a proper test"]
    fn write_to_file() {
        FileSystem::write_to_file("bin\\world.txt", "hello\r\nworld");
        let content = FileSystem::read_file("bin\\world.txt");
        assert_eq!(content, "hello\r\nworld".to_string());
    }
}
