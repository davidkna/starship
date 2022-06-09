| Option | Default | Description |
| `format` | `"via [$symbol($version )]($style)"` | The format for the module. |
| `version_format` | `"v${raw}"` | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol` | `"λ "` |  |
| `style` | `"bold purple"` | The style for the module. |
| `disabled` | `false` |  |
| `detect_extensions` | `["hs", "cabal", "hs-boot"]` | Which extensions should trigger this module. |
| `detect_files` | `["stack.yaml", "cabal.project"]` | Which filenames should trigger this module. |
| `detect_folders` | `[]` | Which folders should trigger this module. |
