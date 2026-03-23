// While adding out new module add out module to src/module.rs ALL_MODULES const array also.
mod aws;
mod azure;
mod buf;
mod bun;
mod c;
mod cc;
mod character;
mod cmake;
mod cmd_duration;
mod cobol;
mod conda;
mod container;
mod cpp;
mod crystal;
pub mod custom;
mod daml;
mod dart;
mod deno;
mod directory;
mod direnv;
mod docker_context;
mod dotnet;
mod elixir;
mod elm;
mod env_var;
mod erlang;
mod fennel;
mod fill;
mod fortran;
mod fossil_branch;
mod fossil_metrics;
mod gcloud;
mod git_branch;
mod git_commit;
mod git_metrics;
mod git_state;
pub(crate) mod git_status;
mod gleam;
mod golang;
mod gradle;
mod guix_shell;
mod haskell;
mod haxe;
mod helm;
mod hg_branch;
mod hg_state;
mod hostname;
mod java;
mod jobs;
mod julia;
mod kotlin;
mod kubernetes;
mod line_break;
mod localip;
mod lua;
mod maven;
mod memory_usage;
mod meson;
mod mise;
mod mojo;
mod nats;
mod netns;
mod nim;
mod nix_shell;
mod nodejs;
mod ocaml;
mod odin;
mod opa;
mod openstack;
mod os;
mod package;
mod perl;
mod php;
mod pijul_channel;
mod pixi;
mod pulumi;
mod purescript;
mod python;
mod quarto;
mod raku;
mod red;
mod rlang;
mod ruby;
mod rust;
mod scala;
mod shell;
mod shlvl;
mod singularity;
mod solidity;
mod spack;
mod status;
mod sudo;
mod swift;
mod terraform;
mod time;
mod username;
mod utils;
mod vagrant;
mod vcs;
mod vcsh;
mod vlang;
mod xmake;
mod zig;

#[cfg(feature = "battery")]
mod battery;
mod typst;

#[cfg(feature = "battery")]
pub use self::battery::{BatteryInfoProvider, BatteryInfoProviderImpl};

use crate::config::ModuleConfig;
use crate::context::{Context, Detected, Shell};
use crate::module::Module;
use std::time::Instant;

fn get_module_name<'a>(instance_name: &'a str, context: &'a Context) -> &'a str {
    context
        .config
        .get_module_config(instance_name)
        .and_then(|c| c.get("module"))
        .and_then(|v| v.as_str())
        .unwrap_or(instance_name)
}

