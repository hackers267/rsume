use std::path::Path;

/// 获取文件名的文件部分
pub(crate) fn get_filename(str: &str) -> Option<std::path::PathBuf> {
    let path = Path::new(str);
    let path = path.with_extension("yaml");
    let filename = path.file_name()?;
    let path = Path::new(filename).to_path_buf();
    Some(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{path::PathBuf, str::FromStr};

    #[test]
    fn only_filename() {
        let str = "data";
        let filename = get_filename(str);
        assert_eq!(filename, PathBuf::from_str("data.yaml").ok())
    }

    #[test]
    fn with_extension() {
        let str = "data.html";
        let filename = get_filename(str);
        assert_eq!(filename, PathBuf::from_str("data.yaml").ok());
    }

    #[test]
    fn with_dir() {
        let str = "home/data";
        let filename = get_filename(str);
        assert_eq!(filename, PathBuf::from_str("data.yaml").ok());
    }

    #[test]
    fn with_dir_and_extension() {
        let str = "home/data.txt";
        let filename = get_filename(str);
        assert_eq!(filename, PathBuf::from_str("data.yaml").ok());
    }
}
