| Option | Default | Description |
| `format` | `'via [$symbol($version )(\($switch_indicator$switch_name\) )]($style)'` | The format for the module. |
| `version_format` | `"v${raw}"` | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `global_switch_indicator` | `""` |  |
| `local_switch_indicator` | `"*"` |  |
| `symbol` | `"🐫 "` |  |
| `style` | `"bold yellow"` | The style for the module. |
| `disabled` | `false` |  |
| `detect_extensions` | `["opam", "ml", "mli", "re", "rei"]` | Which extensions should trigger this module. |
| `detect_files` | `["dune", "dune-project", "jbuild", "jbuild-ignore", ".merlin"]` | Which filenames should trigger this module. |
| `detect_folders` | `["_opam", "esy.lock"]` | Which folders should trigger this module. |
