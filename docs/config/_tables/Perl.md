| Option | Default | Description |
| `format` | `"via [$symbol($version )]($style)"` | The format for the module. |
| `version_format` | `"v${raw}"` | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol` | `"🐪 "` |  |
| `style` | `"149 bold"` | The style for the module. |
| `disabled` | `false` |  |
| `detect_extensions` | `["pl", "pm", "pod"]` | Which extensions should trigger this module. |
| `detect_files` | `["Makefile.PL", "Build.PL", "cpanfile", "cpanfile.snapshot", "META.json", "META.yml", ".perl-version"]` | Which filenames should trigger this module. |
| `detect_folders` | `[]` | Which folders should trigger this module. |
