| Option | Default | Description |
| `format` | `"via [$symbol($version )]($style)"` | The format for the module. |
| `version_format` | `"v${raw}"` | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol` | `"🦕 "` |  |
| `style` | `"green bold"` | The style for the module. |
| `disabled` | `false` |  |
| `detect_extensions` | `[]` | Which extensions should trigger this module. |
| `detect_files` | `["deno.json", "deno.jsonc", "mod.ts", "deps.ts", "mod.js", "deps.js"]` | Which filenames should trigger this module. |
| `detect_folders` | `[]` | Which folders should trigger this module. |
