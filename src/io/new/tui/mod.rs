use super::data::{LocalData, LocalDataBuilder};
use basic::get_basic_from_tui;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use education::get_edu_from_tui;
use language::get_languages_from_tui;
use reference::get_ref_from_tui;
use skill::get_skills_from_tui;

#[macro_use]
mod basic;
mod education;
mod language;
mod reference;
mod skill;

/// 从tui中获取用户的输入数
pub(super) fn get_data_from_tui() -> anyhow::Result<LocalData> {
    let basic = get_basic_from_tui()?;
    let languages = get_languages_from_tui()?;
    let skills = get_skills_from_tui()?;
    let references = get_ref_from_tui()?;
    let educations = get_edu_from_tui()?;
    let data = LocalDataBuilder::default()
        .basic(basic)
        .languages(languages)
        .skill(skills)
        .references(references)
        .education(educations)
        .build()
        .unwrap();
    Ok(data)
}

fn dialoguer_theme() -> ColorfulTheme {
    ColorfulTheme::default()
}

/// Set the filed of builder by the tui input
///
/// # Arguments
/// - **builder** builder entiry builder实体
/// - **filed** The filed which be set. 需要设置的字段
/// - **prompt** The prompt which remind user to input. 输出提示语
#[macro_export]
macro_rules! builder_set {
    ($builder:expr,$filed:ident,$prompt:expr) => {
        let value: String = input_prompt($prompt).unwrap();
        $builder.$filed(value);
    };
}

/// The text inputed by user;从input中获取用户输出的数据
///
/// # Arguments
/// - prompt The prompt which tell user to input what; 用户提示语
///
/// # Return
/// - The text which is input by user. 用户输入
pub(super) fn input_prompt(prompt: &str) -> Result<String, dialoguer::Error> {
    let theme = dialoguer_theme();
    Input::with_theme(&theme)
        .with_prompt(prompt)
        .interact_text()
}
/// The selected option by user; 获取用户在列表中的选项
/// # Arguments
/// - **prompt** The prompt which tell user to select; 用户提示语
fn select_prompt(prompt: &str, items: &[&str], length: usize) -> Result<usize, dialoguer::Error> {
    let theme = dialoguer_theme();
    Select::with_theme(&theme)
        .with_prompt(prompt)
        .items(items)
        .max_length(length)
        .interact()
}
