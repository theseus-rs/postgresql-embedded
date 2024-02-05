use crate::command::traits::CommandBuilder;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// pg_config provides information about the installed version of PostgreSQL.
#[derive(Clone, Debug, Default)]
pub struct PgConfigBuilder {
    program_dir: Option<PathBuf>,
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
    pub fn new() -> Self {
        Self::default()
    }

    /// Location of the program binary
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// Set the bindir
    pub fn bindir<S: AsRef<OsStr>>(mut self, bindir: S) -> Self {
        self.bindir = Some(bindir.as_ref().to_os_string());
        self
    }

    /// Set the docdir
    pub fn docdir<S: AsRef<OsStr>>(mut self, docdir: S) -> Self {
        self.docdir = Some(docdir.as_ref().to_os_string());
        self
    }

    /// Set the htmldir
    pub fn htmldir<S: AsRef<OsStr>>(mut self, htmldir: S) -> Self {
        self.htmldir = Some(htmldir.as_ref().to_os_string());
        self
    }

    /// Set the includedir
    pub fn includedir<S: AsRef<OsStr>>(mut self, includedir: S) -> Self {
        self.includedir = Some(includedir.as_ref().to_os_string());
        self
    }

    /// Set the pkgincludedir
    pub fn pkgincludedir<S: AsRef<OsStr>>(mut self, pkgincludedir: S) -> Self {
        self.pkgincludedir = Some(pkgincludedir.as_ref().to_os_string());
        self
    }

    /// Set the includedir_server
    pub fn includedir_server<S: AsRef<OsStr>>(mut self, includedir_server: S) -> Self {
        self.includedir_server = Some(includedir_server.as_ref().to_os_string());
        self
    }

    /// Set the libdir
    pub fn libdir<S: AsRef<OsStr>>(mut self, libdir: S) -> Self {
        self.libdir = Some(libdir.as_ref().to_os_string());
        self
    }

    /// Set the pkglibdir
    pub fn pkglibdir<S: AsRef<OsStr>>(mut self, pkglibdir: S) -> Self {
        self.pkglibdir = Some(pkglibdir.as_ref().to_os_string());
        self
    }

    /// Set the localedir
    pub fn localedir<S: AsRef<OsStr>>(mut self, localedir: S) -> Self {
        self.localedir = Some(localedir.as_ref().to_os_string());
        self
    }

    /// Set the mandir
    pub fn mandir<S: AsRef<OsStr>>(mut self, mandir: S) -> Self {
        self.mandir = Some(mandir.as_ref().to_os_string());
        self
    }

    /// Set the sharedir
    pub fn sharedir<S: AsRef<OsStr>>(mut self, sharedir: S) -> Self {
        self.sharedir = Some(sharedir.as_ref().to_os_string());
        self
    }

    /// Set the sysconfdir
    pub fn sysconfdir<S: AsRef<OsStr>>(mut self, sysconfdir: S) -> Self {
        self.sysconfdir = Some(sysconfdir.as_ref().to_os_string());
        self
    }

    /// Set the pgxs
    pub fn pgxs<S: AsRef<OsStr>>(mut self, pgxs: S) -> Self {
        self.pgxs = Some(pgxs.as_ref().to_os_string());
        self
    }

    /// Set the configure flag
    pub fn configure(mut self) -> Self {
        self.configure = true;
        self
    }

    /// Set the cc flag
    pub fn cc(mut self) -> Self {
        self.cc = true;
        self
    }

    /// Set the cppflags flag
    pub fn cppflags(mut self) -> Self {
        self.cppflags = true;
        self
    }

    /// Set the cflags flag
    pub fn cflags(mut self) -> Self {
        self.cflags = true;
        self
    }

    /// Set the cflags_sl flag
    pub fn cflags_sl(mut self) -> Self {
        self.cflags_sl = true;
        self
    }

    /// Set the ldflags flag
    pub fn ldflags(mut self) -> Self {
        self.ldflags = true;
        self
    }

    /// Set the ldflags_ex flag
    pub fn ldflags_ex(mut self) -> Self {
        self.ldflags_ex = true;
        self
    }

    /// Set the ldflags_sl flag
    pub fn ldflags_sl(mut self) -> Self {
        self.ldflags_sl = true;
        self
    }

    /// Set the libs flag
    pub fn libs(mut self) -> Self {
        self.libs = true;
        self
    }

    /// output version information, then exit
    pub fn version(mut self) -> Self {
        self.version = true;
        self
    }

    /// show help, then exit
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::traits::CommandToString;

    #[test]
    fn test_builder_new() {
        let command = PgConfigBuilder::new().build();

        assert_eq!(r#""pg_config""#, command.to_command_string());
    }

    #[test]
    fn test_builder() {
        let command = PgConfigBuilder::new()
            .program_dir("/usr/bin")
            .bindir("/usr/local/pgsql/bin")
            .docdir("/usr/local/pgsql/doc")
            .htmldir("/usr/local/pgsql/html")
            .includedir("/usr/local/pgsql/include")
            .pkgincludedir("/usr/local/pgsql/include")
            .includedir_server("/usr/local/pgsql/include/server")
            .libdir("/usr/local/pgsql/lib")
            .pkglibdir("/usr/local/pgsql/lib")
            .localedir("/usr/local/pgsql/share/locale")
            .bindir("/usr/local/pgsql/bin")
            .docdir("/usr/local/pgsql/doc")
            .htmldir("/usr/local/pgsql/html")
            .includedir("/usr/local/pgsql/include")
            .pkgincludedir("/usr/local/pgsql/include")
            .includedir_server("/usr/local/pgsql/include/server")
            .libdir("/usr/local/pgsql/lib")
            .pkglibdir("/usr/local/pgsql/lib")
            .localedir("/usr/local/pgsql/share/locale")
            .mandir("/usr/local/pgsql/man")
            .sharedir("/usr/local/pgsql/share")
            .sysconfdir("/usr/local/pgsql/sysconf")
            .pgxs("/usr/local/pgsql/pgxs")
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
            r#""/usr/bin/pg_config" "--bindir" "/usr/local/pgsql/bin" "--docdir" "/usr/local/pgsql/doc" "--htmldir" "/usr/local/pgsql/html" "--includedir" "/usr/local/pgsql/include" "--pkgincludedir" "/usr/local/pgsql/include" "--includedir-server" "/usr/local/pgsql/include/server" "--libdir" "/usr/local/pgsql/lib" "--pkglibdir" "/usr/local/pgsql/lib" "--localedir" "/usr/local/pgsql/share/locale" "--mandir" "/usr/local/pgsql/man" "--sharedir" "/usr/local/pgsql/share" "--sysconfdir" "/usr/local/pgsql/sysconf" "--pgxs" "/usr/local/pgsql/pgxs" "--configure" "--cc" "--cppflags" "--cflags" "--cflags_sl" "--ldflags" "--ldflags_ex" "--ldflags_sl" "--libs" "--version" "--help""#,
            command.to_command_string()
        );
    }
}
