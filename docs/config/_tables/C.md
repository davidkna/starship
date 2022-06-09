| Option | Default | Description |
| `format` | `"via [$symbol($version(-$name) )]($style)"` | The format string for the module. |
| `version_format` | `"v${raw}"` | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style` | `"149 bold"` | The style for the module. |
| `symbol` | `"C "` | The symbol used before displaying the compiler details |
| `disabled` | `false` | Disables the `c` module. |
| `detect_extensions` | `["c", "h"]` | Which extensions should trigger this module. |
| `detect_files` | `[]` | Which filenames should trigger this module. |
| `detect_folders` | `[]` | Which folders should trigger this module. |
| `commands` | `[["cc", "--version"], ["gcc", "--version"], ["clang", "--version"]]` | How to detect what the compiler is |
