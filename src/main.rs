use clap::Parser;
use std::fs;

use std::fs::read_to_string;

fn process_blog(
    hugo_base_dir: &String,
    obsidan_img_folder: &String,
    filename: &str,
    obsidan_dir: &String,
) {
    fs::create_dir_all("").unwrap();

    let mut count = 0;

    let mut parsed_path = String::new();
    let mut image_base = String::new();

    for line in read_to_string(filename).unwrap().lines() {
        let s = line.to_string();
        // println!("{}",s);

        if s.contains("staticPath:") {
            if let Some(path_part) = s.split(':').nth(1) {
                let path_part_trimmed = path_part.trim(); // Trims any extra whitespace
                parsed_path = path_part_trimmed.to_string(); // Assign the string directly

                if let Some(image_parsed_base) = parsed_path.split("/").nth(2) {
                    image_base = image_parsed_base.to_string();
                }

                let mut static_base = hugo_base_dir.clone();

                static_base.push_str("/static");

                println!("static base {}", static_base);
                fs::create_dir_all(static_base.clone()).unwrap();
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
            dest_path.push_str(&parsed_path);
            dest_path.push_str(&image_base);
            dest_path.push_str(&count.to_string());
            dest_path.push_str(".png");
            
            println!("copied {} to {}",attachment_base,dest_path);

            fs::copy(attachment_base.clone(), dest_path.clone()).unwrap();

            println!(
                "{}",
                "![img](".to_owned() + &parsed_path + &image_base + &count.to_string() + ".png)"
            );
            count += 1;
        } else {
            println!("{}", s);
        }
    }
}

/// Progam to generate hugo markdown from obsidian markdown
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// path to obsidian source markdown
    #[arg( long)]
    obsidian_dir: String,

    ///path to obsidian img folder
    #[arg( long)]
    obsidian_dir_imgs: String,

    /// path to hugo folder
    #[arg( long)]
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
