| Option | Default | Description |
| `format` | `"via [$symbol$workspace]($style) "` | The format for the module. |
| `version_format` | `"v${raw}"` | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol` | `"💠 "` |  |
| `style` | `"bold 105"` | The style for the module. |
| `disabled` | `false` |  |
| `detect_extensions` | `["tf", "tfplan", "tfstate"]` | Which extensions should trigger this module. |
| `detect_files` | `[]` | Which filenames should trigger this module. |
| `detect_folders` | `[".terraform"]` | Which folders should trigger this module. |
