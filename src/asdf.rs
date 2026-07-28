use std::io;
use std::process::Command;

#[derive(Debug)]
pub enum AsdfCommands {
    // ==========================
    // MANAGE PLUGINS
    // ==========================
    /// Add a plugin from the plugin repository or a Git repository.
    /// asdf plugin add <name> [<git-url>]
    PluginAdd(String, Option<String>),

    /// List installed plugins.
    /// asdf plugin list [--urls] [--refs]
    PluginList { urls: bool, refs: bool },

    /// List all plugins available in the asdf plugin repository.
    /// asdf plugin list all
    PluginListAll,

    /// Remove a plugin.
    /// asdf plugin remove <name>
    PluginRemove(String),

    /// Update a plugin to the latest commit on the default branch or a specific git reference.
    /// asdf plugin update <name> [<git-ref>]
    PluginUpdate(String, Option<String>),

    /// Update all plugins.
    /// asdf plugin update --all
    PluginUpdateAll,

    // ==========================
    // MANAGE TOOLS
    // ==========================
    /// Display the current version for all installed tools.
    /// asdf current
    Current,

    /// Display the current version for a specific tool.
    /// asdf current <name>
    CurrentTool(String),

    /// Show documentation for a plugin or tool version.
    /// asdf help <name> [<version>]
    Help(String, Option<String>),

    /// Install all versions specified in the .tool-versions file.
    /// asdf install
    Install,

    /// Install the version specified for a tool in the .tool-versions file.
    /// asdf install <name>
    InstallTool(String),

    /// Install a specific version of a tool.
    /// asdf install <name> <version>
    InstallVersion(String, String),

    /// Install the latest stable version, optionally constrained by a version prefix.
    /// asdf install <name> latest[:<version>]
    InstallLatest(String, Option<String>),

    /// Show the latest stable version of a tool.
    /// asdf latest <name> [<version>]
    Latest(String, Option<String>),

    /// Show the latest stable versions for all tools.
    /// asdf latest --all
    LatestAll,

    /// List installed versions of a tool.
    /// asdf list <name> [<version>]
    List(String, Option<String>),

    /// List all available versions of a tool.
    /// asdf list all <name> [<version>]
    ListAll(String, Option<String>),

    /// Set the version of a tool in a .tool-versions file.
    /// asdf set [-u] [-p] <name> <versions...>
    Set {
        user: bool,
        parent: bool,
        name: String,
        versions: Vec<String>,
    },

    /// Uninstall a specific version of a tool.
    /// asdf uninstall <name> <version>
    Uninstall(String, String),

    /// Display the installation path for a tool.
    /// asdf where <name> [<version>]
    Where(String, Option<String>),

    /// Display the executable path for a command.
    /// asdf which <command>
    Which(String),

    // ==========================
    // UTILS
    // ==========================
    /// Execute a command through the asdf shim.
    /// asdf exec <command> [args...]
    Exec { command: String, args: Vec<String> },

    /// Run a utility inside the environment used for shim execution.
    /// asdf env <command> [util]
    Env {
        command: String,
        util: Option<String>,
    },

    /// Print OS, shell, and ASDF debug information.
    /// asdf info
    Info,

    /// Print the installed ASDF version.
    /// asdf version
    Version,

    /// Recreate shims for a specific tool version.
    /// asdf reshim <name> <version>
    Reshim(String, String),

    /// List the plugins and versions that provide a command.
    /// asdf shimversions <command>
    ShimVersions(String),
}

