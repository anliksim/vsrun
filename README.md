# vsrun

Visual Studio Code run cli that simplifies extension management.



VSCode currently laks automatic extension management. You can manage recommended and unwated extensions in `.vscode/extensions.json` but there is no way to automatically install and enable/disable.

**_This is a one-time effort, so why use a cli?_**

I like everything to work without any manual steps. In addition, if you work with many different technologies you end up running many extensions you do not really need all the time. Sometimes you even need to disable extensions for one project that you might need for another e.g. if you use take-over mode for vuejs but also work on other typescript projects.

## Usage

> Note: This has been implemented for Windows only so far and was tested with Windows 11. Please open an issue if you want to use it on Mac/Linux or if you are running into any issues.


Get the lastest released binary from the releases and put it on your path, then open your project using the cli.

```
vsrun path/to/project
```

**Prerequisite:** Visual Studio Code needs to be installed on your system. You can test this by running the follow line in your terminal:

```
code --help
```



## Develop

Show logs
```
RUST_LOG=debug cargo run .
```

---

Thanks to [rust-cli](https://github.com/rust-cli) for the comprehensive guide on how to create a CLI in rust.
