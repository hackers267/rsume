use json_resume::Resume;
use std::{fs, path::Path};
use tui::get_data_from_tui;

mod data;
#[macro_use]
mod tui;

fn data_file() -> Option<std::path::PathBuf> {
    dirs::data_local_dir().map(|path| path.join("rsume/data.yaml"))
}

pub fn new() -> anyhow::Result<()> {
    let source = include_str!("source.yaml");
    let data_file = data_file();
    match data_file {
        Some(path) if path.exists() => {
            let value = fs::read_to_string(path).unwrap();
            let mut input_resume: Resume = serde_yaml_ng::from_str(&value).unwrap();
            let source_resume: Resume = serde_yaml_ng::from_str(source).unwrap();
            input_resume.projects = source_resume.projects;
            let output = serde_yaml_ng::to_string(&input_resume).unwrap();
            fs::write("resume.yaml", output)?;
            Ok(())
        }
        Some(path) => {
            let data = create_data();
            create_data_file(&path, &data)?;
            let mut input_resume: Resume = serde_yaml_ng::from_str(&data).unwrap();
            let source_resume: Resume = serde_yaml_ng::from_str(source).unwrap();
            input_resume.projects = source_resume.projects;
            let output = serde_yaml_ng::to_string(&input_resume).unwrap();
            fs::write("resume.yaml", output)?;
            Ok(())
        }
        None => {
            fs::write("resume.yaml", source)?;
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
fn create_data_file<'a>(path: &Path, content: &'a str) -> anyhow::Result<&'a str> {
    let dir = path.parent();
    if let Some(dir) = dir {
        if fs::create_dir_all(dir).is_ok() {
            fs::write(path, content)?;
        }
    }
    Ok(content)
}
