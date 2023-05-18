# vsrun

<p align="center">
  <img width="150" src="docs/logo.svg">
</p>
<p align="center">Visual Studio Code run cli that simplifies extension management.</p>
<br/>

VSCode currently laks automatic extension management. You can manage recommended and unwated extensions in `.vscode/extensions.json` but there is no way to automatically install and enable/disable.

**_This is a one-time effort, so why use a cli?_**

I like everything to work without any manual steps. In addition, if you work with many different technologies you end up running many extensions you do not really need all the time. Sometimes you even need to disable extensions for one project that you might need for another e.g. if you use take-over mode for vuejs but also work on other typescript projects.

## Usage

> Note: This has been implemented for Windows only so far and was tested with Windows 11. Please open an issue if you want to use it on Mac/Linux or if you are running into any issues.


Get the lastest released binary from the releases and put it on your path, then open your project using the cli. Both absolute and relative paths are supported as `path/to/project`.

```
vsrun path/to/project
```

**Prerequisite:** Visual Studio Code needs to be installed on your system. You can test this by running the follow line in your terminal:

```
code --help
```

When opening a project, the cli will read your `.vscode/extensions.json` file, install the recommendations and disable all other extensions installed. Additionally, all unwanted recommendations are disabled. This allows you to also disable built-in extensions.

A valid `extensions.json` file is required, e.g.:
```json
{
    "recommendations": [
        "vadimcn.vscode-lldb",
        "mutantdino.resourcemonitor",
        "rust-lang.rust-analyzer",
        "tamasfe.even-better-toml",
        "serayuzgur.crates"
    ],
    "unwantedRecommendations": [
        "vscode.typescript-language-features",
        "vscode.github",
        "vscode.github-authentication"
    ]
}
```

If not `extensions.json` is present, an empty fallback is used:
```json
{
    "recommendations": [],
    "unwantedRecommendations": []
}
```


## Develop

Show logs
```
RUST_LOG=debug cargo run .
```

---

Thanks to [rust-cli](https://github.com/rust-cli) for the comprehensive guide on how to create a CLI in rust.
