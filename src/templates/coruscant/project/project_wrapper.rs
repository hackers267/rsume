use json_resume::{Feature, Project};
use minijinja::context;

use crate::templates::coruscant::{
    data_model::supported_resume_data::SupportedResumeData,
    shared::render_template::render_template, supported_languages::SupportedLanguages,
};

/// Return the project wrapper as HTML.
pub fn build_project_wrapper(
    resume_data: &SupportedResumeData,
    language: &SupportedLanguages,
) -> String {
    if resume_data.projects.is_empty() {
        return String::new();
    }

    let rendered_template = render_template(
        include_str!("index.html"),
        context!(entries => build_entries(&resume_data.projects), title => language.project_section_title()),
    );

    match rendered_template {
        Ok(t) => t,
        Err(_) => panic!("Failed to render projects template."),
    }
}

fn build_entries(project: &Vec<Project>) -> String {
    let mut entries_html = String::new();

    for entry in project {
        let highlight = build_entry_highlight(entry);
        let keywords = build_entry_keyword(entry);
        let duties = build_entry_duty(entry);
        let features = build_entry_feature(entry);
        let profits = build_entry_profit(entry);
        let desc = build_entry_desc(entry);
        let entry = Entry {
            desc,
            start_date: entry.start_date.clone(),
            end_date: entry.end_date.clone(),
            title: entry.name.clone(),
            highlights: highlight.clone(),
            keywords: keywords.clone(),
            duties,
            features,
            profits,
        };
        entries_html.push_str(&build_entry(entry));
    }

    entries_html
}

pub struct Entry {
    start_date: Option<String>,
    end_date: Option<String>,
    title: Option<String>,
    highlights: String,
    keywords: String,
    duties: String,
    features: String,
    profits: String,
    desc: String,
}

pub fn build_entry(entry: Entry) -> String {
    let Entry {
        start_date,
        end_date,
        title,
        highlights,
        keywords,
        duties,
        features,
        profits,
        desc,
    } = entry;
    let rendered_template = render_template(
        include_str!("share.html"),
        context!(
            start_date => start_date.unwrap_or_default(),
            end_date => end_date.unwrap_or_default(),
            title => title.unwrap_or_default(),
            highlights => highlights,
            desc=>desc,
            keywords => keywords,
            duties => duties,
            features => features,
            profits => profits
        ),
    );

    match rendered_template {
        Ok(t) => t,
        Err(_) => panic!("Failed to render entry template."),
    }
}

fn build_entry_desc(project_entry: &Project) -> String {
    let mut entry_body = String::new();
    if let Some(desc) = &project_entry.description {
        entry_body.push_str(desc);
    }
    entry_body
}

fn build_entry_highlight(project_entry: &Project) -> String {
    let mut entry_body = String::new();
    entry_body.push_str("<div class='entry-label'>工作亮点</div>");
    entry_body.push_str("\n<ul>");

    for highlight in &project_entry.highlights {
        entry_body.push_str(&format!(
            "
            \n<li>{highlight}</li>
            "
        ));
    }

    entry_body.push_str("\n</ul>");

    entry_body
}

fn build_entry_duty(project_entry: &Project) -> String {
    let mut entry_body = String::new();
    entry_body.push_str("<div class='entry-label'>职责</div>");
    entry_body.push_str("\n<ul>");

    for duty in &project_entry.duties {
        entry_body.push_str(&format!(
            "
            \n<li>{duty}</li>
            "
        ));
    }

    entry_body.push_str("\n</ul>");

    entry_body
}
fn build_entry_feature(entry: &Project) -> String {
    let mut entry_body = String::new();
    entry_body.push_str("<div class='entry-label'>经典案例</div>");

    for feature in &entry.features {
        let Feature {
            situation,
            task,
            action,
            result,
            name,
        } = feature;
        entry_body.push_str(&format!("<div class='feature-title'>{name}</div>"));
        entry_body.push_str("\n<ol>");
        let action = build_paragraph(action);
        let result = build_paragraph(result);
        entry_body.push_str(&format!(
            r###"
            <li><span class='bold'>情景: </span>{situation}</li>
            <li><span class='bold'>任务: </span>{task}</li>
            <li><span class='bold'>方案: </span>{action}</li>
            <li><span class='bold'>结果: </span>{result}</li>
            "###
        ));
        entry_body.push_str("\n</ol>");
    }

    entry_body
}
fn build_paragraph(content: &str) -> String {
    if content.contains("\\n") {
        let list = content
            .split("\\n")
            .map(|v| format!("<li>{v}</li>"))
            .fold(String::new(), |acc, cur| format!("{}{}", acc, cur));
        format!("<ol>{}</ol>", list)
    } else {
        content.to_string()
    }
}
fn build_entry_profit(project_entry: &Project) -> String {
    let mut entry_body = String::new();
    entry_body.push_str("<div class='entry-label'>成果</div>");
    entry_body.push_str("\n<ul>");

    for profit in &project_entry.profits {
        entry_body.push_str(&format!(
            "
            \n<li>{profit}</li>
            "
        ));
    }

    entry_body.push_str("\n</ul>");

    entry_body
}

fn build_entry_keyword(project_entry: &Project) -> String {
    let mut result = String::new();
    for ele in &project_entry.keywords {
        result.push_str(&format!("<li>{}</li>", ele.0));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_action() {
        let action = "Learn Rust";
        let result = build_paragraph(action);
        assert_eq!(result, action);
    }
    #[test]
    fn mutil_actions() {
        let actions = "Learn Rust.\\nLearn Cargo";
        let result = build_paragraph(actions);
        let actual = "<ol><li>Learn Rust.</li><li>Learn Cargo</li></ol>";
        assert_eq!(result, actual);
    }
}
