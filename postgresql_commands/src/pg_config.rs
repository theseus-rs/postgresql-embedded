use crate::traits::CommandBuilder;
use crate::Settings;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// `pg_config` provides information about the installed version of `PostgreSQL`.
#[derive(Clone, Debug, Default)]
pub struct PgConfigBuilder {
    program_dir: Option<PathBuf>,
    envs: Vec<(OsString, OsString)>,
    bindir: bool,
    docdir: bool,
    htmldir: bool,
    includedir: bool,
    pkgincludedir: bool,
    includedir_server: bool,
    libdir: bool,
    pkglibdir: bool,
    localedir: bool,
    mandir: bool,
    sharedir: bool,
    sysconfdir: bool,
    pgxs: bool,
    configure: bool,
    cc: bool,
    cppflags: bool,
    cflags: bool,
    cflags_sl: bool,
    ldflags: bool,
    ldflags_ex: bool,
    ldflags_sl: bool,
    libs: bool,
    version: bool,
    help: bool,
}

impl PgConfigBuilder {
    /// Create a new [`PgConfigBuilder`]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`PgConfigBuilder`] from [Settings]
    pub fn from(settings: &dyn Settings) -> Self {
        Self::new().program_dir(settings.get_binary_dir())
    }

    /// Location of the program binary
    #[must_use]
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// Set the bindir
    #[must_use]
    pub fn bindir(mut self) -> Self {
        self.bindir = true;
        self
    }

    /// Set the docdir
    #[must_use]
    pub fn docdir(mut self) -> Self {
        self.docdir = true;
        self
    }

    /// Set the htmldir
    #[must_use]
    pub fn htmldir(mut self) -> Self {
        self.htmldir = true;
        self
    }

    /// Set the includedir
    #[must_use]
    pub fn includedir(mut self) -> Self {
        self.includedir = true;
        self
    }

    /// Set the pkgincludedir
    #[must_use]
    pub fn pkgincludedir(mut self) -> Self {
        self.pkgincludedir = true;
        self
    }

    /// Set the `includedir_server`
    #[must_use]
    pub fn includedir_server(mut self) -> Self {
        self.includedir_server = true;
        self
    }

    /// Set the libdir
    #[must_use]
    pub fn libdir(mut self) -> Self {
        self.libdir = true;
        self
    }

    /// Set the pkglibdir
    #[must_use]
    pub fn pkglibdir(mut self) -> Self {
        self.pkglibdir = true;
        self
    }

    /// Set the localedir
    #[must_use]
    pub fn localedir(mut self) -> Self {
        self.localedir = true;
        self
    }

    /// Set the mandir
    #[must_use]
    pub fn mandir(mut self) -> Self {
        self.mandir = true;
        self
    }

    /// Set the sharedir
    #[must_use]
    pub fn sharedir(mut self) -> Self {
        self.sharedir = true;
        self
    }

    /// Set the sysconfdir
    #[must_use]
    pub fn sysconfdir(mut self) -> Self {
        self.sysconfdir = true;
        self
    }

    /// Set the pgxs
    #[must_use]
    pub fn pgxs(mut self) -> Self {
        self.pgxs = true;
        self
    }

    /// Set the configure flag
    #[must_use]
    pub fn configure(mut self) -> Self {
        self.configure = true;
        self
    }

    /// Set the cc flag
    #[must_use]
    pub fn cc(mut self) -> Self {
        self.cc = true;
        self
    }

    /// Set the cppflags flag
    #[must_use]
    pub fn cppflags(mut self) -> Self {
        self.cppflags = true;
        self
    }

    /// Set the cflags flag
    #[must_use]
    pub fn cflags(mut self) -> Self {
        self.cflags = true;
        self
    }

    /// Set the `cflags_sl` flag
    #[must_use]
    pub fn cflags_sl(mut self) -> Self {
        self.cflags_sl = true;
        self
    }

    /// Set the ldflags flag
    #[must_use]
    pub fn ldflags(mut self) -> Self {
        self.ldflags = true;
        self
    }

    /// Set the `ldflags_ex` flag
    #[must_use]
    pub fn ldflags_ex(mut self) -> Self {
        self.ldflags_ex = true;
        self
    }

    /// Set the `ldflags_sl` flag
    #[must_use]
    pub fn ldflags_sl(mut self) -> Self {
        self.ldflags_sl = true;
        self
    }

