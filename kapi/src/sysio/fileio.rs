use alloc::string;

pub enum FileResult {
    FileHandle(FileHandle),
    FileNotFound,
}

pub type FileHandle = u64;
pub type FilePath = string::String;

pub trait FileIO {
    fn open_file(path: FilePath);
    fn read_file();
    fn write_file();
    fn close_file(file_handle: FileHandle);
}
