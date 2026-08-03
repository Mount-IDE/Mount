<p align="center">
    <img src="./public/icon.svg" width="200px">
</p>

<div align="center">

<img alt="Static Badge" src="https://img.shields.io/badge/License-MIT-blue?style=for-the-badge">
<img alt="Static Badge" src="https://img.shields.io/badge/Platform-Windows%2FLinux-green?style=for-the-badge">
<img alt="GitHub Release" src="https://img.shields.io/github/v/release/Mount-IDE/Mount?style=for-the-badge">
<img alt="Static Badge" src="https://img.shields.io/badge/Status-Active_Development-green?style=for-the-badge">

</div>
<div align="center"><h1>Mount IDE</h1></div>

An experimental modular integrated development environment where everything needed for development – compilers, LSP,
formatters, debuggers – is managed by a single package system.

<img src="./docs/images/1.png">

# Why Mount

Many code development systems are monolithic and support development specifically in a single area. Even if the opposite
is true, such a program requires extensive configuration and remains very limited nonetheless. Additionally, many
development programs continuously log user actions. This creates an unnecessary additional load on the system.
Mount was created to address these issues.



# Technologies

<div align="center" style="display: flex; justify-content: center">
    <div >
        <img src="https://skillicons.dev/icons?i=tauri">
        <p style="font-weight: 500;">Tauri</p>
    </div>
    <div>
        <img src="https://skillicons.dev/icons?i=rust">
        <p>Rust</p>
    </div>
    <div>
        <img src="https://skillicons.dev/icons?i=react">
        <p>React</p>
    </div>
    <div>
        <img src="https://skillicons.dev/icons?i=ts">
        <p>Ts</p>
    </div>
</div>



# How to install

1. Clone repository
```bash
git clone https://github.com/Mount-IDE/Mount.git
cd Mount
```
2. Install dependencies
```bash
npm i
```
3. Compile project
```bash
# dev 
npm run tauri
# release
npm run tauri:release
```

4. Run
```bash
cd src-tauri/target/release
./mount
```

# Roadmap

- [x] Project Creation Menu
- [ ] Project Filtering
- [x] View Project Files
- [x] Terminal
- [x] Project Settings
- [x] Global Settings
- [x] Launch Configurations
- [ ] Package Manager
- [ ] Process Monitor
- [ ] Plugin System

# License

The project is licensed under the [MIT License](./LICENSE)
