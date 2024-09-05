use crate::traits::CommandBuilder;
use crate::Settings;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// `postgres` is the `PostgreSQL` server.
#[derive(Clone, Debug, Default)]
pub struct PostgresBuilder {
    program_dir: Option<PathBuf>,
    envs: Vec<(OsString, OsString)>,
    n_buffers: Option<u32>,
    runtime_param: Option<(OsString, OsString)>,
    print_runtime_param: Option<OsString>,
    debugging_level: Option<u8>,
    data_dir: Option<PathBuf>,
    european_date_format: bool,
    fsync_off: bool,
    host: Option<OsString>,
    tcp_ip_connections: bool,
    socket_location: Option<PathBuf>,
    max_connections: Option<u32>,
    port: Option<u16>,
    show_stats: bool,
    work_mem: Option<u32>,
    version: bool,
    describe_config: bool,
    help: bool,
    forbidden_plan_types: Option<OsString>,
    allow_system_table_changes: bool,
    disable_system_indexes: bool,
    show_timings: Option<OsString>,
    send_sigabrt: bool,
    wait_seconds: Option<u32>,
    single_user_mode: bool,
    dbname: Option<OsString>,
    override_debugging_level: Option<u8>,
    echo_statement: bool,
    no_newline_delimiter: bool,
    output_file: Option<PathBuf>,
    bootstrapping_mode: bool,
    check_mode: bool,
}

impl PostgresBuilder {
    /// Create a new [`PostgresBuilder`]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`PostgresBuilder`] from [Settings]
    pub fn from(settings: &dyn Settings) -> Self {
        Self::new()
            .program_dir(settings.get_binary_dir())
            .host(settings.get_host())
            .port(settings.get_port())
    }

    /// Location of the program binary
    #[must_use]
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// number of shared buffers
    #[must_use]
    pub fn n_buffers(mut self, n_buffers: u32) -> Self {
        self.n_buffers = Some(n_buffers);
        self
    }

    /// set run-time parameter
    #[must_use]
    pub fn runtime_param<S: AsRef<OsStr>>(mut self, name: S, value: S) -> Self {
        self.runtime_param = Some((name.as_ref().into(), value.as_ref().into()));
        self
    }

    /// print value of run-time parameter, then exit
    #[must_use]
    pub fn print_runtime_param<S: AsRef<OsStr>>(mut self, name: S) -> Self {
        self.print_runtime_param = Some(name.as_ref().to_os_string());
        self
    }

    /// debugging level
    #[must_use]
    pub fn debugging_level(mut self, level: u8) -> Self {
        self.debugging_level = Some(level);
        self
    }

    /// database directory
    #[must_use]
    pub fn data_dir<P: Into<PathBuf>>(mut self, dir: P) -> Self {
        self.data_dir = Some(dir.into());
        self
    }

    /// use European date input format (DMY)
    #[must_use]
    pub fn european_date_format(mut self) -> Self {
        self.european_date_format = true;
        self
    }

    /// turn fsync off
    #[must_use]
    pub fn fsync_off(mut self) -> Self {
        self.fsync_off = true;
        self
    }

    /// host name or IP address to listen on
    #[must_use]
    pub fn host<S: AsRef<OsStr>>(mut self, host: S) -> Self {
        self.host = Some(host.as_ref().to_os_string());
        self
    }

    /// enable TCP/IP connections (deprecated)
    #[must_use]
    pub fn tcp_ip_connections(mut self) -> Self {
        self.tcp_ip_connections = true;
        self
    }

    /// Unix-domain socket location
    #[must_use]
    pub fn socket_location<P: Into<PathBuf>>(mut self, dir: P) -> Self {
        self.socket_location = Some(dir.into());
        self
    }

    /// maximum number of allowed connections
    #[must_use]
    pub fn max_connections(mut self, max: u32) -> Self {
        self.max_connections = Some(max);
        self
    }

    /// port number to listen on
    #[must_use]
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// show statistics after each query
    #[must_use]
    pub fn show_stats(mut self) -> Self {
        self.show_stats = true;
        self
    }

    /// set amount of memory for sorts (in kB)
    #[must_use]
    pub fn work_mem(mut self, mem: u32) -> Self {
        self.work_mem = Some(mem);
        self
    }