impl AsdfCommands {
    pub fn execute(&self) -> io::Result<String> {
        let mut cmd = Command::new("asdf");

        match self {
            // ==========================
            // MANAGE PLUGINS
            // ==========================
            Self::PluginAdd(name, Some(url)) => {
                cmd.args(["plugin", "add", name, url]);
            }
            Self::PluginAdd(name, None) => {
                cmd.args(["plugin", "add", name]);
            }
            Self::PluginList { urls, refs } => {
                cmd.args(["plugin", "list"]);
                if *urls {
                    cmd.arg("--urls");
                }
                if *refs {
                    cmd.arg("--refs");
                }
            }
            Self::PluginListAll => {
                cmd.args(["plugin", "list", "all"]);
            }
            Self::PluginRemove(name) => {
                cmd.args(["plugin", "remove", name]);
            }
            Self::PluginUpdate(name, Some(git_ref)) => {
                cmd.args(["plugin", "update", name, git_ref]);
            }
            Self::PluginUpdate(name, None) => {
                cmd.args(["plugin", "update", name]);
            }
            Self::PluginUpdateAll => {
                cmd.args(["plugin", "update", "--all"]);
            }

            // ==========================
            // MANAGE TOOLS
            // ==========================
            Self::Current => {
                cmd.arg("current");
            }
            Self::CurrentTool(name) => {
                cmd.args(["current", name]);
            }
            Self::Help(name, Some(version)) => {
                cmd.args(["help", name, version]);
            }
            Self::Help(name, None) => {
                cmd.args(["help", name]);
            }
            Self::Install => {
                cmd.arg("install");
            }
            Self::InstallTool(name) => {
                cmd.args(["install", name]);
            }
            Self::InstallVersion(name, version) => {
                cmd.args(["install", name, version]);
            }
            Self::InstallLatest(name, Some(prefix)) => {
                cmd.args(["install", name, &format!("latest:{prefix}")]);
            }
            Self::InstallLatest(name, None) => {
                cmd.args(["install", name, "latest"]);
            }
            Self::Latest(name, Some(prefix)) => {
                cmd.args(["latest", name, prefix]);
            }
            Self::Latest(name, None) => {
                cmd.args(["latest", name]);
            }
            Self::LatestAll => {
                cmd.args(["latest", "--all"]);
            }
            Self::List(name, Some(version)) => {
                cmd.args(["list", name, version]);
            }
            Self::List(name, None) => {
                cmd.args(["list", name]);
            }
            Self::ListAll(name, Some(version)) => {
                cmd.args(["list", "all", name, version]);
            }
            Self::ListAll(name, None) => {
                cmd.args(["list", "all", name]);
            }
            Self::Set {
                user,
                parent,
                name,
                versions,
            } => {
                cmd.arg("set");

                if *user {
                    cmd.arg("-u");
                }

                if *parent {
                    cmd.arg("-p");
                }

                cmd.arg(name);
                cmd.args(versions);
            }
            Self::Uninstall(name, version) => {
                cmd.args(["uninstall", name, version]);
            }
            Self::Where(name, Some(version)) => {
                cmd.args(["where", name, version]);
            }
            Self::Where(name, None) => {
                cmd.args(["where", name]);
            }
            Self::Which(command) => {
                cmd.args(["which", command]);
            }

            // ==========================
            // UTILS
            // ==========================
            Self::Exec { command, args } => {
                cmd.arg("exec");
                cmd.arg(command);
                cmd.args(args);
            }
            Self::Env { command, util } => {
                cmd.arg("env");
                cmd.arg(command);

                if let Some(util) = util {
                    cmd.arg(util);
                }
            }
            Self::Info => {
                cmd.arg("info");
            }
            Self::Version => {
                cmd.arg("version");
            }
            Self::Reshim(name, version) => {
                cmd.args(["reshim", name, version]);
            }
            Self::ShimVersions(command) => {
                cmd.args(["shimversions", command]);
            }
        }

        let output = cmd.output()?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                String::from_utf8_lossy(&output.stderr).into_owned(),
            ))
        }
    }

    pub fn from_name(name: &str, params: Vec<String>) -> Result<Self, String> {
        match name {
            "PluginAdd" => match params.as_slice() {
                [plugin] => Ok(Self::PluginAdd(plugin.clone(), None)),
                [plugin, url] => Ok(Self::PluginAdd(plugin.clone(), Some(url.clone()))),
                _ => Err("PluginAdd expects 1 or 2 parameters".to_string()),
            },

            "PluginList" => {
                let urls = params.iter().any(|p| p == "--urls");
                let refs = params.iter().any(|p| p == "--refs");
                // Reject unknown flags if you want
                if params.iter().any(|p| p != "--urls" && p != "--refs") {
                    return Err("Invalid PluginList arguments".into());
                }

                Ok(Self::PluginList { urls, refs })
            }

            "PluginListAll" => Ok(Self::PluginListAll),

            "PluginRemove" => match params.as_slice() {
                [plugin] => Ok(Self::PluginRemove(plugin.clone())),
                _ => Err("PluginRemove expects 1 parameter".to_string()),
            },

            "PluginUpdate" => match params.as_slice() {
                [plugin] => Ok(Self::PluginUpdate(plugin.clone(), None)),
                [plugin, git_ref] => Ok(Self::PluginUpdate(plugin.clone(), Some(git_ref.clone()))),
                _ => Err("PluginUpdate expects 1 or 2 parameters".to_string()),
            },

            "PluginUpdateAll" => Ok(Self::PluginUpdateAll),

            "Current" => Ok(Self::Current),

            "CurrentTool" => match params.as_slice() {
                [tool] => Ok(Self::CurrentTool(tool.clone())),
                _ => Err("CurrentTool expects 1 parameter".to_string()),
            },

            "Help" => match params.as_slice() {
                [tool] => Ok(Self::Help(tool.clone(), None)),
                [tool, version] => Ok(Self::Help(tool.clone(), Some(version.clone()))),
                _ => Err("Help expects 1 or 2 parameters".to_string()),
            },

            "Install" => Ok(Self::Install),

            "InstallTool" => match params.as_slice() {
                [tool] => Ok(Self::InstallTool(tool.clone())),
                _ => Err("InstallTool expects 1 parameter".to_string()),
            },

            "InstallVersion" => match params.as_slice() {
                [tool, version] => Ok(Self::InstallVersion(tool.clone(), version.clone())),
                _ => Err("InstallVersion expects 2 parameters".to_string()),
            },

            "InstallLatest" => match params.as_slice() {
                [tool] => Ok(Self::InstallLatest(tool.clone(), None)),
                [tool, prefix] => Ok(Self::InstallLatest(tool.clone(), Some(prefix.clone()))),
                _ => Err("InstallLatest expects 1 or 2 parameters".to_string()),
            },

            "Latest" => match params.as_slice() {
                [tool] => Ok(Self::Latest(tool.clone(), None)),
                [tool, prefix] => Ok(Self::Latest(tool.clone(), Some(prefix.clone()))),
                _ => Err("Latest expects 1 or 2 parameters".into()),
            },

            "LatestAll" => Ok(Self::LatestAll),

            "List" => match params.as_slice() {
                [tool] => Ok(Self::List(tool.clone(), None)),
                [tool, version] => Ok(Self::List(tool.clone(), Some(version.clone()))),
                _ => Err("List expects 1 or 2 parameters".into()),
            },

            "ListAll" => match params.as_slice() {
                [tool] => Ok(Self::ListAll(tool.clone(), None)),
                [tool, version] => Ok(Self::ListAll(tool.clone(), Some(version.clone()))),
                _ => Err("ListAll expects 1 or 2 parameters".into()),
            },

            "Set" => {
                if params.len() < 2 {
                    return Err("Set expects at least a tool and one version".into());
                }

                Ok(Self::Set {
                    user: false,
                    parent: false,
                    name: params[0].clone(),
                    versions: params[1..].to_vec(),
                })
            }

            "Uninstall" => match params.as_slice() {
                [tool, version] => Ok(Self::Uninstall(tool.clone(), version.clone())),
                _ => Err("Uninstall expects 2 parameters".into()),
            },

            "Where" => match params.as_slice() {
                [tool] => Ok(Self::Where(tool.clone(), None)),
                [tool, version] => Ok(Self::Where(tool.clone(), Some(version.clone()))),
                _ => Err("Where expects 1 or 2 parameters".into()),
            },

            "Which" => match params.as_slice() {
                [command] => Ok(Self::Which(command.clone())),
                _ => Err("Which expects 1 parameter".into()),
            },

            "Exec" => {
                if params.is_empty() {
                    return Err("Exec expects a command".into());
                }

                Ok(Self::Exec {
                    command: params[0].clone(),
                    args: params[1..].to_vec(),
                })
            }

            "Env" => match params.as_slice() {
                [command] => Ok(Self::Env {
                    command: command.clone(),
                    util: None,
                }),
                [command, util] => Ok(Self::Env {
                    command: command.clone(),
                    util: Some(util.clone()),
                }),
                _ => Err("Env expects 1 or 2 parameters".into()),
            },

            "Info" => Ok(Self::Info),

            "Version" => Ok(Self::Version),

            "Reshim" => match params.as_slice() {
                [tool, version] => Ok(Self::Reshim(tool.clone(), version.clone())),
                _ => Err("Reshim expects 2 parameters".into()),
            },

            "ShimVersions" => match params.as_slice() {
                [command] => Ok(Self::ShimVersions(command.clone())),
                _ => Err("ShimVersions expects 1 parameter".into()),
            },

            _ => Err(format!("Unknown command '{name}'")),
        }
    }
}

