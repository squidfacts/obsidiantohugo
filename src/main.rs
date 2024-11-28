use clap::Parser;
use std::fs;
use std::fs::read_to_string;
use std::fs::File;
use std::io::prelude::*;

fn process_blog(
    hugo_base_dir: &String,
    obsidan_img_folder: &String,
    filename: &str,
    _obsidan_dir: &String,
) {
    fs::create_dir_all("").unwrap();

    let mut count = 0;

    let mut parsed_path = String::new();

    let mut description: Option<String> = None;
    let mut title: Option<String> = None;
    let mut date: Option<String> = None;
    let mut file: Option<fs::File> = None;
    let mut tags: Vec<String> = Vec::new();
    let mut write = false;
    for line in read_to_string(filename).unwrap().lines() {
        let s = line.to_string();
        // println!("{}",s);

        if s.contains("staticPath:") {
            if let Some(path_part) = s.splitn(2, ':').nth(1) {
                let path_part_trimmed = path_part.trim();

                parsed_path = path_part_trimmed.to_string();

                let mut static_base = hugo_base_dir.clone();

                static_base.push_str("static/");
                static_base.push_str(&parsed_path);

                println!("created static folder {}", static_base);
                fs::create_dir_all(static_base.clone()).unwrap();

                let mut hugo_md_file = hugo_base_dir.clone();
                hugo_md_file.push_str("content/posts/");
                hugo_md_file.push_str(&parsed_path);
                hugo_md_file.push_str(".md");
                println!("Creating hugo md file {}", hugo_md_file);
                file = Some(File::create(hugo_md_file.clone()).unwrap());
            }
        }
        if s.contains("Description:") {
            if let Some(parsed_description) = s.splitn(2, ':').nth(1) {
                let parsed_description: &str = parse_text(parsed_description);
                description = Some(parsed_description.to_string());
            }
        }

        if s.contains("Title:") {
            if let Some(parsed_description) = s.splitn(2, ':').nth(1) {
                let parsed_description: &str = parse_text(parsed_description);
                title = Some(parsed_description.to_string());
            }
        }

        if s.contains("Date:") {
            if let Some(parsed_description) = s.splitn(2, ':').nth(1) {
                date = Some(parsed_description.to_string());
            }
        }
        if s.contains(" - ") {
            if let Some(parsed_tag) = s.splitn(2, "-").nth(1) {
                let parsed_tag = &parsed_tag[1..];
                tags.push(parsed_tag.to_string());
            }
        }
        if s.contains("---") {
            if !file.is_none() {
                write = true;
                write_header(&file, &title, &date, &description, &tags);
            }
        }
        if s.contains("![[") {
            let imagename = s
                .split("[[")
                .nth(1)
                .expect("Couldn't parse")
                .split("]]")
                .nth(0)
                .unwrap();

            let mut attachment_base = obsidan_img_folder.clone();
            attachment_base.push_str(imagename);

            let mut dest_path = hugo_base_dir.clone();
            dest_path.push_str("static/");
            dest_path.push_str(&parsed_path);
            dest_path.push_str("/");
            dest_path.push_str(&parsed_path);
            dest_path.push_str(&count.to_string());
            dest_path.push_str(".png");

            println!("copied {} to {}", attachment_base, dest_path);

            fs::copy(attachment_base.clone(), dest_path.clone()).unwrap();

            let new_image_markdown: String = "![img](".to_owned()
                + &parsed_path
                + "/"
                + &parsed_path
                + &count.to_string()
                + ".png)";

            match file {
                Some(ref file) =>write_to_file(file, new_image_markdown),
                None => panic!("Trying to write image markdown to file without declaring `staticPath` have you set in the obsidian file?"),
            }

            count += 1;
        } else {
            if write && !s.contains("---") {
                match file {
                Some(ref file) =>write_to_file(file, s),
                None => panic!("Trying to text to file without declaring `staticPath` have you set in the obsidian file?"),
            }
            }
        }
    }
}

fn write_to_file(mut file: &File, str: String) {
    file.write((str + "\n").as_bytes()).unwrap();
}

fn parse_text(s: &str) -> &str {
    let mut s = &s[1..];

    if s.contains("\"") {
        //trim qoutes
        s = &s[1..];
        s = &s[0..s.len() - 1];
    }

    return s;
}

fn write_header(
    file: &Option<File>,
    title: &Option<String>,
    date: &Option<String>,
    description: &Option<String>,
    tags: &Vec<String>,
) {
    let mut file = file.as_ref().unwrap();

    file.write(b"+++\n").unwrap();

    let title = "title = '".to_owned() + title.as_ref().expect("Title: Not set") + "'\n";

    file.write(title.as_bytes()).unwrap();

    let date: String =
        "date = ".to_owned() + date.as_ref().expect("Date: not set") + "T00:13:48-05:00\n";

    file.write(date.as_bytes()).unwrap();

    let draft: String = "draft = false\n".to_string();

    file.write(draft.as_bytes()).unwrap();

    let description: String =
        "summary = '".to_owned() + description.as_ref().expect("Description: not set") + "'\n";

    file.write(description.as_bytes()).unwrap();

    if !tags.is_empty() {
        let mut tags_str = "tags = [".to_string();

        for i in tags {
            let tag_str = "'".to_owned() + i + "',";

            tags_str.push_str(&tag_str);
        }

        //remove trailing comma
        let  tags_str = &tags_str[0..tags_str.len()-1];

        let mut tags_str = tags_str.to_string();

        tags_str.push_str("]\n");

        file.write(tags_str.as_bytes()).unwrap();
    }

    file.write(b"+++\n").unwrap();
}

/// Progam to generate hugo markdown from obsidian markdown
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// path to obsidian source markdown
    #[arg(long)]
    obsidian_dir: String,

    ///path to obsidian img folder
    #[arg(long)]
    obsidian_dir_imgs: String,

    /// path to hugo folder
    #[arg(long)]
    hugo_path: String,
}

fn main() {
    let args = Args::parse();

    let paths = fs::read_dir(args.obsidian_dir.clone()).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        let display = path.display();

        process_blog(
            &args.hugo_path,
            &args.obsidian_dir_imgs,
            &display.to_string(),
            &args.obsidian_dir,
        );
    }
}
