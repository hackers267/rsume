use super::{dialoguer_theme, input_prompt};
use crate::{
    builder_set,
    io::new::data::{Work, WorkBuilder},
};
use colored::Colorize;
use dialoguer::Confirm;

/// 从tui中获取用户的工作经历
pub(super) fn get_work_from_tui() -> anyhow::Result<Vec<Work>> {
    let mut result = vec![];
    let theme = dialoguer_theme();
    loop {
        println!("{}", "请输入您的工作经历:".green());
        let mut builder = WorkBuilder::default();
        builder_set!(builder, name, "公司名称");
        builder_set!(builder, description, "职位介绍");
        builder_set!(builder, start_date, "开始时间");
        builder_set!(builder, end_date, "离开时间");
        let highlights = get_highlights_from_tui()?;
        builder.highlights(highlights);
        let work = builder.build().unwrap();
        result.push(work);
        if !Confirm::with_theme(&theme)
            .with_prompt("继续添加工作经历?")
            .interact()
            .unwrap()
        {
            return Ok(result);
        }
    }
}

fn get_highlights_from_tui() -> anyhow::Result<Vec<String>> {
    let mut result = vec![];
    let theme = dialoguer_theme();
    loop {
        let value = input_prompt("技术栈或重点项目")?;
        result.push(value);
        if !Confirm::with_theme(&theme)
            .with_prompt("继续添加技术栈或重点项目?")
            .interact()
            .unwrap()
        {
            return Ok(result);
        }
    }
}
