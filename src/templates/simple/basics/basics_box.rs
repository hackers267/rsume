use minijinja::context;

use crate::templates::simple::{
    basics::{
        contact_info::contact_info_wrapper::build_contact_info_wrapper,
        languages::language_wrapper::build_languages_wrapper, open_source::build_open_source,
        skills::skills_wrapper::build_skills_wrapper,
    },
    data_model::supported_resume_data::SupportedResumeData,
    shared::render_template::render_template,
    supported_languages::SupportedLanguages,
};

/// Return the basics wrapper as HTML.
pub fn build_basics_wrapper(
    resume_data: &SupportedResumeData,
    language: &SupportedLanguages,
) -> String {
    let rendered_template = render_template(
        include_str!("index.html"),
        context!(
            name => resume_data.basics.name,
            image => resume_data.basics.image,
            label => resume_data.basics.label,
            contact_info => build_contact_info_wrapper(resume_data),
            languages => build_languages_wrapper(resume_data, language),
            skills => build_skills_wrapper(resume_data, language),
            gits => build_open_source(resume_data,language),
        ),
    );

    match rendered_template {
        Ok(t) => t,
        Err(_) => panic!("Failed to render basics template."),
    }
}
