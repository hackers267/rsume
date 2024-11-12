use json_resume::Resume;
use minijinja::context;

use crate::{
    templates::{
        simple::{
            basics::basics_box::build_basics_wrapper, book::book_wrapper::build_book,
            education::education_wrapper::build_education_wrapper,
            project::project_wrapper::build_project_wrapper,
            publication::publication_wrapper::build_publication_wrapper,
            summary::summary_wrapper::build_summary_wrapper,
            work::work_wrapper::build_work_wrapper,
        },
        template::Template,
    },
    GloballySupportedLanguages,
};

use super::{
    data_model::supported_resume_data::SupportedResumeData,
    shared::render_template::render_template, supported_languages::SupportedLanguages,
};

/// A modern, minimalist, and professional resume design.
pub struct Simple {
    /// Underlying personal data defining the content of the resume (like education, work experience, ...).
    resume_data: SupportedResumeData,
    /// Language used in the section headers of the resume.
    language: SupportedLanguages,
}
impl Template for Simple {
    fn new(
        json_resume_data: Resume,
        language: &GloballySupportedLanguages,
    ) -> Result<Self, String> {
        Ok(Simple {
            resume_data: SupportedResumeData::try_from(json_resume_data)?,
            language: SupportedLanguages::try_from(language)?,
        })
    }

    fn build(&self) -> String {
        let rendered_template = render_template(
            include_str!("index.html"),
            context!(
                style => include_str!("style.css"),
                basics => build_basics_wrapper(&self.resume_data, &self.language),
                work => build_work_wrapper(&self.resume_data, &self.language),
                education => build_education_wrapper(&self.resume_data, &self.language),
                publications => build_publication_wrapper(&self.resume_data, &self.language),
                project => build_project_wrapper(&self.resume_data,&self.language),
                summary => build_summary_wrapper(&self.resume_data,&self.language),
                book => build_book(&self.resume_data, &self.language)
            ),
        );

        match rendered_template {
            Ok(t) => t,
            Err(_) => panic!("Failed to render root template."),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::{fs, path::PathBuf};

    use crate::{generate_pdf, io::load_json_resume::load_json_resume};

    fn html_is_different(
        resume_data_path: &PathBuf,
        language: &GloballySupportedLanguages,
        html_path: &PathBuf,
    ) -> bool {
        let generated_html = Simple::new(load_json_resume(resume_data_path).unwrap(), language)
            .unwrap()
            .build();

        let previous_html = fs::read_to_string(html_path);

        if previous_html.is_ok() && generated_html == previous_html.unwrap() {
            return false;
        }

        fs::write(html_path, generated_html).unwrap();
        true
    }

    #[test]
    fn build_example_en() {
        let resume_data_path = PathBuf::from("examples/kirk_resume_en.yaml");
        let target_path = PathBuf::from("examples/coruscant_en.pdf");
        let html_path = resume_data_path.parent().unwrap().join("coruscant_en.html");
        let template_enum = crate::AvailableTemplates::Coruscant;
        let language = GloballySupportedLanguages::EN;

        if !html_is_different(&resume_data_path, &language, &html_path) {
            return;
        }

        let _ = fs::remove_file(&target_path);
        assert!(!target_path.is_file());

        generate_pdf(
            resume_data_path,
            target_path.clone(),
            template_enum,
            language,
        )
        .unwrap();
        assert!(target_path.is_file());
    }

    #[test]
    fn build_example_de() {
        let resume_data_path = PathBuf::from("examples/kirk_resume_de.yaml");
        let target_path = PathBuf::from("examples/coruscant_de.pdf");
        let html_path = resume_data_path.parent().unwrap().join("coruscant_de.html");
        let template_enum = crate::AvailableTemplates::Coruscant;
        let language = GloballySupportedLanguages::DE;

        if !html_is_different(&resume_data_path, &language, &html_path) {
            return;
        }

        let _ = fs::remove_file(&target_path);
        assert!(!target_path.is_file());

        generate_pdf(
            resume_data_path,
            target_path.clone(),
            template_enum,
            language,
        )
        .unwrap();
        assert!(target_path.is_file());
    }
}