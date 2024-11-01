use super::dialoguer_theme;
use super::input_prompt;
use crate::{
    builder_set,
    io::new::data::{Skill, SkillBuilder},
};
use colored::Colorize;
use dialoguer::Confirm;

/// 从tui中获取用户的技能
pub(super) fn get_skills_from_tui() -> anyhow::Result<Vec<Skill>> {
    let mut result = vec![];
    let theme = &dialoguer_theme();
    loop {
        println!("{}", "请输入您的技能:".green());
        let mut builder = SkillBuilder::default();
        builder_set!(builder, name, "技能名称");
        let skill = builder.build().unwrap();
        result.push(skill);
        if !Confirm::with_theme(theme)
            .with_prompt("继续添加技能?")
            .interact()
            .unwrap()
        {
            return Ok(result);
        }
    }
}
