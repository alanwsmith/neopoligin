use crate::file_lists::file_lists;
use crate::neo_sections::neo_sections;
use crate::source_file::title::title;
use crate::source_file::SourceFile;
use minijinja::context;
use minijinja::path_loader;
use minijinja::Environment;
use std::fs;
use std::path::PathBuf;
use std::vec;
use walkdir::WalkDir;

pub fn build_site() {
    println!("Building the site");
    let template_dir = PathBuf::from("/Users/alan/workshop/alanwsmith.com/templates");
    let content_dir = PathBuf::from("/Users/alan/workshop/alanwsmith.com/content");
    let site_root_dir = PathBuf::from("/Users/alan/workshop/alanwsmith.com/_site");
    let mut env = Environment::new();
    let template_path = template_dir.display().to_string();
    env.set_loader(path_loader(template_path.as_str()));
    let mut source_files: Vec<SourceFile> = vec![];

    for entry in WalkDir::new(&content_dir).into_iter() {
        let initial_path = entry.as_ref().unwrap().path().to_path_buf();
        let initial_path2 = entry.unwrap().path().to_path_buf();
        let file_sub_path = initial_path2.strip_prefix(&content_dir).unwrap();
        if let Some(ext) = initial_path.extension() {
            if ext == "neo" {
                //println!("-------------------------");
                // println!("Loading: {}", &initial_path.display());
                let sf = SourceFile {
                    source_path: initial_path
                        .clone()
                        .strip_prefix(&content_dir)
                        .unwrap()
                        .to_path_buf(),
                    source_data: fs::read_to_string(initial_path).unwrap(),
                    url: format!("/{}", file_sub_path.parent().unwrap().display()),
                };
                source_files.push(sf);
            } else {
                let mut output_path = site_root_dir.clone();
                let file_sub_path = initial_path.strip_prefix(&content_dir).unwrap();
                output_path.push(file_sub_path);
                let parent_dir = output_path.parent().unwrap();
                if initial_path.to_path_buf().is_file() {
                    if !parent_dir.exists() {
                        fs::create_dir_all(parent_dir).unwrap();
                    }
                    fs::copy(initial_path, output_path).unwrap();
                }
            }
        }
    }

    let file_lists = file_lists(&source_files);

    source_files.iter().for_each(|source_file| {
        println!("-------------------------");
        println!("Outputting:\n{}", &source_file.source_path.display());
        let template = env.get_template(
            format!(
                "{}/base.j2",
                &source_file.template(&source_file.source_data).unwrap().1,
            )
            .as_str(),
        );
        let the_content = neo_sections(&source_file.source_data).unwrap().1;
        let the_date = &source_file
            .date(&source_file.source_data, "%B %Y")
            .unwrap()
            .1;
        let title_string = title(&source_file.source_data).unwrap().1;
        let output = template
            .unwrap()
            .render(context!(
                content => the_content,
                date => the_date,
                title_string => title_string,
                file_lists => file_lists
            ))
            .unwrap();
        let mut file_path = site_root_dir.clone();
        file_path.push(&source_file.source_path);
        file_path.set_extension("html");
        let dir_path = file_path.parent().unwrap();
        fs::create_dir_all(dir_path).unwrap();
        fs::write(file_path, output).unwrap();
    });

    println!("Process complete");

    //
}
