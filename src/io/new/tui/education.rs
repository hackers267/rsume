use super::{dialoguer_theme, input_prompt};
use crate::{
    builder_set,
    io::new::data::{Education, EducationBuilder},
};
use colored::Colorize;
use dialoguer::Confirm;

/// 从tui中获取用户的教育经历
pub(super) fn get_edu_from_tui() -> anyhow::Result<Vec<Education>> {
    let mut result = vec![];
    let theme = dialoguer_theme();
    loop {
        println!("{}", "请输入你的教育经历:".green());
        let mut builder = EducationBuilder::default();
        builder_set!(builder, institution, "学校名称");
        builder_set!(builder, area, "领域或专业");
        builder_set!(builder, start_date, "开始日期");
        builder_set!(builder, end_date, "结束日期");
        builder_set!(builder, study_type, "学习方式(如全日制)");
        let edu = builder.build().unwrap();
        result.push(edu);
        if !Confirm::with_theme(&theme)
            .with_prompt("继续添加教育经历?")
            .interact()
            .unwrap()
        {
            return Ok(result);
        }
    }
}
