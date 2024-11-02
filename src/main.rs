mod io;
mod templates;
mod utils;

use crate::{
    io::{
        load_json_resume::load_json_resume, resolve_image_path::resolve_image_path,
        save_to_pdf::save_to_pdf,
    },
    templates::Coruscant,
};
use clap::{Parser, Subcommand};
use io::new;
use std::{
    fs,
    path::{Path, PathBuf},
};
use templates::template::Template;
use utils::get_filename;

/// Program for generating a resume from JSONResume data.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Init {
        #[arg(short, long, default_value = "data")]
        name: String,
    },
    New {
        #[arg(short, long, default_value = "data")]
        name: String,
    },
    Gen {
        /// Path to the data describing your resume. It needs to comply with theJSONResume schema (see https://jsonresume.org/).
        resume_data_path: PathBuf,

        /// Location where the generated PDF should be stored.
        target_path: PathBuf,

        /// Template that should be used. Currently, the only available option is 'coruscant'. Default is 'coruscant'.
        #[arg(short, long)]
        template: Option<String>,

        /// Language of the template. Available options are 'english','chinese' and 'deutsch'. Default is english.
        #[arg(short, long)]
        language: Option<String>,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args.command {
        Commands::Init { name } | Commands::New { name } => {
            let path = get_filename(&name).unwrap();
            new::new(&path)
        }
        Commands::Gen {
            resume_data_path,
            target_path,
            template,
            language,
        } => {
            let language = match language {
                Some(language_string) => GloballySupportedLanguages::try_from(language_string)
                    .map_err(|e| anyhow::anyhow!(e))?,
                None => GloballySupportedLanguages::EN,
            };

            let template = match template {
                Some(template_string) => {
                    AvailableTemplates::try_from(template_string).map_err(|e| anyhow::anyhow!(e))?
                }
                None => AvailableTemplates::Coruscant,
            };

            generate_pdf(resume_data_path, target_path, template, language)?;

            Ok(())
        }
    }
}

/// Generate a resume and save it as a html and a PDF.
pub fn generate_pdf(
    resume_data_path: PathBuf,
    target_path: PathBuf,
    template_enum: AvailableTemplates,
    language: GloballySupportedLanguages,
) -> anyhow::Result<()> {
    let mut resume_data = load_json_resume(&resume_data_path).unwrap();

    if let Some(ref mut basics) = resume_data.basics {
        basics.image = resolve_image_path(&resume_data_path, &basics.image);
    }

    let template = match template_enum {
        AvailableTemplates::Coruscant => Coruscant::new(resume_data, &language).unwrap(),
    };

    let html_resume = template.build();
    save_to_html(&html_resume, &target_path)?;
    save_to_pdf(html_resume, &target_path)?;

    Ok(())
}

fn save_to_html(html: &String, target_path: &Path) -> anyhow::Result<()> {
    let target = target_path.with_extension("html");
    fs::write(target, html)?;
    Ok(())
}

/// All templates that are available.
pub enum AvailableTemplates {
    /// A modern, minimalist, and professional resume design.
    Coruscant,
}
impl AvailableTemplates {
    /// Try constructing a this struct from a string.
    pub fn try_from(template_string: String) -> Result<Self, String> {
        match template_string.to_lowercase().as_str() {
            "coruscant" => Ok(AvailableTemplates::Coruscant),
            _ => Err(format!("{template_string} is not a supported template.")),
        }
    }
}

/// Language in which the resume should be generated in.
pub enum GloballySupportedLanguages {
    /// English
    EN,
    /// German
    DE,
    /// Chinese
    CN,
}
impl GloballySupportedLanguages {
    /// Try constructing a this struct from a string.
    pub fn try_from(language_string: String) -> Result<Self, String> {
        match language_string.to_lowercase().as_str() {
            "english" | "en" => Ok(GloballySupportedLanguages::EN),
            "chinese" | "cn" => Ok(GloballySupportedLanguages::CN),
            "deutsch" | "german" | "de" => Ok(GloballySupportedLanguages::DE),
            _ => Err(format!("{language_string} is not a supported language.")),
        }
    }
}
