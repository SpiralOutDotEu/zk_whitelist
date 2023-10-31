/// movejs.rs
use crate::utils::filesystem_operations::FileSystemOps;
use std::io;

/// Handles the `movejs` subcommand by moving the contents
/// of the `circuit_js` directory to the current directory.
///
/// This function accepts a generic parameter `F` which implements
/// the `FileSystemOps` trait, allowing for the move operation to be handled by `fs_ops`.
///
/// # Parameters
/// - `fs_ops`: The file system operations runner which will perform the move operation.
///
/// # Returns
/// - An `io::Result<()>` which will be `Ok(())` if the move operation completes successfully,
///   or an `Err` wrapping an `io::Error` if an error occurs.
pub fn handle_movejs_subcommand<F: FileSystemOps>(fs_ops: &F) -> io::Result<()> {
    // Call the move_item method with fs_ops to move the contents of circuit_js to the current directory.
    fs_ops
        .move_item("circuit_js", ".")
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

#[cfg(test)]
mod tests {
    use crate::utils::filesystem_operations::MockFileSystemOps;

    use super::*;

    /// Tests the `handle_movejs_subcommand` function to ensure it can perform the move operation.
    ///
    /// This test creates a `MockFileSystemOps` instance to simulate the behavior of the `FileSystemOps` trait.
    /// It then calls `handle_movejs_subcommand` with this mock fs_ops, checks the returned result to ensure
    /// it is `Ok`, and finally, it checks the operations performed by `MockFileSystemOps` to ensure they match the expected operations.
    #[test]
    fn test_handle_movejs_subcommand() {
        // Create a new MockFileSystemOps instance for testing.
        let fs_ops = MockFileSystemOps::new();

        // Call the function with the mock fs_ops.
        let result = handle_movejs_subcommand(&fs_ops); // Now passing a reference

        // Assert the result is Ok(()) indicating success.
        assert!(result.is_ok());

        // Assert that the `move_item` method of `MockFileSystemOps` has been called with correct arguments
        assert_eq!(
            fs_ops.operations(),
            vec![("circuit_js".to_string(), ".".to_string())]
        );
    }
}
