use std::path::Path;

/// 获取文件名的文件部分
pub(crate) fn get_filename(str: &str) -> Option<&Path> {
    let path = Path::new(str);
    path.file_name()
        .map(Path::new)
        .and_then(|path| path.file_stem())
        .map(Path::new)
}
