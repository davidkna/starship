use super::{Context, Module, ModuleConfig};
use gix::commit::describe::SelectRef::AllTags;

use crate::configs::git_tag::GitTagConfig;
use crate::context::GitRepo;
use crate::formatter::StringFormatter;

/// Creates a module with the Git tag in the current directory.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("git_tag");
    let config: GitTagConfig = GitTagConfig::try_load(module.config);
    if config.disabled {
        return None;
    }
    let tag = git_tag(context.get_git_repo().ok()?, &config)?;

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "tag" => Some(Ok(&tag)),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `git_tag`:\n{error}");
            return None;
        }
    });
    Some(module)
}

fn git_tag(repo: &GitRepo, config: &GitTagConfig) -> Option<String> {
    let mut git_repo = repo.open();
    // Increase the default object cache size to speed up operation for some repos
    git_repo.object_cache_size_if_unset(4 * 1024 * 1024);
    if config.only_detached && !git_repo.head().ok()?.is_detached() {
        return None;
    }
    let head_commit = git_repo.head_commit().ok()?;

    let describe_platform = head_commit
        .describe()
        .names(AllTags)
        .max_candidates(config.max_candidates)
        .traverse_first_parent(true);
    let formatter = describe_platform.try_format().ok()??;

    Some(formatter.name?.to_string())
}

#[cfg(test)]
mod tests {
    use crate::test::{FixtureProvider, ModuleRenderer, fixture_repo};
    use crate::utils::create_command;
    use nu_ansi_term::Color;
    use std::{io, str};

    // TODO: Support reftable and switch to `crate::test::COMMON_GIT_PROVIDERS`.
    static COMMON_GIT_PROVIDERS: &[FixtureProvider] = &[FixtureProvider::Git {
        bare: false,
        reftable: false,
    }];

