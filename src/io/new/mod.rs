use std::{fs, path::Path};
use tui::get_data_from_tui;

mod data;
#[macro_use]
mod tui;

fn data_file() -> Option<std::path::PathBuf> {
    dirs::data_local_dir().map(|path| path.join("rsume/data.yaml"))
}

pub fn new() -> anyhow::Result<()> {
    let string = include_str!("source.yaml");
    let data_file = data_file();
    match data_file {
        // Some(path) if path.exists() => {
        //     let value = fs::read_to_string(path).unwrap();
        //     let resume: HashMap<String, String> = serde_yaml_ng::from_str(&value).unwrap();
        //     println!("resume is {:?}", resume);
        //     Ok(())
        // }
        Some(path) => {
            let data = create_data();
            create_data_file(&path, &data)?;
            fs::write("resume.yaml", string)?;
            Ok(())
        }
        None => {
            fs::write("resume.yaml", string)?;
            Ok(())
        }
    }
}

/// 创建基础数据
fn create_data() -> String {
    let data = get_data_from_tui().unwrap();
    serde_yaml_ng::to_string(&data).unwrap_or_default()
}

/// 创建基础文件
fn create_data_file(path: &Path, content: &str) -> anyhow::Result<()> {
    let dir = path.parent();
    if let Some(dir) = dir {
        if fs::create_dir_all(dir).is_ok() {
            fs::write(path, content)?
        }
    }
    Ok(())
}
