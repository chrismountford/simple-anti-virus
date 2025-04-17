use std::{fs, path::{Path, PathBuf}};

use sha2::{Digest, Sha256};

fn main() {
    let path = Path::new("./");

    // Mutating this in place will allow further use of this further in the scope of this function
    let mut files_to_scan = Vec::new();
    get_files(path, &mut files_to_scan);
}

fn get_files(path: &Path, collection: &mut Vec<PathBuf>) {
    let paths = fs::read_dir(path).unwrap();

    for path in paths {
        let p = path.unwrap().path();
        if p.is_dir() {
            get_files(p.as_path(), collection);
        }
        if p.is_file() {
            collection.push(p);
        }
    }
}

fn compute_hash(path: &Path) -> Vec<u8> {
    let mut hasher = Sha256::new();
    let file_contents = fs::read(path);

    match file_contents {
        Ok(contents) => hasher.update(contents),
        Err(err) => panic!("{err}")
    }

    let result = hasher.finalize();

    result[..].to_vec()
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;
    use tempdir::TempDir;

    use super::*;

    struct TestContext {
        root: TempDir,
    }

    impl TestContext {
        fn new(root_path: &str) -> Self {
            let root = TempDir::new(root_path).expect("");
            return TestContext { root };
        }

        fn path(&self) -> &Path {
            self.root.path()
        }

        fn create_file(&self, relative: &str) {
            let file_path = self.root.path().join(relative);
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent).unwrap()
            }
            fs::write(file_path, "contents").unwrap()
        }
    }

    #[test]
    fn test_get_files_from_folder() {
        let ctx = TestContext::new("test_dir");
        ctx.create_file("foo.txt");
        ctx.create_file("bar.txt");

        let mut paths = Vec::new();
        get_files(ctx.path(), &mut paths);

        assert_eq!(paths.len(), 2, "Mismatched folder sizes");
    }

    #[test]
    fn test_get_files_from_from_folder_recursively() {
        let ctx = TestContext::new("test_dir");
        let test_files = ["foo.txt", "bar.txt", "inner/baz.txt", "inner/faz.txt", "inner/inner/boo.txt"];
       
        for file in test_files {
            ctx.create_file(file);
        }
        
        let mut paths = Vec::new();
        get_files(ctx.path(), &mut paths);

        assert_eq!(paths.len(), 5, "Mismatched folder sizes");
    }

    #[test]
    fn test_generate_hash_from_file() {
        let ctx = TestContext::new("test_dir");
        ctx.create_file("foo.txt");

        let file_path = ctx.path().join("foo.txt");

        let hash = compute_hash(file_path.as_path());

        let expected = hex!("d1b2a59fbea7e20077af9f91b27e95e865061b270be03ff539ab3b73587882e8");
        assert_eq!(hash, expected, "Mismatched hash of file");
    }
}
