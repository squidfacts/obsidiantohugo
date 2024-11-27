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

    let mut file: Option<fs::File> = None;
    let mut write = false;
    for line in read_to_string(filename).unwrap().lines() {
        let s = line.to_string();
        // println!("{}",s);

        if s.contains("staticPath:") {
            if let Some(path_part) = s.split(':').nth(1) {
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
        if s.contains("---") {
            if !file.is_none() {
                write = true;
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
    file.write((str  + "\n").as_bytes()).unwrap();
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
