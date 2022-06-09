| Option | Default | Description |
| `format` | `"via [$symbol($version )]($style)"` | The format for the module. |
| `version_format` | `"v${raw}"` | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol` | `"V "` |  |
| `style` | `"blue bold"` | The style for the module. |
| `disabled` | `false` |  |
| `detect_extensions` | `["v"]` | Which extensions should trigger this module. |
| `detect_files` | `["v.mod", "vpkg.json", ".vpkg-lock.json"]` | Which filenames should trigger this module. |
| `detect_folders` | `[]` | Which folders should trigger this module. |