    /// output version information, then exit
    #[must_use]
    pub fn version(mut self) -> Self {
        self.version = true;
        self
    }

    /// describe configuration parameters, then exit
    #[must_use]
    pub fn describe_config(mut self) -> Self {
        self.describe_config = true;
        self
    }

    /// show help, then exit
    #[must_use]
    pub fn help(mut self) -> Self {
        self.help = true;
        self
    }

    /// forbid use of some plan types
    #[must_use]
    pub fn forbidden_plan_types<S: AsRef<OsStr>>(mut self, types: S) -> Self {
        self.forbidden_plan_types = Some(types.as_ref().to_os_string());
        self
    }

    /// allow system table structure changes
    #[must_use]
    pub fn allow_system_table_changes(mut self) -> Self {
        self.allow_system_table_changes = true;
        self
    }

    /// disable system indexes
    #[must_use]
    pub fn disable_system_indexes(mut self) -> Self {
        self.disable_system_indexes = true;
        self
    }

    /// show timings after each query
    #[must_use]
    pub fn show_timings<S: AsRef<OsStr>>(mut self, timings: S) -> Self {
        self.show_timings = Some(timings.as_ref().to_os_string());
        self
    }

    /// send SIGABRT to all backend processes if one dies
    #[must_use]
    pub fn send_sigabrt(mut self) -> Self {
        self.send_sigabrt = true;
        self
    }

    /// wait NUM seconds to allow attach from a debugger
    #[must_use]
    pub fn wait_seconds(mut self, seconds: u32) -> Self {
        self.wait_seconds = Some(seconds);
        self
    }

    /// selects single-user mode (must be first argument)
    #[must_use]
    pub fn single_user_mode(mut self) -> Self {
        self.single_user_mode = true;
        self
    }

    /// database name (defaults to user name)
    #[must_use]
    pub fn dbname<S: AsRef<OsStr>>(mut self, dbname: S) -> Self {
        self.dbname = Some(dbname.as_ref().to_os_string());
        self
    }

    /// override debugging level
    #[must_use]
    pub fn override_debugging_level(mut self, level: u8) -> Self {
        self.override_debugging_level = Some(level);
        self
    }

    /// echo statement before execution
    #[must_use]
    pub fn echo_statement(mut self) -> Self {
        self.echo_statement = true;
        self
    }

    /// do not use newline as interactive query delimiter
    #[must_use]
    pub fn no_newline_delimiter(mut self) -> Self {
        self.no_newline_delimiter = true;
        self
    }

    /// send stdout and stderr to given file
    #[must_use]
    pub fn output_file<P: Into<PathBuf>>(mut self, file: P) -> Self {
        self.output_file = Some(file.into());
        self
    }

    /// selects bootstrapping mode (must be first argument)
    #[must_use]
    pub fn bootstrapping_mode(mut self) -> Self {
        self.bootstrapping_mode = true;
        self
    }

    /// selects check mode (must be first argument)
    #[must_use]
    pub fn check_mode(mut self) -> Self {
        self.check_mode = true;
        self
    }
}

