use clap::Parser;
use std::fs;

use std::fs::read_to_string;

fn process_blog(hugo_base_dir: &String, filename: &str) {
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

                let mut static_base = hugo_base_dir + "/static";
                static_base.push_str(&parsed_path);

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

            let mut attachment_base = "/Users/emma-training/vred/Img/".to_owned();
            attachment_base.push_str(imagename);

            let mut dest_path = BLOG_STATIC_BASE.to_string();

            dest_path.push_str(&parsed_path);
            dest_path.push_str(&image_base);
            dest_path.push_str(&count.to_string());
            dest_path.push_str(".png");

            //println!("copied {} to {}",attachment_base,dest_path);

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
    #[arg(short, long)]
    obisdian_dir: String,

    ///path to obsidian img folder
    #[arg(short, long)]
    obisdian_dir_imgs: String,

    /// path to hugo folder
    #[arg(short, long)]
    hugo_path: String,
}

fn main() {
    let args = Args::parse();

    let paths = fs::read_dir(args.obisdian_dir).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        let display = path.display();

        process_blog(&args.hugo_path, &display.to_string());
    }
}
