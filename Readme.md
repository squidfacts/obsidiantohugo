

# Usage

```
cargo run -- --obsidian-dir ~/Desktop/vault/Publish --obsidian-dir-imgs ~/Desktop/vault/Imgs/ --hugo-path ~/blag/
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/blogger --obsidian-dir /Users/exit/Desktop/vault/Publish --obsidian-dir-imgs /Users/exit/Desktop/vault/Imgs/ --hugo-path /Users/exit/blag/`
created static folder /Users/exit/blag/static/lockbit2
Creating hugo md file /Users/exit/blag/content/posts/lockbit2.md
copied /Users/exit/Desktop/vault/Imgs/Pasted image 20241127141342.png to /Users/exit/blag/static/lockbit2/lockbit20.png
copied /Users/exit/Desktop/vault/Imgs/Pasted image 20241127141604.png to /Users/exit/blag/static/lockbit2/lockbit21.png
copied /Users/exit/Desktop/vault/Imgs/Pasted image 20241127143642.png to /Users/exit/blag/static/lockbit2/lockbit22.png
```

### Example obsidian config

---
staticPath: lockbit2
Title: "Hack the Box: Lockbit 2.0"
Description: Analysis of the Hack the Box 2.0 malware sherlock
Date: 2024-11-27
---


- `staticPath` this path is used for the image path and hugo file name
- `Title` this is the title set in the hugo file
- `Description` this is the description of the page in the hugo file
- `Date` this is the date for the hugo publish date