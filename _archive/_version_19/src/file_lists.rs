use crate::source_file::title::title;
use crate::source_file::SourceFile;

pub fn file_lists(input_files: &Vec<SourceFile>) -> Vec<(String, String)> {
    let mut file_list: Vec<(String, String)> = vec![];

    input_files.iter().for_each(|file| {
        file_list.push((
            file.url.clone(),
            title(file.source_data.as_str()).unwrap().1,
        ));
    });
    file_list
}

// #[cfg(test)]
// mod test {
//     use crate::files::Files;
//     use crate::source_file::SourceFile;
//     use std::path::PathBuf;
//     #[test]
//     pub fn test_posts_basic() {
//         let mut content = Files { files: vec![] };
//         let lines = vec![
//             "-> title",
//             "",
//             "Alfa Bravooo",
//             "",
//             "-> attributes",
//             ">> type: post",
//             "",
//         ];
//         let sf = SourceFile {
//             source_data: lines.join("\n"),
//             source_path: PathBuf::from(
//                 "some/path/index.neo",
//             ),
//         };
//         content.files.push(sf);
//         assert_eq!(
//             content.all_posts(),
//             vec![(
//                 String::from("Alfa Bravooo"),
//                 String::from("/some/path/index.html")
//             )],
//         );
//     }
// }
