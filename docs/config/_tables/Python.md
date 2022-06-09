| Option | Default | Description |
| `pyenv_version_name` | `false` |  |
| `pyenv_prefix` | `"pyenv "` |  |
| `python_binary` | `["python", "python3", "python2"]` |  |
| `format` | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'` | The format for the module. |
| `version_format` | `"v${raw}"` | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style` | `"yellow bold"` | The style for the module. |
| `symbol` | `"🐍 "` |  |
| `disabled` | `false` |  |
| `detect_extensions` | `["py"]` | Which extensions should trigger this module. |
| `detect_files` | `["requirements.txt", ".python-version", "pyproject.toml", "Pipfile", "tox.ini", "setup.py", "__init__.py"]` | Which filenames should trigger this module. |
| `detect_folders` | `[]` | Which folders should trigger this module. |
