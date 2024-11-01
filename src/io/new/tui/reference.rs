use super::{dialoguer_theme, input_prompt};
use crate::{
    builder_set,
    io::new::data::{Reference, ReferenceBuilder},
};
use dialoguer::Confirm;

/// 从tui中获取用户的参考，比如书籍
pub(super) fn get_ref_from_tui() -> anyhow::Result<Vec<Reference>> {
    let mut result = vec![];
    let theme = dialoguer_theme();
    loop {
        let mut builder = ReferenceBuilder::default();
        builder_set!(builder, name, "名称");
        let reference = builder.build().unwrap();
        result.push(reference);
        if !Confirm::with_theme(&theme)
            .with_prompt("继续添加参考?，比如书籍")
            .interact()
            .unwrap()
        {
            return Ok(result);
        }
    }
}
