use clap::{Arg, Command};
use std::{ffi::OsStr, fs, fs::OpenOptions, io::prelude::Write, path::Path};
use tera::{Context, Tera};
use thiserror::Error;
mod schema;
use schema::Resume;

const JSON_EXT: &str = "json";
const YAML_EXT: &str = "yaml";
const YML_EXT: &str = "yml";
const TEMPLATE: &str = "resume.tex";
const TEMPLATE_DIR: &str = "templates/*.tex";
const OUTPUT_DIR: &str = "./output/generated.tex";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Command::new("Latex Resume Generator")
        .version("0.1.0")
        .author("M. Lawal <muideen.lawal320@gmail.com>")
        .about("Latex CV generator written in rust")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .required(true)
                .help("resume data file in either json or yaml format"),
        )
        .arg(
            Arg::new("template")
                .short('t')
                .long("template")
                .default_value(TEMPLATE)
                .required(false)
                .help("template file written in tera`s dialect"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .default_value(OUTPUT_DIR)
                .required(false)
                .help("location on disk where you want the output to be written"),
        )
        .get_matches();

    let output_file = cli
        .get_one::<String>("output")
        .expect("output file path is missing and its default value is unset")
        .as_str();

    let template_file = cli
        .get_one::<String>("template")
        .expect("template file is missing in path")
        .as_str();

    let resume_data = cli
        .get_one::<String>("input")
        .expect("input file is missing");

    let result = process_resume(resume_data.as_str(), template_file)?;

    write_template(result.as_str(), output_file)?;

    println!("Output file written to {:?}", &output_file);

    Ok(())
}

fn process_resume(file_path: &str, template: &str) -> Result<String, Error> {
    let extension = Path::new(file_path).extension().and_then(OsStr::to_str);
    let data = fs::read_to_string(file_path)?;

    let resume_data = match extension {
        Some(JSON_EXT) => Ok(serde_json::from_str::<Resume>(&data)?),
        Some(YAML_EXT | YML_EXT) => Ok(serde_yaml::from_str::<Resume>(&data)?),
        _ => Err(Error::InvalidTypeError),
    };

    let rendered =
        Tera::new(TEMPLATE_DIR)?.render(template, &Context::from_serialize(resume_data?)?)?;

    Ok(rendered)
}

fn write_template(data: &str, write_path: &str) -> std::io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(write_path)?;

    writeln!(&file, "{}", data)?;

    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("file error: {0}")]
    FileError(#[from] std::io::Error),
    #[error("error parsing json: {0}")]
    JsonParserError(#[from] serde_json::Error),
    #[error("error parsing yaml: {0}")]
    YamlParserError(#[from] serde_yaml::Error),
    #[error("download error: {0}")]
    ParsingError(#[from] tera::Error),
    #[error("invalid file type. Expects '.json' or '.yaml' file.")]
    InvalidTypeError,
}

#[cfg(test)]
mod tests {

    use super::*;

    const EXPECTED: &str = r#"\name{ John Doe }
\subtitle{ Programmer }
\setemail{ john@gmail.com }
\setlinkedin{ john-doe }
\setgithub{ John }
\setphone{ (912) 555-4321 }"#;

    #[test]
    fn test_process_resume() {
        let actual = process_resume("./resources/resume.json", "test.tex").unwrap();
        assert_eq!(actual.as_str(), EXPECTED);
    }

    #[test]
    fn test_process_resume_yml() {
        let actual = process_resume("./resources/resume.yaml", "test.tex").unwrap();
        assert_eq!(actual.as_str(), EXPECTED);
    }
}
