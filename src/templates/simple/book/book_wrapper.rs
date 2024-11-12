use minijinja::context;

use crate::templates::simple::{
    data_model::supported_resume_data::SupportedResumeData,
    shared::render_template::render_template, supported_languages::SupportedLanguages,
};

pub fn build_book(data: &SupportedResumeData, lang: &SupportedLanguages) -> String {
    let title = lang.book_title();
    let mut list = String::new();
    for ele in data.references.iter() {
        list.push_str(&format!("<li>{ele}</li>"));
    }
    let template = render_template(
        include_str!("index.html"),
        context!(title=>title,body=>list),
    );
    match template {
        Ok(t) => t,
        Err(_) => panic!("Failed to render book template"),
    }
}
