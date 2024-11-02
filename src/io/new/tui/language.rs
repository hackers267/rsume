use std::fmt::Display;

use super::{dialoguer_theme, input_prompt};
use crate::{
    builder_set,
    io::new::{
        data::{Language, LanguageBuilder},
        tui::select_prompt,
    },
};
use colored::Colorize;
use dialoguer::{Confirm, Input};

enum Frency {
    A1,
    A2,
    B1,
    B2,
    C1,
    C2,
    Other,
}

impl Display for Frency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Frency::A1 => f.write_str("A1(17%)"),
            Frency::A2 => f.write_str("A2(33%)"),
            Frency::B1 => f.write_str("B1(50%)"),
            Frency::B2 => f.write_str("B2(65%)"),
            Frency::C1 => f.write_str("C1(83%)"),
            Frency::C2 => f.write_str("C2(100%)"),
            Frency::Other => f.write_str("其它"),
        }
    }
}

impl Frency {
    fn to_percent(&self) -> Option<&str> {
        match self {
            Frency::A1 => Some("17%"),
            Frency::A2 => Some("33%"),
            Frency::B1 => Some("50%"),
            Frency::B2 => Some("65%"),
            Frency::C1 => Some("83%"),
            Frency::C2 => Some("100%"),
            Frency::Other => None,
        }
    }
}

const LIST: [Frency; 7] = [
    Frency::A1,
    Frency::A2,
    Frency::B1,
    Frency::B2,
    Frency::C1,
    Frency::C2,
    Frency::Other,
];

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
        match fluency.to_percent() {
            Some(percent) => {
                builder.fluency(percent.to_string());
            }
            None => {
                println!("{}", "请输入您的熟练度,示例：80%".green());
                let percent = Input::with_theme(&theme)
                    .with_prompt("熟练度(如80%):")
                    .interact_text()
                    .unwrap();
                builder.fluency(percent);
            }
        }
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
