use super::data::{Basic, BasicBuilder, LocalData, LocalDataBuilder, Location, LocationBuilder};
use dialoguer::{theme::ColorfulTheme, Input};

pub(super) fn get_data_from_tui() -> anyhow::Result<LocalData> {
    let basic = get_basic_from_tui()?;
    let data = LocalDataBuilder::default().basic(basic).build().unwrap();
    Ok(data)
}
/// Set the filed of builder by the tui input
///
/// # Arguments
/// - **builder** builder entiry builder实体
/// - **filed** The filed which be set. 需要设置的字段
/// - **prompt** The prompt which remind user to input. 输出提示语
macro_rules! builder_set {
    ($builder:expr,$filed:ident,$prompt:expr) => {
        let value: String = input_prompt($prompt).unwrap();
        $builder.$filed(value);
    };
}
/// 通过tui获取基础信息数据
fn get_basic_from_tui() -> anyhow::Result<Basic> {
    let mut basic_builder = BasicBuilder::default();
    builder_set!(basic_builder, name, "姓名");
    builder_set!(basic_builder, label, "职位");
    builder_set!(basic_builder, email, "电子邮箱");
    builder_set!(basic_builder, phone, "联系电话");
    builder_set!(basic_builder, summary, "自我评价");
    println!("请输入您的个人住址信息:");
    let location = get_location_from_tui()?;
    basic_builder.location(location);
    let basic = basic_builder.build().unwrap();

    Ok(basic)
}

/// 通过tui获取个人住址
fn get_location_from_tui() -> anyhow::Result<Location> {
    let mut location_builder = LocationBuilder::default();
    builder_set!(location_builder, address, "地址");
    builder_set!(location_builder, postal_code, "邮政编码");
    builder_set!(location_builder, city, "城市");
    builder_set!(location_builder, coutry_code, "图家编码");
    let location = location_builder.build()?;
    Ok(location)
}

/// The text inputed by user;从input中获取用户输出的数据
///
/// # Arguments
/// - prompt The prompt which tell user to input what; 用户提示语
///
/// # Return
/// - The text which is input by user. 用户输入
fn input_prompt(prompt: &str) -> Result<String, dialoguer::Error> {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact_text()
}
