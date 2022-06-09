| Option | Default | Description |
| `format` | `"via [$symbol($version )]($style)"` | The format for the module. |
| `version_format` | `"v${raw}"` | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol` | `" "` |  |
| `style` | `"bold green"` | The style for the module. |
| `disabled` | `false` |  |
| `not_capable_style` | `"bold red"` |  |
| `detect_extensions` | `["js", "mjs", "cjs", "ts", "mts", "cts"]` | Which extensions should trigger this module. |
| `detect_files` | `["package.json", ".node-version", ".nvmrc"]` | Which filenames should trigger this module. |
| `detect_folders` | `["node_modules"]` | Which folders should trigger this module. |