    /// Set the libs flag
    #[must_use]
    pub fn libs(mut self) -> Self {
        self.libs = true;
        self
    }

    /// output version information, then exit
    #[must_use]
    pub fn version(mut self) -> Self {
        self.version = true;
        self
    }

    /// show help, then exit
    #[must_use]
    pub fn help(mut self) -> Self {
        self.help = true;
        self
    }
}

impl CommandBuilder for PgConfigBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "pg_config".as_ref()
    }

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf> {
        &self.program_dir
    }

    /// Get the arguments for the command
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if self.bindir {
            args.push("--bindir".into());
        }

        if self.docdir {
            args.push("--docdir".into());
        }

        if self.htmldir {
            args.push("--htmldir".into());
        }

        if self.includedir {
            args.push("--includedir".into());
        }

        if self.pkgincludedir {
            args.push("--pkgincludedir".into());
        }

        if self.includedir_server {
            args.push("--includedir-server".into());
        }

        if self.libdir {
            args.push("--libdir".into());
        }

        if self.pkglibdir {
            args.push("--pkglibdir".into());
        }

        if self.localedir {
            args.push("--localedir".into());
        }

        if self.mandir {
            args.push("--mandir".into());
        }

        if self.sharedir {
            args.push("--sharedir".into());
        }

        if self.sysconfdir {
            args.push("--sysconfdir".into());
        }

        if self.pgxs {
            args.push("--pgxs".into());
        }

        if self.configure {
            args.push("--configure".into());
        }

        if self.cc {
            args.push("--cc".into());
        }

        if self.cppflags {
            args.push("--cppflags".into());
        }

        if self.cflags {
            args.push("--cflags".into());
        }

        if self.cflags_sl {
            args.push("--cflags_sl".into());
        }

        if self.ldflags {
            args.push("--ldflags".into());
        }

        if self.ldflags_ex {
            args.push("--ldflags_ex".into());
        }

        if self.ldflags_sl {
            args.push("--ldflags_sl".into());
        }

        if self.libs {
            args.push("--libs".into());
        }

        if self.version {
            args.push("--version".into());
        }

        if self.help {
            args.push("--help".into());
        }

        args
    }

    /// Get the environment variables for the command
    fn get_envs(&self) -> Vec<(OsString, OsString)> {
        self.envs.clone()
    }

    /// Set an environment variable for the command
    fn env<S: AsRef<OsStr>>(mut self, key: S, value: S) -> Self {
        self.envs
            .push((key.as_ref().to_os_string(), value.as_ref().to_os_string()));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::CommandToString;
    use crate::TestSettings;
    use test_log::test;

    #[test]
    fn test_builder_new() {
        let command = PgConfigBuilder::new().program_dir(".").build();
        assert_eq!(
            PathBuf::from(".").join("pg_config"),
            PathBuf::from(command.to_command_string().replace('"', ""))
        );
    }

    #[test]
    fn test_builder_from() {
        let command = PgConfigBuilder::from(&TestSettings).build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#""./pg_config""#;
        #[cfg(target_os = "windows")]
        let command_prefix = r#"".\\pg_config""#;

        assert_eq!(format!("{command_prefix}"), command.to_command_string());
    }

    #[test]
    fn test_builder() {
        let command = PgConfigBuilder::new()
            .env("PGDATABASE", "database")
            .bindir()
            .docdir()
            .htmldir()
            .includedir()
            .pkgincludedir()
            .includedir_server()
            .libdir()
            .pkglibdir()
            .localedir()
            .mandir()
            .sharedir()
            .sysconfdir()
            .pgxs()
            .configure()
            .cc()
            .cppflags()
            .cflags()
            .cflags_sl()
            .ldflags()
            .ldflags_ex()
            .ldflags_sl()
            .libs()
            .version()
            .help()
            .build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#"PGDATABASE="database" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = String::new();

        assert_eq!(
            format!(
                r#"{command_prefix}"pg_config" "--bindir" "--docdir" "--htmldir" "--includedir" "--pkgincludedir" "--includedir-server" "--libdir" "--pkglibdir" "--localedir" "--mandir" "--sharedir" "--sysconfdir" "--pgxs" "--configure" "--cc" "--cppflags" "--cflags" "--cflags_sl" "--ldflags" "--ldflags_ex" "--ldflags_sl" "--libs" "--version" "--help""#
            ),
            command.to_command_string()
        );
    }
}
