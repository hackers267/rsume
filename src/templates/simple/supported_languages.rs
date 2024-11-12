use crate::GloballySupportedLanguages;

/// Languages supported by this template.
pub enum SupportedLanguages {
    EN,
    DE,
    CN,
}
impl SupportedLanguages {
    pub fn try_from(language: &GloballySupportedLanguages) -> Result<Self, String> {
        match language {
            GloballySupportedLanguages::EN => Ok(Self::EN),
            GloballySupportedLanguages::DE => Ok(Self::DE),
            GloballySupportedLanguages::CN => Ok(Self::CN),
        }
    }

    pub fn languages_section_title(&self) -> String {
        match self {
            SupportedLanguages::EN => "Languages".to_string(),
            SupportedLanguages::DE => "Sprachen".to_string(),
            SupportedLanguages::CN => "语言".to_string(),
        }
    }

    pub fn work_section_title(&self) -> String {
        match self {
            SupportedLanguages::EN => "Experience".to_string(),
            SupportedLanguages::DE => "Arbeitserfahrung".to_string(),
            SupportedLanguages::CN => "工作经历".to_string(),
        }
    }
    pub fn project_section_title(&self) -> String {
        match self {
            SupportedLanguages::EN => "Project".to_string(),
            SupportedLanguages::DE => "Projekt".to_string(),
            SupportedLanguages::CN => "项目经验".to_string(),
        }
    }

    pub fn education_section_title(&self) -> String {
        match self {
            SupportedLanguages::EN => "Education".to_string(),
            SupportedLanguages::CN => "教育经历".to_string(),
            SupportedLanguages::DE => "Bildung".to_string(),
        }
    }

    pub fn publication_section_title(&self) -> String {
        match self {
            SupportedLanguages::EN => "Publications".to_string(),
            SupportedLanguages::DE => "Veröffentlichungen".to_string(),
            SupportedLanguages::CN => "出版物".to_string(),
        }
    }

    pub fn skills_section_title(&self) -> String {
        match self {
            SupportedLanguages::EN => "Skills".to_string(),
            SupportedLanguages::CN => "技能".to_string(),
            SupportedLanguages::DE => "Kenntnisse".to_string(),
        }
    }
    pub fn open_source_title(&self) -> String {
        match self {
            SupportedLanguages::EN => "OpenSource".to_string(),
            SupportedLanguages::DE => "OpenSource".to_string(),
            SupportedLanguages::CN => "开源库".to_string(),
        }
    }
    pub fn book_title(&self) -> String {
        match self {
            SupportedLanguages::EN => "Book".to_string(),
            SupportedLanguages::DE => "Buch".to_string(),
            SupportedLanguages::CN => "书籍".to_string(),
        }
    }
    pub fn summary_title(&self) -> String {
        match self {
            SupportedLanguages::EN => "PersonalAdvantage".to_string(),
            SupportedLanguages::DE => "PersönlicherVorteil".to_string(),
            SupportedLanguages::CN => "个人优势".to_string(),
        }
    }
}
