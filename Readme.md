

# Usage

```
cargo run -- --obsidian-dir ~/Desktop/vault/Publish --obsidian-dir-imgs ~/Desktop/vault/Imgs/ --hugo-path ~/blag/
   Compiling blogger v0.1.0 (/Users/exit/blogger)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.56s
     Running `target/debug/blogger --obsidian-dir /Users/exit/Desktop/vault/Publish --obsidian-dir-imgs /Users/exit/Desktop/vault/Imgs/ --hugo-path /Users/exit/blag/`
static base /Users/exit/blag//static
copied /Users/exit/Desktop/vault/Imgs/Pasted image 20241127141342.png to /Users/exit/blag/lifeos2.00.png
copied /Users/exit/Desktop/vault/Imgs/Pasted image 20241127141604.png to /Users/exit/blag/lifeos2.01.png
copied /Users/exit/Desktop/vault/Imgs/Pasted image 20241127143642.png to /Users/exit/blag/lifeos2.02.png
```

### Note

Obsidian markdown files must set a path with the the property `staticPath` this path is used for the image path.