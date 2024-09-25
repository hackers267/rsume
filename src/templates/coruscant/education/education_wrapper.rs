use crate::templates::coruscant::{
    data_model::{education::Education, supported_resume_data::SupportedResumeData},
    shared::entry::Entry,
};

pub struct EducationWrapper {
    resume_data: SupportedResumeData,
}
impl EducationWrapper {
    pub fn from(resume_data: SupportedResumeData) -> Self {
        Self { resume_data }
    }

    pub fn build(&self) -> String {
        if self.resume_data.education.is_empty() {
            return String::new();
        }

        let entries = build_entries(&self.resume_data.education);

        let html = format!(
            "
        <div class='section-title'>
            Education
        </div>
        {entries}
        "
        );

        html
    }
}

fn build_entries(education: &Vec<Education>) -> String {
    let mut entries_html = String::new();

    for education_entry in education {
        let entry = Entry {
            start_date: education_entry.start_date.clone(),
            end_date: education_entry.end_date.clone(),
            title: education_entry.institution.clone(),
            body: build_entry_body(education_entry),
        };
        entries_html.push_str(&entry.build());
    }

    entries_html
}

fn build_entry_body(education_entry: &Education) -> String {
    let study_type = match education_entry.clone().study_type {
        None => "".to_string(),
        Some(s) => format!("{s} in "),
    };
    let area = education_entry.clone().area;

    format!("{study_type}{area}")
}
