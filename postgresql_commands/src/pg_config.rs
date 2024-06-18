use crate::traits::CommandBuilder;
use crate::Settings;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// `pg_config` provides information about the installed version of `PostgreSQL`.
#[derive(Clone, Debug, Default)]
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::struct_excessive_bools)]
pub struct PgConfigBuilder {
    program_dir: Option<PathBuf>,
    envs: Vec<(OsString, OsString)>,
    bindir: Option<OsString>,
    docdir: Option<OsString>,
    htmldir: Option<OsString>,
    includedir: Option<OsString>,
    pkgincludedir: Option<OsString>,
    includedir_server: Option<OsString>,
    libdir: Option<OsString>,
    pkglibdir: Option<OsString>,
    localedir: Option<OsString>,
    mandir: Option<OsString>,
    sharedir: Option<OsString>,
    sysconfdir: Option<OsString>,
    pgxs: Option<OsString>,
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
    pub fn bindir<S: AsRef<OsStr>>(mut self, bindir: S) -> Self {
        self.bindir = Some(bindir.as_ref().to_os_string());
        self
    }

    /// Set the docdir
    #[must_use]
    pub fn docdir<S: AsRef<OsStr>>(mut self, docdir: S) -> Self {
        self.docdir = Some(docdir.as_ref().to_os_string());
        self
    }

    /// Set the htmldir
    #[must_use]
    pub fn htmldir<S: AsRef<OsStr>>(mut self, htmldir: S) -> Self {
        self.htmldir = Some(htmldir.as_ref().to_os_string());
        self
    }

    /// Set the includedir
    #[must_use]
    pub fn includedir<S: AsRef<OsStr>>(mut self, includedir: S) -> Self {
        self.includedir = Some(includedir.as_ref().to_os_string());
        self
    }

    /// Set the pkgincludedir
    #[must_use]
    pub fn pkgincludedir<S: AsRef<OsStr>>(mut self, pkgincludedir: S) -> Self {
        self.pkgincludedir = Some(pkgincludedir.as_ref().to_os_string());
        self
    }

    /// Set the `includedir_server`
    #[must_use]
    pub fn includedir_server<S: AsRef<OsStr>>(mut self, includedir_server: S) -> Self {
        self.includedir_server = Some(includedir_server.as_ref().to_os_string());
        self
    }

    /// Set the libdir
    #[must_use]
    pub fn libdir<S: AsRef<OsStr>>(mut self, libdir: S) -> Self {
        self.libdir = Some(libdir.as_ref().to_os_string());
        self
    }

    /// Set the pkglibdir
    #[must_use]
    pub fn pkglibdir<S: AsRef<OsStr>>(mut self, pkglibdir: S) -> Self {
        self.pkglibdir = Some(pkglibdir.as_ref().to_os_string());
        self
    }

    /// Set the localedir
    #[must_use]
    pub fn localedir<S: AsRef<OsStr>>(mut self, localedir: S) -> Self {
        self.localedir = Some(localedir.as_ref().to_os_string());
        self
    }

    /// Set the mandir
    #[must_use]
    pub fn mandir<S: AsRef<OsStr>>(mut self, mandir: S) -> Self {
        self.mandir = Some(mandir.as_ref().to_os_string());
        self
    }

    /// Set the sharedir
    #[must_use]
    pub fn sharedir<S: AsRef<OsStr>>(mut self, sharedir: S) -> Self {
        self.sharedir = Some(sharedir.as_ref().to_os_string());
        self
    }

    /// Set the sysconfdir
    #[must_use]
    pub fn sysconfdir<S: AsRef<OsStr>>(mut self, sysconfdir: S) -> Self {
        self.sysconfdir = Some(sysconfdir.as_ref().to_os_string());
        self
    }

    /// Set the pgxs
    #[must_use]
    pub fn pgxs<S: AsRef<OsStr>>(mut self, pgxs: S) -> Self {
        self.pgxs = Some(pgxs.as_ref().to_os_string());
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

        if let Some(bindir) = &self.bindir {
            args.push("--bindir".into());
            args.push(bindir.into());
        }

        if let Some(docdir) = &self.docdir {
            args.push("--docdir".into());
            args.push(docdir.into());
        }

        if let Some(htmldir) = &self.htmldir {
            args.push("--htmldir".into());
            args.push(htmldir.into());
        }

        if let Some(includedir) = &self.includedir {
            args.push("--includedir".into());
            args.push(includedir.into());
        }

        if let Some(pkgincludedir) = &self.pkgincludedir {
            args.push("--pkgincludedir".into());
            args.push(pkgincludedir.into());
        }

        if let Some(includedir_server) = &self.includedir_server {
            args.push("--includedir-server".into());
            args.push(includedir_server.into());
        }

        if let Some(libdir) = &self.libdir {
            args.push("--libdir".into());
            args.push(libdir.into());
        }

        if let Some(pkglibdir) = &self.pkglibdir {
            args.push("--pkglibdir".into());
            args.push(pkglibdir.into());
        }

        if let Some(localedir) = &self.localedir {
            args.push("--localedir".into());
            args.push(localedir.into());
        }

        if let Some(mandir) = &self.mandir {
            args.push("--mandir".into());
            args.push(mandir.into());
        }

        if let Some(sharedir) = &self.sharedir {
            args.push("--sharedir".into());
            args.push(sharedir.into());
        }

        if let Some(sysconfdir) = &self.sysconfdir {
            args.push("--sysconfdir".into());
            args.push(sysconfdir.into());
        }

        if let Some(pgxs) = &self.pgxs {
            args.push("--pgxs".into());
            args.push(pgxs.into());
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
        assert_eq!(r#""./pg_config""#, command.to_command_string());
    }

    #[test]
    fn test_builder() {
        let command = PgConfigBuilder::new()
            .env("PGDATABASE", "database")
            .bindir("bindir")
            .docdir("docdir")
            .htmldir("htmldir")
            .includedir("includedir")
            .pkgincludedir("pkgincludedir")
            .includedir_server("includedir_server")
            .libdir("libdir")
            .pkglibdir("pkglibdir")
            .localedir("localedir")
            .mandir("mandir")
            .sharedir("sharedir")
            .sysconfdir("sysconfdir")
            .pgxs("pgxs")
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

        assert_eq!(
            r#"PGDATABASE="database" "pg_config" "--bindir" "bindir" "--docdir" "docdir" "--htmldir" "htmldir" "--includedir" "includedir" "--pkgincludedir" "pkgincludedir" "--includedir-server" "includedir_server" "--libdir" "libdir" "--pkglibdir" "pkglibdir" "--localedir" "localedir" "--mandir" "mandir" "--sharedir" "sharedir" "--sysconfdir" "sysconfdir" "--pgxs" "pgxs" "--configure" "--cc" "--cppflags" "--cflags" "--cflags_sl" "--ldflags" "--ldflags_ex" "--ldflags_sl" "--libs" "--version" "--help""#,
            command.to_command_string()
        );
    }
}
