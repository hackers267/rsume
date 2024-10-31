use super::data::{Basic, BasicBuilder, LocalData, LocalDataBuilder};
use dialoguer::{theme::ColorfulTheme, Input};

pub(super) fn get_data_from_tui() -> anyhow::Result<LocalData> {
    let basic = get_basic_from_tui()?;
    let data = LocalDataBuilder::default().basic(basic).build().unwrap();
    Ok(data)
}
fn get_basic_from_tui() -> anyhow::Result<Basic> {
    let mut basic_builder = BasicBuilder::default();
    let name: String = input_prompt("姓名").unwrap();
    basic_builder.name(name);
    let label: String = input_prompt("职位").unwrap();
    basic_builder.label(label);
    let email = input_prompt("电子邮箱").unwrap();
    basic_builder.email(email);
    let phone = input_prompt("联系电话").unwrap();
    basic_builder.phone(phone);
    let basic = basic_builder.build().unwrap();

    Ok(basic)
}

fn input_prompt(prompt: &str) -> Result<String, dialoguer::Error> {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact_text()
}
