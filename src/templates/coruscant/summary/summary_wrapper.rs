use crate::templates::coruscant::{
    data_model::supported_resume_data::SupportedResumeData, supported_languages::SupportedLanguages,
};

/// 创建个人优势
pub fn build_summary_wrapper(
    resume: &SupportedResumeData,
    _language: &SupportedLanguages,
) -> String {
    match &resume.basics.summary {
        Some(summary) => {
            let mut str = String::new();
            let title = _language.summary_title();
            str.push_str(&format!("<div class='section-title'>{}</div>", title));
            str.push_str(&format!(
                "<section class='entry' style='text-indent:2em'><div class='box-column'>{summary}</div></section>"
            ));
            str
        }
        None => String::new(),
    }
}
