| Option | Default | Description |
| `format` | `"via [$symbol($version )]($style)"` | The format for the module. |
| `version_format` | `"v${raw}"` | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol` | `"🌳 "` |  |
| `style` | `"cyan bold"` | The style for the module. |
| `disabled` | `false` |  |
| `detect_extensions` | `["elm"]` | Which extensions should trigger this module. |
| `detect_files` | `["elm.json", "elm-package.json", ".elm-version"]` | Which filenames should trigger this module. |
| `detect_folders` | `["elm-stuff"]` | Which folders should trigger this module. |
