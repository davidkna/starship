| Option | Default | Description |
| `format` | `"with [$symbol ($version)]($style)"` | The format for the `buf` module. |
| `version_format` | `"v${raw}"` | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol` | `""` | The symbol used before displaying the version of Buf. |
| `style` | `"bold blue"` | The style for the module. |
| `disabled` | `false` | Disables the `elixir` module. |
| `detect_extensions` | `[]` | Which extensions should trigger this module. |
| `detect_files` | `["buf.yaml", "buf.gen.yaml", "buf.work.yaml"]` | Which filenames should trigger this module. |
| `detect_folders` | `[]` | Which folders should trigger this module. |
