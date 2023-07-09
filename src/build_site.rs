use crate::sections::sections;
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

    let template_dir = PathBuf::from("/Users/alan/workshop/neopolengine/templates");
    let content_dir = PathBuf::from("/Users/alan/workshop/alanwsmith.com/content");
    let site_root_dir = PathBuf::from("/Users/alan/workshop/alanwsmith.com/site");
    let mut env = Environment::new();
    // env.set_source(Source::from_path(template_dir));
    let template_path = template_dir.display().to_string();
    env.set_loader(path_loader(template_path.as_str()));
    //env.set_loader(path_loader("path/to/templates"));
    let mut source_files: Vec<SourceFile> = vec![];

    for entry in WalkDir::new(&content_dir).into_iter() {
        let initial_path = entry.unwrap().path().to_path_buf();
        if let Some(ext) = initial_path.extension() {
            if ext == "neo" {
                println!("Loading: {}", &initial_path.display());
                let sf = SourceFile {
                    source_path: initial_path
                        .clone()
                        .strip_prefix(&content_dir)
                        .unwrap()
                        .to_path_buf(),
                    source_data: fs::read_to_string(initial_path).unwrap(),
                };
                source_files.push(sf);
            }
        }
    }

    source_files.iter().for_each(|source_file| {
        println!("Outputting: {}", &source_file.source_path.display());
        let template = env.get_template(
            format!(
                "{}.j2",
                // "{}/{}.j2",
                // template_dir.to_path_buf().as_os_str().to_str().unwrap(),
                &source_file.template(&source_file.source_data).unwrap().1,
            )
            .as_str(),
        );

        let the_content = sections(&source_file.source_data).unwrap().1;
        let the_date = &source_file
            .date(&source_file.source_data, "%B %Y")
            .unwrap()
            .1;
        let the_title = title(&source_file.source_data).unwrap().1;

        let output = template
            .unwrap()
            .render(context!(
                content => the_content,
                date => the_date,
                title => the_title,
            ))
            .unwrap();
        let mut file_path = site_root_dir.clone();
        file_path.push(&source_file.source_path);
        file_path.set_extension("html");
        let dir_path = file_path.parent().unwrap();
        fs::create_dir_all(dir_path).unwrap();
        fs::write(file_path, output).unwrap();
    });

    //
}
