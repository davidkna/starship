| Option | Default | Description |
| `format` | `"via [$symbol($version )(🎯 $tfm )]($style)"` | The format for the module. |
| `version_format` | `"v${raw}"` | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol` | `".NET "` |  |
| `style` | `"blue bold"` | The style for the module. |
| `heuristic` | `true` |  |
| `disabled` | `false` |  |
| `detect_extensions` | `["csproj", "fsproj", "xproj"]` | Which extensions should trigger this module. |
| `detect_files` | `["global.json", "project.json", "Directory.Build.props", "Directory.Build.targets", "Packages.props"]` | Which filenames should trigger this module. |
| `detect_folders` | `[]` | Which folders should trigger this module. |