pub fn get_asdf_metadata() -> Vec<(&'static str, &'static str)> {
    return vec![
        // ==========================
        // MANAGE PLUGINS
        // ==========================
        (
            "PluginAdd",
            "Add a plugin from the plugin repository or a Git repository.",
        ),
        ("PluginList", "List installed plugins."),
        (
            "PluginListAll",
            "List all plugins available in the asdf plugin repository.",
        ),
        ("PluginRemove", "Remove an installed plugin."),
        (
            "PluginUpdate",
            "Update a plugin to the latest commit or a specified git reference.",
        ),
        (
            "PluginUpdateAll",
            "Update all installed plugins to their latest versions.",
        ),
        // ==========================
        // MANAGE TOOLS
        // ==========================
        (
            "Current",
            "Display the current version for all installed tools.",
        ),
        (
            "CurrentTool",
            "Display the current version for a specific tool.",
        ),
        ("Help", "Show documentation for a plugin or tool."),
        (
            "Install",
            "Install all tool versions listed in the .tool-versions file.",
        ),
        (
            "InstallTool",
            "Install the configured version of a specific tool.",
        ),
        ("InstallVersion", "Install a specific version of a tool."),
        (
            "InstallLatest",
            "Install the latest stable version of a tool.",
        ),
        ("Latest", "Display the latest stable version of a tool."),
        (
            "LatestAll",
            "Display the latest stable versions for all tools.",
        ),
        ("List", "List installed versions of a tool."),
        ("ListAll", "List all available versions of a tool."),
        (
            "Set",
            "Set one or more tool versions in a .tool-versions file.",
        ),
        ("Uninstall", "Remove an installed version of a tool."),
        ("Where", "Display the installation path of a tool."),
        ("Which", "Display the executable path for a command."),
        // ==========================
        // UTILS
        // ==========================
        (
            "Exec",
            "Execute a command using the current asdf tool versions.",
        ),
        (
            "Env",
            "Run a utility inside the environment used by an asdf shim.",
        ),
        (
            "Info",
            "Display operating system, shell, and ASDF debug information.",
        ),
        ("Version", "Display the installed ASDF version."),
        ("Reshim", "Recreate shims for a specific tool version."),
        (
            "ShimVersions",
            "List the plugins and versions that provide a command.",
        ),
    ];
}
