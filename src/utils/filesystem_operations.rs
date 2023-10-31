use std::{
    fs, io,
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
}

/// A mock implementation of the `FileSystemOps` trait for testing purposes.
///
/// This implementation records the file system operations it was called with,
/// allowing tests to verify correct behavior without performing real file system operations.
pub struct MockFileSystemOps {
    operations: Arc<Mutex<Vec<(String, String)>>>,
}

impl MockFileSystemOps {
    /// Creates a new `MockFileSystemOps` instance.
    pub fn new() -> Self {
        MockFileSystemOps {
            operations: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Returns a list of operations that this runner was called with.
    pub fn operations(&self) -> Vec<(String, String)> {
        self.operations.lock().unwrap().clone()
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
}