pub fn handle<'a>(instance_name: &str, context: &'a Context) -> Option<Module<'a>> {
    let module_name = get_module_name(instance_name, context);

    let start: Instant = Instant::now();
    let mut m: Option<Module> = {
        match module_name {
            // Keep these ordered alphabetically.
            // Default ordering is handled in configs/starship_root.rs
            "aws" => aws::module(context, Some(instance_name)),
            "azure" => azure::module(context, Some(instance_name)),
            #[cfg(feature = "battery")]
            "battery" => battery::module(context, Some(instance_name)),
            "buf" => buf::module(context, Some(instance_name)),
            "bun" => bun::module(context, Some(instance_name)),
            "c" => c::module(context, Some(instance_name)),
            "character" => character::module(context, Some(instance_name)),
            "cmake" => cmake::module(context, Some(instance_name)),
            "cmd_duration" => cmd_duration::module(context, Some(instance_name)),
            "cobol" => cobol::module(context, Some(instance_name)),
            "conda" => conda::module(context, Some(instance_name)),
            "container" => container::module(context, Some(instance_name)),
            "cpp" => cpp::module(context, Some(instance_name)),
            "daml" => daml::module(context, Some(instance_name)),
            "dart" => dart::module(context, Some(instance_name)),
            "deno" => deno::module(context, Some(instance_name)),
            "directory" => directory::module(context, Some(instance_name)),
            "direnv" => direnv::module(context, Some(instance_name)),
            "docker_context" => docker_context::module(context, Some(instance_name)),
            "dotnet" => dotnet::module(context, Some(instance_name)),
            "elixir" => elixir::module(context, Some(instance_name)),
            "elm" => elm::module(context, Some(instance_name)),
            "erlang" => erlang::module(context, Some(instance_name)),
            "env_var" => env_var::module(None, context),
            "fennel" => fennel::module(context, Some(instance_name)),
            "fill" => fill::module(context, Some(instance_name)),
            "fortran" => fortran::module(context, Some(instance_name)),
            "fossil_branch" => fossil_branch::module(context, Some(instance_name)),
            "fossil_metrics" => fossil_metrics::module(context, Some(instance_name)),
            "gcloud" => gcloud::module(context, Some(instance_name)),
            "git_branch" => git_branch::module(context, Some(instance_name)),
            "git_commit" => git_commit::module(context, Some(instance_name)),
            "git_metrics" => git_metrics::module(context, Some(instance_name)),
            "git_state" => git_state::module(context, Some(instance_name)),
            "git_status" => git_status::module(context, Some(instance_name)),
            "gleam" => gleam::module(context, Some(instance_name)),
            "golang" => golang::module(context, Some(instance_name)),
            "gradle" => gradle::module(context, Some(instance_name)),
            "guix_shell" => guix_shell::module(context, Some(instance_name)),
            "haskell" => haskell::module(context, Some(instance_name)),
            "haxe" => haxe::module(context, Some(instance_name)),
            "helm" => helm::module(context, Some(instance_name)),
            "hg_branch" => hg_branch::module(context, Some(instance_name)),
            "hg_state" => hg_state::module(context, Some(instance_name)),
            "hostname" => hostname::module(context, Some(instance_name)),
            "java" => java::module(context, Some(instance_name)),
            "jobs" => jobs::module(context, Some(instance_name)),
            "julia" => julia::module(context, Some(instance_name)),
            "kotlin" => kotlin::module(context, Some(instance_name)),
            "kubernetes" => kubernetes::module(context, Some(instance_name)),
            "line_break" => line_break::module(context, Some(instance_name)),
            "localip" => localip::module(context, Some(instance_name)),
            "lua" => lua::module(context, Some(instance_name)),
            "maven" => maven::module(context, Some(instance_name)),
            "memory_usage" => memory_usage::module(context, Some(instance_name)),
            "meson" => meson::module(context, Some(instance_name)),
            "mise" => mise::module(context, Some(instance_name)),
            "mojo" => mojo::module(context, Some(instance_name)),
            "nats" => nats::module(context, Some(instance_name)),
            "netns" => netns::module(context, Some(instance_name)),
            "nim" => nim::module(context, Some(instance_name)),
            "nix_shell" => nix_shell::module(context, Some(instance_name)),
            "nodejs" => nodejs::module(context, Some(instance_name)),
            "ocaml" => ocaml::module(context, Some(instance_name)),
            "odin" => odin::module(context, Some(instance_name)),
            "opa" => opa::module(context, Some(instance_name)),
            "openstack" => openstack::module(context, Some(instance_name)),
            "os" => os::module(context, Some(instance_name)),
            "package" => package::module(context, Some(instance_name)),
            "perl" => perl::module(context, Some(instance_name)),
            "php" => php::module(context, Some(instance_name)),
            "pijul_channel" => pijul_channel::module(context, Some(instance_name)),
            "pixi" => pixi::module(context, Some(instance_name)),
            "pulumi" => pulumi::module(context, Some(instance_name)),
            "purescript" => purescript::module(context, Some(instance_name)),
            "python" => python::module(context, Some(instance_name)),
            "quarto" => quarto::module(context, Some(instance_name)),
            "raku" => raku::module(context, Some(instance_name)),
            "rlang" => rlang::module(context, Some(instance_name)),
            "red" => red::module(context, Some(instance_name)),
            "ruby" => ruby::module(context, Some(instance_name)),
            "rust" => rust::module(context, Some(instance_name)),
            "scala" => scala::module(context, Some(instance_name)),
            "shell" => shell::module(context, Some(instance_name)),
            "shlvl" => shlvl::module(context, Some(instance_name)),
            "singularity" => singularity::module(context, Some(instance_name)),
            "solidity" => solidity::module(context, Some(instance_name)),
            "spack" => spack::module(context, Some(instance_name)),
            "swift" => swift::module(context, Some(instance_name)),
            "status" => status::module(context, Some(instance_name)),
            "sudo" => sudo::module(context, Some(instance_name)),
            "terraform" => terraform::module(context, Some(instance_name)),
            "time" => time::module(context, Some(instance_name)),
            "typst" => typst::module(context, Some(instance_name)),
            "crystal" => crystal::module(context, Some(instance_name)),
            "username" => username::module(context, Some(instance_name)),
            "vlang" => vlang::module(context, Some(instance_name)),
            "vagrant" => vagrant::module(context, Some(instance_name)),
            "vcs" => vcs::module(context, Some(instance_name)),
            "vcsh" => vcsh::module(context, Some(instance_name)),
            "xmake" => xmake::module(context, Some(instance_name)),
            "zig" => zig::module(context, Some(instance_name)),
            env if env.starts_with("env_var.") => {
                env_var::module(env.strip_prefix("env_var."), context)
            }
            custom if custom.starts_with("custom.") => {
                // SAFETY: We just checked that the module starts with "custom."
                custom::module(custom.strip_prefix("custom.").unwrap(), context)
            }
            _ => {
                eprintln!(
                    "Error: Unknown module {instance_name}. Use starship module --list to list out all supported modules."
                );
                None
            }
        }
    };

    let elapsed = start.elapsed();
    log::trace!("Took {elapsed:?} to compute module {instance_name:?}");
    if elapsed.as_millis() >= 1 {
        // If we take less than 1ms to compute a None, then we will not return a module at all
        // if we have a module: default duration is 0 so no need to change it
        // if we took more than 1ms we want to report that and so--in case we have None currently--
        // need to create an empty module just to hold the duration for that case
        m.get_or_insert_with(|| context.new_module(module_name, Some(instance_name)))
            .duration = elapsed;
    }
    m
}

