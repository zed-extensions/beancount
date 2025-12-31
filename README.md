# Zed Beancount

This extension adds support for the [Beancount](https://github.com/beancount/beancount) language.

## Setup

By default this extension just provides syntax highlighting for `.beancount` and `.bean` files, but also optional support for [`beancount-language-server`](https://github.com/polarmutex/beancount-language-server) as well.

To use that you will need `beancount` and `beancount-language-server` available in your path. For example, on mac:

```
brew install beancount beancount-language-server
```

With that installed Zed will show Diagnostic information inline and in the Zed Diagnostics Panel like this:

![beancount-zed-extension-screenshot](https://github.com/user-attachments/assets/bd6a3ac8-9196-4954-ab33-4ed969189cfa)

## Configuration

You can configure the beancount-language-server by adding initialization options to either your project settings or global Zed settings:

```json
// Project-specific: .zed/settings.json
// Global: ~/.config/zed/settings.json
{
  "lsp": {
    "beancount": {
      "initialization_options": {
        "journal_file": "/path/to/main.beancount"
      }
    }
  }
}
```

### Custom Binary Path

By default, the extension searches for `beancount-language-server` in your system PATH. If you need to use a custom binary location, you can configure it:

```json
{
  "lsp": {
    "beancount": {
      "binary": {
        "path": "/custom/path/to/beancount-language-server",
        "arguments": ["--debug"],
        "env": {
          "CUSTOM_VAR": "value"
        }
      }
    }
  }
}
```

Configuration options:
- `path`: Absolute path to the beancount-language-server binary (optional, defaults to searching PATH)
- `arguments`: Additional command-line arguments to pass to the language server (optional)
- `env`: Additional environment variables to set for the language server process (optional)
