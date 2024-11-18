use minijinja::context;

use crate::templates::simple::{
    data_model::supported_resume_data::SupportedResumeData,
    shared::render_template::render_template, supported_languages::SupportedLanguages,
};

/// Return the basics wrapper as HTML.
pub fn build_basics_wrapper(
    resume_data: &SupportedResumeData,
    language: &SupportedLanguages,
) -> String {
    let city = &resume_data.basics.location.city;
    let address = &resume_data.basics.location.address;
    let address = format!("{} {}", city, address);
    let school = &resume_data
        .education
        .first()
        .map(|v| v.institution.clone())
        .unwrap_or_default();
    let education = &resume_data
        .education
        .first()
        .map(|v| v.study_type.clone())
        .unwrap_or_default();
    let rendered_template = render_template(
        include_str!("index.html"),
        context!(
            name => resume_data.basics.name,
            image => resume_data.basics.image,
            label => resume_data.basics.label,
            email => resume_data.basics.email,
            phone => resume_data.basics.phone,
            address => address,
            school => school,
            education => education,
            email_icon => include_str!("icons/email.svg"),
            phone_icon => include_str!("icons/phone.svg"),
            address_icon => include_str!("icons/address.svg"),
        ),
    );

    match rendered_template {
        Ok(t) => t,
        Err(_) => panic!("Failed to render basics template."),
    }
}
