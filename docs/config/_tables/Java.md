| Option | Default | Description |
| `disabled` | `false` |  |
| `format` | `"via [$symbol($version )]($style)"` | The format for the module. |
| `version_format` | `"v${raw}"` | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style` | `"red dimmed"` | The style for the module. |
| `symbol` | `"☕ "` |  |
| `detect_extensions` | `["java", "class", "jar", "gradle", "clj", "cljc"]` | Which extensions should trigger this module. |
| `detect_files` | `["pom.xml", "build.gradle.kts", "build.sbt", ".java-version", "deps.edn", "project.clj", "build.boot"]` | Which filenames should trigger this module. |
| `detect_folders` | `[]` | Which folders should trigger this module. |
