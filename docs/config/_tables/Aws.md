| Option | Default | Description |
| `format` | `'on [$symbol($profile )(\($region\) )(\[$duration\] )]($style)'` | The format for the module. |
| `symbol` | `"☁️  "` | The symbol used before displaying the current AWS profile. |
| `style` | `"bold yellow"` | The style for the module. |
| `disabled` | `false` | Disables the AWS module. |
| `region_aliases` | `` | Table of region aliases to display in addition to the AWS name. |
| `profile_aliases` | `` | Table of profile aliases to display in addition to the AWS name. |
| `expiration_symbol` | `"X"` | The symbol displayed when the temporary credentials have expired. |
| `force_display` | `false` | If true displays info even if `credentials`, `credential_process` or `sso_start_url` have not been setup. |
