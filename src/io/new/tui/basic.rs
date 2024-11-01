use super::{dialoguer_theme, input_prompt};
use crate::{
    builder_set,
    io::new::data::{Basic, BasicBuilder, Location, LocationBuilder, Profile, ProfileBuilder},
};
use colored::Colorize;
use dialoguer::Confirm;

/// 通过tui获取基础信息数据
pub(super) fn get_basic_from_tui() -> anyhow::Result<Basic> {
    let mut builder = BasicBuilder::default();
    builder_set!(builder, name, "姓名");
    builder_set!(builder, label, "职位");
    builder_set!(builder, email, "电子邮箱");
    builder_set!(builder, phone, "联系电话");
    builder_set!(builder, summary, "自我评价");
    println!("{}", "请输入您的个人住址信息:".green());
    let location = get_location_from_tui()?;
    builder.location(location);
    let profiles = get_profile_from_tui()?;
    builder.profiles(profiles);
    let basic = builder.build().unwrap();

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

/// 通过tui获取个人社交网站
fn get_profile_from_tui() -> anyhow::Result<Vec<Profile>> {
    let mut profiles = vec![];
    let theme = dialoguer_theme();
    loop {
        println!("{}", "请输入您的个人社交信息:".green());
        let mut builder = ProfileBuilder::default();
        builder_set!(builder, network, "网站名称");
        builder_set!(builder, url, "网址");
        builder_set!(builder, username, "用户名称");
        let profile = builder.build().unwrap();
        profiles.push(profile);
        if !Confirm::with_theme(&theme)
            .with_prompt("继续添加社交网站?")
            .interact()
            .unwrap()
        {
            return Ok(profiles);
        }
    }
}
