use std::fs;
use std::io;
use std::path::Path;

pub fn ensure_storage_directory(path: &Path) -> io::Result<()> {
    fs::create_dir_all(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_storage_directory() {
        let temp = tempfile::tempdir().unwrap();
        let stash_path = temp.path().join("Stash");

        assert!(!stash_path.exists());

        ensure_storage_directory(&stash_path).unwrap();

        assert!(stash_path.is_dir());
    }

    #[test]
    fn preserves_existing_directory_contents() {
        let temp = tempfile::tempdir().unwrap();
        let stash_path = temp.path().join("Stash");

        ensure_storage_directory(&stash_path).unwrap();

        let existing_file = stash_path.join("existing.txt");
        fs::write(&existing_file, "Keep this file").unwrap();

        ensure_storage_directory(&stash_path).unwrap();

        assert_eq!(
            fs::read_to_string(&existing_file).unwrap(),
            "Keep this file"
        );
    }
}