pub fn description(module: &str) -> &'static str {
    match module {
        "aws" => "The current AWS region and profile",
        "azure" => "The current Azure subscription",
        "battery" => "The current charge of the device's battery and its current charging status",
        "buf" => "The currently installed version of the Buf CLI",
        "bun" => "The currently installed version of the Bun",
        "c" => "Your C compiler type",
        "character" => {
            "A character (usually an arrow) beside where the text is entered in your terminal"
        }
        "cmake" => "The currently installed version of CMake",
        "cmd_duration" => "How long the last command took to execute",
        "cobol" => "The currently installed version of COBOL/GNUCOBOL",
        "conda" => "The current conda environment, if $CONDA_DEFAULT_ENV is set",
        "container" => "The container indicator, if inside a container.",
        "cpp" => "your cpp compiler type",
        "crystal" => "The currently installed version of Crystal",
        "daml" => "The Daml SDK version of your project",
        "dart" => "The currently installed version of Dart",
        "deno" => "The currently installed version of Deno",
        "directory" => "The current working directory",
        "direnv" => "The currently applied direnv file",
        "docker_context" => "The current docker context",
        "dotnet" => "The relevant version of the .NET Core SDK for the current directory",
        "elixir" => "The currently installed versions of Elixir and OTP",
        "elm" => "The currently installed version of Elm",
        "erlang" => "Current OTP version",
        "fennel" => "The currently installed version of Fennel",
        "fill" => "Fills the remaining space on the line with a pad string",
        "fortran" => "The currently used version of Fortran",
        "fossil_branch" => "The active branch of the check-out in your current directory",
        "fossil_metrics" => "The currently added/deleted lines in your check-out",
        "gcloud" => "The current GCP client configuration",
        "git_branch" => "The active branch of the repo in your current directory",
        "git_commit" => "The active commit (and tag if any) of the repo in your current directory",
        "git_metrics" => "The currently added/deleted lines in your repo",
        "git_state" => "The current git operation, and it's progress",
        "git_status" => "Symbol representing the state of the repo",
        "gleam" => "The currently installed version of Gleam",
        "golang" => "The currently installed version of Golang",
        "gradle" => "The currently installed version of Gradle",
        "guix_shell" => "The guix-shell environment",
        "haskell" => "The selected version of the Haskell toolchain",
        "haxe" => "The currently installed version of Haxe",
        "helm" => "The currently installed version of Helm",
        "hg_branch" => "The active branch and topic of the repo in your current directory",
        "hg_state" => "The current hg operation",
        "hostname" => "The system hostname",
        "java" => "The currently installed version of Java",
        "jobs" => "The current number of jobs running",
        "julia" => "The currently installed version of Julia",
        "kotlin" => "The currently installed version of Kotlin",
        "kubernetes" => "The current Kubernetes context name and, if set, the namespace",
        "line_break" => "Separates the prompt into two lines",
        "localip" => "The currently assigned ipv4 address",
        "lua" => "The currently installed version of Lua",
        "maven" => "The Maven Wrapper version of the current project",
        "memory_usage" => "Current system memory and swap usage",
        "meson" => {
            "The current Meson environment, if $MESON_DEVENV and $MESON_PROJECT_NAME are set"
        }
        "mise" => "The current mise status",
        "mojo" => "The currently installed version of Mojo",
        "nats" => "The current NATS context",
        "netns" => "The current network namespace",
        "nim" => "The currently installed version of Nim",
        "nix_shell" => "The nix-shell environment",
        "nodejs" => "The currently installed version of NodeJS",
        "ocaml" => "The currently installed version of OCaml",
        "odin" => "The currently installed version of Odin",
        "opa" => "The currently installed version of Open Platform Agent",
        "openstack" => "The current OpenStack cloud and project",
        "os" => "The current operating system",
        "package" => "The package version of the current directory's project",
        "perl" => "The currently installed version of Perl",
        "php" => "The currently installed version of PHP",
        "pijul_channel" => "The current channel of the repo in the current directory",
        "pixi" => {
            "The currently installed version of Pixi, and the active environment if $PIXI_ENVIRONMENT_NAME is set"
        }
        "pulumi" => "The current username, stack, and installed version of Pulumi",
        "purescript" => "The currently installed version of PureScript",
        "python" => "The currently installed version of Python",
        "quarto" => "The current installed version of quarto",
        "raku" => "The currently installed version of Raku",
        "red" => "The currently installed version of Red",
        "rlang" => "The currently installed version of R",
        "ruby" => "The currently installed version of Ruby",
        "rust" => "The currently installed version of Rust",
        "scala" => "The currently installed version of Scala",
        "shell" => "The currently used shell indicator",
        "shlvl" => "The current value of SHLVL",
        "singularity" => "The currently used Singularity image",
        "solidity" => "The current installed version of Solidity",
        "spack" => "The current spack environment, if $SPACK_ENV is set",
        "status" => "The status of the last command",
        "sudo" => "The sudo credentials are currently cached",
        "swift" => "The currently installed version of Swift",
        "terraform" => "The currently selected terraform workspace and version",
        "time" => "The current local time",
        "typst" => "The current installed version of typst",
        "username" => "The active user's username",
        "vagrant" => "The currently installed version of Vagrant",
        "vcs" => "The currently active VCS repository (first one matching)",
        "vcsh" => "The currently active VCSH repository",
        "vlang" => "The currently installed version of V",
        "xmake" => "The currently installed version of XMake",
        "zig" => "The currently installed version of Zig",
        _ => "<no description>",
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::module::ALL_MODULES;

    #[test]
    fn all_modules_have_description() {
        for module in ALL_MODULES {
            println!("Checking if {module:?} has a description");
            assert_ne!(description(module), "<no description>");
        }
    }

    #[test]
    fn test_alias_module() {
        use crate::test::ModuleRenderer;
        use nu_ansi_term::Color;

        let actual = ModuleRenderer::new("work_aws")
            .config(toml::toml! {
                [work_aws]
                module = "aws"
                symbol = "WorkAWS "
                [work_aws.profile_aliases]
                work-profile = "work"
            })
            .env("AWS_PROFILE", "work-profile")
            .env("AWS_ACCESS_KEY_ID", "dummy")
            .collect();

        let expected = Some(format!(
            "on {}",
            Color::Yellow.bold().paint("WorkAWS work ")
        ));

        assert_eq!(expected, actual);
    }
}
