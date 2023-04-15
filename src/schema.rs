use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Resume {
    basics: Basics,
    work: Vec<Work>,
    education: Vec<Education>,
    awards: Vec<Award>,
    certificates: Vec<Certificate>,
    publications: Vec<Publication>,
    skills: Vec<Skill>,
    languages: Vec<Language>,
    projects: Vec<Project>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Basics {
    name: String,
    job_title: String,
    image: String,
    email: String,
    phone: String,
    url: String,
    linkedin: String,
    github: String,
    summary: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Work {
    name: String,
    position: String,
    url: String,
    start_date: String,
    end_date: String,
    description: Vec<String>,
    keywords: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Education {
    institution: String,
    country: String,
    course_of_study: String,
    study_type: String,
    start_date: String,
    end_date: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Award {
    title: String,
    date: String,
    awarder: String,
    summary: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Certificate {
    name: String,
    date: String,
    issuer: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Publication {
    name: String,
    publisher: String,
    release_date: String,
    url: String,
    summary: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Skill {
    name: String,
    keywords: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Language {
    language: String,
    fluency: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    name: String,
    description: String,
    keywords: Vec<String>,
    url: String,
}
