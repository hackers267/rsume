use crate::templates::simple::{
    data_model::supported_resume_data::SupportedResumeData, supported_languages::SupportedLanguages,
};

/// Build the open source name and url
///
/// # Arguments
/// - data resume data
/// - lang supported language
///
/// # Result
/// The open source html
pub fn build_open_source(data: &SupportedResumeData, lang: &SupportedLanguages) -> String {
    let title = lang.open_source_title();
    let mut str = String::new();
    str.push_str(&format!("<div>{title}</div>"));
    let list = data
        .basics
        .profiles
        .iter()
        .filter_map(|profile| {
            profile.network.as_ref().and_then(|net| {
                profile
                    .url
                    .as_ref()
                    .map(|url| format!("<dt>{}</dt><dd>{}</dd>", net, url))
            })
        })
        .collect::<String>();
    format!("{}<dl>{}</dl>", title, list)
}
