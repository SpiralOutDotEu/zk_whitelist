use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    path::Path,
    sync::{Arc, Mutex},
};

/// A trait defining a generic file system operations interface.
///
/// Implementations of this trait can be used to perform
/// file system operations like moving files and directories.
pub trait FileSystemOps {
    /// Moves a file or directory from the source path to the destination path.
    ///
    /// # Parameters
    /// - `src`: The path of the file or directory to move.
    /// - `dst`: The destination path to move the file or directory to.
    ///
    /// # Returns
    /// A `Result` indicating the success or failure of the operation.
    fn move_item(&self, src: &str, dst: &str) -> Result<(), String>;
    fn read_lines(&self, path: &str) -> Result<Vec<String>, String>;
    fn write_to_file(&self, path: &str, content: &str) -> Result<(), String>;
}

/// A real implementation of the `FileSystemOps` trait that performs file system operations.
pub struct RealFileSystemOps;

impl FileSystemOps for RealFileSystemOps {
    fn move_item(&self, src: &str, dst: &str) -> Result<(), String> {
        let src_path = Path::new(src);
        let dst_path = Path::new(dst);

        if !src_path.exists() || !src_path.is_dir() {
            return Err("Source directory does not exist or is not a directory".to_string());
        }

        for entry in fs::read_dir(src_path).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();

            if path.is_file() {
                let dest_path = dst_path.join(
                    path.file_name()
                        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "No file name"))
                        .map_err(|e| e.to_string())?,
                );
                fs::rename(&path, &dest_path).map_err(|e| e.to_string())?;
            }
        }

        Ok(())
    }

    fn read_lines(&self, path: &str) -> Result<Vec<String>, String> {
        let file = File::open(path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);
        let lines = reader
            .lines()
            .collect::<Result<Vec<String>, io::Error>>()
            .map_err(|e| e.to_string())?;
        Ok(lines)
    }

    fn write_to_file(&self, path: &str, content: &str) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .map_err(|e| e.to_string())?;
        file.write_all(content.as_bytes())
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

/// A mock implementation of the `FileSystemOps` trait for testing purposes.
///
/// This implementation records the file system operations it was called with,
/// allowing tests to verify correct behavior without performing real file system operations.
pub struct MockFileSystemOps {
    operations: Arc<Mutex<Vec<(String, String)>>>,
    written_content: Arc<Mutex<HashMap<String, String>>>,
    stubbed_file_content: Arc<Mutex<HashMap<String, Vec<String>>>>,
}

impl MockFileSystemOps {
    /// Creates a new `MockFileSystemOps` instance.
    pub fn new() -> Self {
        MockFileSystemOps {
            operations: Arc::new(Mutex::new(Vec::new())),
            written_content: Arc::new(Mutex::new(HashMap::new())),
            stubbed_file_content: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn stub_file_content(&self, path: &str, content: Vec<String>) {
        self.stubbed_file_content
            .lock()
            .unwrap()
            .insert(path.to_string(), content);
    }
    /// Returns a list of operations that this runner was called with.
    pub fn operations(&self) -> Vec<(String, String)> {
        self.operations.lock().unwrap().clone()
    }

    pub fn set_readable_content(&self, path: &str, content: Vec<String>) {
        self.stubbed_file_content
            .lock()
            .unwrap()
            .insert(path.to_string(), content);
    }

    pub fn get_written_content(&self, path: &str) -> Option<String> {
        self.written_content.lock().unwrap().get(path).cloned()
    }

    pub fn create_file(&self, path: &str) -> Result<(), String> {
        self.operations
            .lock()
            .unwrap()
            .push(("create_file".to_string(), path.to_string()));
        Ok(())
    }

    pub fn delete_file(&self, path: &str) -> Result<(), String> {
        self.operations
            .lock()
            .unwrap()
            .push(("delete_file".to_string(), path.to_string()));
        Ok(())
    }
}

impl FileSystemOps for MockFileSystemOps {
    fn move_item(&self, src: &str, dst: &str) -> Result<(), String> {
        self.operations
            .lock()
            .unwrap()
            .push((src.to_string(), dst.to_string()));
        Ok(())
    }

    fn read_lines(&self, path: &str) -> Result<Vec<String>, String> {
        match self.stubbed_file_content.lock().unwrap().get(path) {
            Some(content) => Ok(content.clone()),
            None => Err("File not found".to_string()),
        }
    }

    fn write_to_file(&self, path: &str, content: &str) -> Result<(), String> {
        self.written_content
            .lock()
            .unwrap()
            .insert(path.to_string(), content.to_string());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test to verify the move operation using `MockFileSystemOps`.
    fn test_move_item() {
        let mock_ops = MockFileSystemOps::new();
        let src = "source_path";
        let dst = "destination_path";

        let result = mock_ops.move_item(src, dst);

        assert!(result.is_ok());
        assert_eq!(
            mock_ops.operations(),
            vec![(src.to_string(), dst.to_string())]
        );
    }

    #[test]
    fn test_read_lines() {
        let mock_ops = MockFileSystemOps::new();
        let path = "test.txt";
        let content = vec!["line 1".to_string(), "line 2".to_string()];
        mock_ops.set_readable_content(path, content.clone());

        let result = mock_ops.read_lines(path);

        assert_eq!(result, Ok(content));
    }

    #[test]
    fn test_write_to_file() {
        let mock_ops = MockFileSystemOps::new();
        let path = "test.txt";
        let content = "some content";

        let result = mock_ops.write_to_file(path, content);

        assert!(result.is_ok());
        assert_eq!(
            mock_ops.get_written_content(path),
            Some(content.to_string())
        );
    }
}
