| Option | Default | Description |
| `format` | `"via [$symbol($version )]($style)"` | The format for the module. |
| `version_format` | `"v${raw}"` | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `disabled` | `false` |  |
| `style` | `"red bold"` | The style for the module. |
| `symbol` | `"🆂 "` |  |
| `detect_extensions` | `["sbt", "scala"]` | Which extensions should trigger this module. |
| `detect_files` | `[".scalaenv", ".sbtenv", "build.sbt"]` | Which filenames should trigger this module. |
| `detect_folders` | `[".metals"]` | Which folders should trigger this module. |
