use crate::templates::coruscant::{
    data_model::supported_resume_data::SupportedResumeData, supported_languages::SupportedLanguages,
};

// TODO: 添加多语言支持
pub fn build_summary_wrapper(
    resume: &SupportedResumeData,
    _language: &SupportedLanguages,
) -> String {
    match &resume.basics.summary {
        Some(summary) => {
            let mut str = String::new();
            str.push_str("<div class='section-title'>自我评价</div>");
            str.push_str(&format!(
                "<section class='entry' style='text-indent:2em'><div class='box-column'>{summary}</div></section>"
            ));
            str
        }
        None => String::new(),
    }
}