    #[test]
    fn hidden_outside_repo() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        assert_eq!(
            ModuleRenderer::new("git_tag")
                .config(toml::toml! {
                    [git_tag]
                    disabled = false
                })
                .path(dir.path())
                .collect(),
            None
        );
        Ok(())
    }

    #[test]
    fn tag_visibility_and_candidates() -> io::Result<()> {
        for &provider in COMMON_GIT_PROVIDERS {
            let dir = fixture_repo(provider)?;
            let render = |config| {
                ModuleRenderer::new("git_tag")
                    .config(config)
                    .path(dir.path())
                    .collect()
            };
            let enabled = toml::toml! { [git_tag]
                disabled = false
            };
            assert_eq!(render(enabled.clone()), None);
            assert!(
                create_command("git")?
                    .args(["tag", "ancestor", "HEAD~1"])
                    .current_dir(dir.path())
                    .status()?
                    .success()
            );
            assert_eq!(render(enabled.clone()), None);
            assert_eq!(
                render(toml::toml! { [git_tag]
                    disabled = false
                    max_candidates = 1
                    symbol = "tag:"
                    format = "[$symbol$tag]($style)"
                    style = "red"
                }),
                Some(Color::Red.paint("tag:ancestor").to_string())
            );
            assert!(
                create_command("git")?
                    .args(["tag", "current"])
                    .current_dir(dir.path())
                    .status()?
                    .success()
            );
            assert_eq!(render(toml::Table::new()), None);
            assert_eq!(
                render(toml::toml! { [git_tag]
                    disabled = true
                }),
                None
            );
            assert_eq!(
                render(enabled),
                Some(format!("{} ", Color::Green.bold().paint("🏷  current")))
            );
        }
        Ok(())
    }

    #[test]
    fn only_detached_hides_tags_on_branches() -> io::Result<()> {
        for &provider in COMMON_GIT_PROVIDERS {
            let dir = fixture_repo(provider)?;
            assert!(
                create_command("git")?
                    .args(["tag", "v1"])
                    .current_dir(dir.path())
                    .status()?
                    .success()
            );
            let render = || {
                ModuleRenderer::new("git_tag")
                    .config(toml::toml! {
                        [git_tag]
                        disabled = false
                        only_detached = true
                    })
                    .path(dir.path())
                    .collect()
            };
            assert_eq!(render(), None);
            assert!(
                create_command("git")?
                    .args(["checkout", "--detach", "HEAD"])
                    .current_dir(dir.path())
                    .output()?
                    .status
                    .success()
            );
            assert_eq!(
                render(),
                Some(format!("{} ", Color::Green.bold().paint("🏷  v1")))
            );
        }
        Ok(())
    }

    #[test]
    fn test_render_tag_on_branch() -> io::Result<()> {
        for &provider in COMMON_GIT_PROVIDERS {
            let repo_dir = fixture_repo(provider)?;

            create_command("git")?
                .args(["tag", "v1", "-m", "Testing tags"])
                .current_dir(repo_dir.path())
                .output()?;

            let git_tag = create_command("git")?
                .args(["describe", "--tags", "--exact-match", "HEAD"])
                .current_dir(repo_dir.path())
                .output()?
                .stdout;
            let tag_output = str::from_utf8(&git_tag).unwrap().trim();

            let expected_output = format!("🏷  {tag_output}");

            let actual = ModuleRenderer::new("git_tag")
                .config(toml::toml! {
                    [git_tag]
                        disabled = false
                })
                .path(repo_dir.path())
                .collect();

            let expected = Some(format!("{} ", Color::Green.bold().paint(expected_output)));

            assert_eq!(expected, actual);
            repo_dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn test_render_tag_on_detached_head() -> io::Result<()> {
        for &provider in COMMON_GIT_PROVIDERS {
            let repo_dir = fixture_repo(provider)?;

            create_command("git")?
                .args(["checkout", "@~1"])
                .current_dir(repo_dir.path())
                .output()?;

            create_command("git")?
                .args(["tag", "tagOnDetached", "-m", "Testing tags on detached"])
                .current_dir(repo_dir.path())
                .output()?;

            let git_tag = create_command("git")?
                .args(["describe", "--tags", "--exact-match", "HEAD"])
                .current_dir(repo_dir.path())
                .output()?
                .stdout;
            let tag_output = str::from_utf8(&git_tag).unwrap().trim();

            let expected_output = format!("🏷  {tag_output}");

            let actual = ModuleRenderer::new("git_tag")
                .config(toml::toml! {
                    [git_tag]
                        disabled = false
                })
                .path(repo_dir.path())
                .collect();

            let expected = Some(format!("{} ", Color::Green.bold().paint(expected_output)));

            assert_eq!(expected, actual);
            repo_dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn test_annotated_tag_selection() -> io::Result<()> {
        for &provider in COMMON_GIT_PROVIDERS {
            let repo_dir = fixture_repo(provider)?;

            create_command("git")?
                .args(["tag", "v2", "-m", "Testing tags v2"])
                .env("GIT_COMMITTER_DATE", "2022-01-01 00:00:00 +0000")
                .current_dir(repo_dir.path())
                .output()?;

            create_command("git")?
                .args(["tag", "v0", "-m", "Testing tags v0", "HEAD~1"])
                .env("GIT_COMMITTER_DATE", "2022-01-01 00:00:01 +0000")
                .current_dir(repo_dir.path())
                .output()?;

            create_command("git")?
                .args(["tag", "v1", "-m", "Testing tags v1"])
                .env("GIT_COMMITTER_DATE", "2022-01-01 00:00:01 +0000")
                .current_dir(repo_dir.path())
                .output()?;

            // Annotated tags are preferred over lightweight tags
            create_command("git")?
                .args(["tag", "l0"])
                .env("GIT_COMMITTER_DATE", "2022-01-01 00:00:02 +0000")
                .current_dir(repo_dir.path())
                .output()?;

            let git_tag = create_command("git")?
                .args(["describe", "--tags"])
                .current_dir(repo_dir.path())
                .output()?
                .stdout;
            let tag_output = str::from_utf8(&git_tag).unwrap().trim();

            let expected_output = format!("🏷  {tag_output}");

            let actual = ModuleRenderer::new("git_tag")
                .config(toml::toml! {
                    [git_tag]
                        disabled = false
                })
                .path(repo_dir.path())
                .collect();

            let expected = Some(format!("{} ", Color::Green.bold().paint(expected_output)));

            assert_eq!(expected, actual);
            repo_dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn test_lightweight_tag_selection() -> io::Result<()> {
        for &provider in COMMON_GIT_PROVIDERS {
            let repo_dir = fixture_repo(provider)?;

            // Lightweight tags are chosen lexicographically
            create_command("git")?
                .args(["tag", "v1"])
                .env("GIT_COMMITTER_DATE", "2022-01-01 00:00:00 +0000")
                .current_dir(repo_dir.path())
                .output()?;

            create_command("git")?
                .args(["tag", "v0", "HEAD~1"])
                .env("GIT_COMMITTER_DATE", "2022-01-01 00:00:01 +0000")
                .current_dir(repo_dir.path())
                .output()?;

            create_command("git")?
                .args(["tag", "v2"])
                .env("GIT_COMMITTER_DATE", "2022-01-01 00:00:01 +0000")
                .current_dir(repo_dir.path())
                .output()?;

            let git_tag = create_command("git")?
                .args(["describe", "--tags"])
                .current_dir(repo_dir.path())
                .output()?
                .stdout;
            let tag_output = str::from_utf8(&git_tag).unwrap().trim();

            let expected_output = format!("🏷  {tag_output}");

            let actual = ModuleRenderer::new("git_tag")
                .config(toml::toml! {
                    [git_tag]
                        disabled = false
                })
                .path(repo_dir.path())
                .collect();

            let expected = Some(format!("{} ", Color::Green.bold().paint(expected_output)));

            assert_eq!(expected, actual);
            repo_dir.close()?;
        }
        Ok(())
    }
}