impl CommandBuilder for PostgresBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "postgres".as_ref()
    }

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf> {
        &self.program_dir
    }

    /// Get the arguments for the command
    #[expect(clippy::too_many_lines)]
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if let Some(n_buffers) = &self.n_buffers {
            args.push("-B".into());
            args.push(n_buffers.to_string().into());
        }

        if let Some((name, value)) = &self.runtime_param {
            args.push("-c".into());
            args.push(format!("{}={}", name.to_string_lossy(), value.to_string_lossy()).into());
        }

        if let Some(name) = &self.print_runtime_param {
            args.push("-C".into());
            args.push(name.into());
        }

        if let Some(level) = &self.debugging_level {
            args.push("-d".into());
            args.push(level.to_string().into());
        }

        if let Some(data_dir) = &self.data_dir {
            args.push("-D".into());
            args.push(data_dir.into());
        }

        if self.european_date_format {
            args.push("-e".into());
        }

        if self.fsync_off {
            args.push("-F".into());
        }

        if let Some(host) = &self.host {
            args.push("-h".into());
            args.push(host.into());
        }

        if self.tcp_ip_connections {
            args.push("-i".into());
        }

        if let Some(socket_location) = &self.socket_location {
            args.push("-k".into());
            args.push(socket_location.into());
        }

        if let Some(max) = &self.max_connections {
            args.push("-N".into());
            args.push(max.to_string().into());
        }

        if let Some(port) = &self.port {
            args.push("-p".into());
            args.push(port.to_string().into());
        }

        if self.show_stats {
            args.push("-s".into());
        }

        if let Some(work_mem) = &self.work_mem {
            args.push("-S".into());
            args.push(work_mem.to_string().into());
        }

        if self.version {
            args.push("--version".into());
        }

        if self.describe_config {
            args.push("--describe-config".into());
        }

        if self.help {
            args.push("--help".into());
        }

        if let Some(forbidden_plan_types) = &self.forbidden_plan_types {
            args.push("-f".into());
            args.push(forbidden_plan_types.into());
        }

        if self.allow_system_table_changes {
            args.push("-O".into());
        }

        if self.disable_system_indexes {
            args.push("-P".into());
        }

        if let Some(show_timings) = &self.show_timings {
            args.push("-t".into());
            args.push(show_timings.into());
        }

        if self.send_sigabrt {
            args.push("-T".into());
        }

        if let Some(seconds) = &self.wait_seconds {
            args.push("-W".into());
            args.push(seconds.to_string().into());
        }

        if self.single_user_mode {
            args.push("--single".into());
        }

        if let Some(dbname) = &self.dbname {
            args.push(dbname.into());
        }

        if let Some(level) = &self.override_debugging_level {
            args.push("-d".into());
            args.push(level.to_string().into());
        }

        if self.echo_statement {
            args.push("-E".into());
        }

        if self.no_newline_delimiter {
            args.push("-j".into());
        }

        if let Some(file) = &self.output_file {
            args.push("-r".into());
            args.push(file.into());
        }

        if self.bootstrapping_mode {
            args.push("--boot".into());
        }

        if self.check_mode {
            args.push("--check".into());
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
        let command = PostgresBuilder::new().program_dir(".").build();
        assert_eq!(
            PathBuf::from(".").join("postgres"),
            PathBuf::from(command.to_command_string().replace('"', ""))
        );
    }

    #[test]
    fn test_builder_from() {
        let command = PostgresBuilder::from(&TestSettings).build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#""./postgres" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = r#"".\\postgres" "#;

        assert_eq!(
            format!(r#"{command_prefix}"-h" "localhost" "-p" "5432""#),
            command.to_command_string()
        );
    }

    #[test]
    fn test_builder() {
        let command = PostgresBuilder::new()
            .env("PGDATABASE", "database")
            .n_buffers(100)
            .runtime_param("name", "value")
            .print_runtime_param("name")
            .debugging_level(3)
            .data_dir("data_dir")
            .european_date_format()
            .fsync_off()
            .host("localhost")
            .tcp_ip_connections()
            .socket_location("socket_location")
            .max_connections(100)
            .port(5432)
            .show_stats()
            .work_mem(100)
            .version()
            .describe_config()
            .help()
            .forbidden_plan_types("type")
            .allow_system_table_changes()
            .disable_system_indexes()
            .show_timings("timings")
            .send_sigabrt()
            .wait_seconds(10)
            .single_user_mode()
            .dbname("dbname")
            .override_debugging_level(3)
            .echo_statement()
            .no_newline_delimiter()
            .output_file("output_file")
            .bootstrapping_mode()
            .check_mode()
            .build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#"PGDATABASE="database" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = String::new();

        assert_eq!(
            format!(
                r#"{command_prefix}"postgres" "-B" "100" "-c" "name=value" "-C" "name" "-d" "3" "-D" "data_dir" "-e" "-F" "-h" "localhost" "-i" "-k" "socket_location" "-N" "100" "-p" "5432" "-s" "-S" "100" "--version" "--describe-config" "--help" "-f" "type" "-O" "-P" "-t" "timings" "-T" "-W" "10" "--single" "dbname" "-d" "3" "-E" "-j" "-r" "output_file" "--boot" "--check""#
            ),
            command.to_command_string()
        );
    }
}
