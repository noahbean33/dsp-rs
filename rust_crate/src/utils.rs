use std::path::Path;

/// Check whether a path exists and is a file.
pub fn is_valid_file(path: &Path) -> bool {
    path.exists() && path.is_file()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn non_existent_path_is_not_valid() {
        let path = PathBuf::from("/this/does/not/exist");
        assert!(!is_valid_file(&path));
    }
}
