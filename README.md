# CrabSay 🦀


### Installation

You can install CrabSay 🦀 using Cargo & Git, the following instructions:  
1. Clone the repository
```bash
git clone https://github.com/lordofwizard/crabsay
```
2. Go inside that directory
```bash
cd crabsay
```
3. Install it...
```bash
cargo install --path .
```

### Usage

After installing CrabSay, you can use it directly from the command line with your desired message. The message can be passed as a single string or as multiple words separated by spaces.

Example:
```bash
crabsay "Aurora is lucid"
```

This will produce the following output:
```
________
| Aurora |
| is     |
| lucid  |
--------

           \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
```

### Credits

This project is a Rust rewrite of the classic `cowsay` program, originally written by Tony Monroe. Special thanks to the `colored` crate for providing easy terminal coloring capabilities. You can find more information about the crate on [crates.io](https://crates.io/crates/colored).

### License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

Feel free to contribute to the project or report any issues on [GitHub](https://github.com/lordofwizard/crabsay). Enjoy your time with Ferris the crab! 🦀
