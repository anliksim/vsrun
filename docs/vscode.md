# vscode

## CLI Options

Disabling all installed extensions and disabling a built-in extension in one go does not work. The second command is ignored.

```
code --disable-extensions --disable-extension vscode.typescript-language-features
```

Disabling all extensions by name would work i.e. list all extensions and then disable them one by one.

```
    code \
        --disable-extension vscode.php-language-features \
        --disable-extension vscode.typescript-language-features \
```

It looks like installation cannot be chained together with disables so it has to be executed as a separate command. All in all this could work:

```
code --install-extension vue.volar &&
    code \
        --disable-extension vscode.php-language-features \
        --disable-extension vscode.typescript-language-features \
        ~/dev/src/$1
```

With some more built-in ones to disable

```
code --install-extension vue.volar &&
    code \
        --disable-extension vscode.github-authentication \
        --disable-extension vscode.git \
        --disable-extension vscode.jake \
        --disable-extension ms-vscode.node-debug \
        --disable-extension vscode.php-language-features \
        --disable-extension vscode.typescript-language-features \
        ~/dev/src/$1
```
