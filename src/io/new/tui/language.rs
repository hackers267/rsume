use super::{dialoguer_theme, input_prompt};
use crate::{
    builder_set,
    io::new::{
        data::{Language, LanguageBuilder},
        tui::select_prompt,
    },
};
use colored::Colorize;
use dialoguer::Confirm;

const LIST: [&str; 5] = ["A", "B", "C", "D", "E"];

/// 从TUI中获取用户输出的语言选项
pub(super) fn get_languages_from_tui() -> anyhow::Result<Vec<Language>> {
    let mut result = vec![];
    let theme = dialoguer_theme();
    loop {
        println!("{}", "添加讲述的语言和熟练度:".green());
        let mut builder = LanguageBuilder::default();
        builder_set!(builder, language, "语言");
        let index = select_prompt("熟练度", &LIST, 5)?;
        let fluency = LIST.get(index).unwrap();
        builder.fluency(fluency.to_string());
        let language = builder.build().unwrap();
        result.push(language);
        if !Confirm::with_theme(&theme)
            .with_prompt("继续添加语言信息?")
            .interact()
            .unwrap()
        {
            return Ok(result);
        }
    }
}
