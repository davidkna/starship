| Option | Default | Description |
| `format` | `"via [$symbol($version )]($style)"` | The format for the module. |
| `version_format` | `"v${raw}"` | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol` | `"🐹 "` |  |
| `style` | `"bold cyan"` | The style for the module. |
| `disabled` | `false` |  |
| `detect_extensions` | `["go"]` | Which extensions should trigger this module. |
| `detect_files` | `["go.mod", "go.sum", "go.work", "glide.yaml", "Gopkg.yml", "Gopkg.lock", ".go-version"]` | Which filenames should trigger this module. |
| `detect_folders` | `["Godeps"]` | Which folders should trigger this module. |
