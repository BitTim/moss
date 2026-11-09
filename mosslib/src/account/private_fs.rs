use crate::Result;
#[cfg(unix)]
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::{
    fs::{DirBuilder, OpenOptions},
    path::Path,
};

/// Make sure 'path' exists as a file readably only by its owner,
/// creating missing parent directories.
pub(super) fn ensure_private_db_file(path: &Path) -> Result<()> {
    // Ensure parent folder exists
    if let Some(parent) = path.parent() {
        DirBuilder::new().recursive(true).create(parent)?;
    }

    let mut options = OpenOptions::new();
    options.write(true).truncate(false).create(true);

    // Set permission mode to exclude group and everyone from any access
    #[cfg(unix)]
    options.mode(0o600);

    let file = options.open(path)?;

    // The above permission mode only applies to newly created files,
    // so here an existing one is checked and tightened if needed
    #[cfg(unix)]
    if file.metadata()?.permissions().mode() & 0o077 != 0 {
        use std::{fs::Permissions, os::unix::fs::PermissionsExt};

        file.set_permissions(Permissions::from_mode(0o600))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn creates_file_and_missing_parents() {
        let tmp = tempdir().unwrap();
        let path = tmp.path().join("test/dir/accounts.json");

        ensure_private_db_file(&path).unwrap();
        assert!(path.is_file());
    }

    #[test]
    fn is_idempotent_and_keeps_contents() {
        let tmp = tempdir().unwrap();
        let path = tmp.path().join("accounts.json");

        ensure_private_db_file(&path).unwrap();
        fs::write(&path, b"account").unwrap();
        ensure_private_db_file(&path).unwrap();

        // guards against an accidental truncate(true)
        assert_eq!(fs::read(&path).unwrap(), b"account");
    }

    #[test]
    fn fails_when_parent_is_a_file() {
        let tmp = tempdir().unwrap();
        let blocker = tmp.path().join("blocker");
        fs::write(&blocker, b"").unwrap();

        // the error must surface instead of being swallowed
        assert!(ensure_private_db_file(&blocker.join("accounts.json")).is_err());
    }

    #[cfg(unix)]
    mod unix {
        use super::*;

        fn mode_of(path: &Path) -> u32 {
            fs::metadata(path).unwrap().permissions().mode() & 0o777
        }

        #[test]
        fn new_file_is_0600() {
            let tmp = tempdir().unwrap();
            let path = tmp.path().join("accounts.json");

            ensure_private_db_file(&path).unwrap();
            assert_eq!(mode_of(&path), 0o600);
        }

        #[test]
        fn loose_existing_file_is_tightened() {
            let tmp = tempdir().unwrap();
            let path = tmp.path().join("accounts.json");
            fs::write(&path, b"account").unwrap();
            fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();

            ensure_private_db_file(&path).unwrap();

            assert_eq!(mode_of(&path), 0o600);
            assert_eq!(fs::read(&path).unwrap(), b"account");
        }

        #[test]
        fn already_private_file_is_unchanged() {
            let tmp = tempdir().unwrap();
            let path = tmp.path().join("accounts.json");
            ensure_private_db_file(&path).unwrap();

            ensure_private_db_file(&path).unwrap();
            assert_eq!(mode_of(&path), 0o600);
        }
    }
}
