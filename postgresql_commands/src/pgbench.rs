use crate::traits::CommandBuilder;
use crate::Settings;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// `pgbench` is a benchmarking tool for `PostgreSQL`.
#[derive(Clone, Debug, Default)]
pub struct PgBenchBuilder {
    program_dir: Option<PathBuf>,
    envs: Vec<(OsString, OsString)>,
    initialize: bool,
    init_steps: Option<OsString>,
    fill_factor: Option<usize>,
    no_vacuum: bool,
    quiet: bool,
    scale: Option<usize>,
    foreign_keys: bool,
    index_tablespace: Option<OsString>,
    partition_method: Option<OsString>,
    partitions: Option<usize>,
    tablespace: Option<OsString>,
    unlogged_tables: bool,
    builtin: Option<OsString>,
    file: Option<OsString>,
    skip_some_updates: bool,
    select_only: bool,
    client: Option<usize>,
    connect: bool,
    define: Option<OsString>,
    jobs: Option<usize>,
    log: bool,
    latency_limit: Option<usize>,
    protocol: Option<OsString>,
    no_vacuum_bench: bool,
    progress: Option<usize>,
    report_per_command: bool,
    rate: Option<usize>,
    scale_bench: Option<usize>,
    transactions: Option<usize>,
    time: Option<usize>,
    vacuum_all: bool,
    aggregate_interval: Option<usize>,
    failures_detailed: bool,
    log_prefix: Option<OsString>,
    max_tries: Option<usize>,
    progress_timestamp: bool,
    random_seed: Option<OsString>,
    sampling_rate: Option<f64>,
    show_script: Option<OsString>,
    verbose_errors: bool,
    debug: bool,
    host: Option<OsString>,
    port: Option<u16>,
    username: Option<OsString>,
    version: bool,
    help: bool,
}

impl PgBenchBuilder {
    /// Create a new [`PgBenchBuilder`]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`PgBenchBuilder`] from [Settings]
    pub fn from(settings: &dyn Settings) -> Self {
        Self::new()
            .program_dir(settings.get_binary_dir())
            .host(settings.get_host())
            .port(settings.get_port())
            .username(settings.get_username())
    }

    /// Location of the program binary
    #[must_use]
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// invokes initialization mode
    #[must_use]
    pub fn initialize(mut self) -> Self {
        self.initialize = true;
        self
    }

    /// run selected initialization steps
    #[must_use]
    pub fn init_steps<S: AsRef<OsStr>>(mut self, steps: S) -> Self {
        self.init_steps = Some(steps.as_ref().to_os_string());
        self
    }

    /// set fill factor
    #[must_use]
    pub fn fill_factor(mut self, factor: usize) -> Self {
        self.fill_factor = Some(factor);
        self
    }

    /// do not run VACUUM during initialization
    #[must_use]
    pub fn no_vacuum(mut self) -> Self {
        self.no_vacuum = true;
        self
    }

    /// quiet logging (one message each 5 seconds)
    #[must_use]
    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }

    /// scaling factor
    #[must_use]
    pub fn scale(mut self, scale: usize) -> Self {
        self.scale = Some(scale);
        self
    }

    /// create foreign key constraints between tables
    #[must_use]
    pub fn foreign_keys(mut self) -> Self {
        self.foreign_keys = true;
        self
    }

    /// create indexes in the specified tablespace
    #[must_use]
    pub fn index_tablespace<S: AsRef<OsStr>>(mut self, tablespace: S) -> Self {
        self.index_tablespace = Some(tablespace.as_ref().to_os_string());
        self
    }

    /// partition `pgbench_accounts` with this method (default: range)
    #[must_use]
    pub fn partition_method<S: AsRef<OsStr>>(mut self, method: S) -> Self {
        self.partition_method = Some(method.as_ref().to_os_string());
        self
    }

    /// partition `pgbench_accounts` into NUM parts (default: 0)
    #[must_use]
    pub fn partitions(mut self, num: usize) -> Self {
        self.partitions = Some(num);
        self
    }

    /// create tables in the specified tablespace
    #[must_use]
    pub fn tablespace<S: AsRef<OsStr>>(mut self, tablespace: S) -> Self {
        self.tablespace = Some(tablespace.as_ref().to_os_string());
        self
    }

    /// create tables as unlogged tables
    #[must_use]
    pub fn unlogged_tables(mut self) -> Self {
        self.unlogged_tables = true;
        self
    }

    /// add builtin script NAME weighted at W (default: 1)
    #[must_use]
    pub fn builtin<S: AsRef<OsStr>>(mut self, name: S) -> Self {
        self.builtin = Some(name.as_ref().to_os_string());
        self
    }

    /// add script FILENAME weighted at W (default: 1)
    #[must_use]
    pub fn file<S: AsRef<OsStr>>(mut self, filename: S) -> Self {
        self.file = Some(filename.as_ref().to_os_string());
        self
    }

    /// skip some updates
    #[must_use]
    pub fn skip_some_updates(mut self) -> Self {
        self.skip_some_updates = true;
        self
    }

    /// perform SELECT-only transactions
    #[must_use]
    pub fn select_only(mut self) -> Self {
        self.select_only = true;
        self
    }

    /// number of concurrent database clients (default: 1)
    #[must_use]
    pub fn client(mut self, num: usize) -> Self {
        self.client = Some(num);
        self
    }

    /// establish new connection for each transaction
    #[must_use]
    pub fn connect(mut self) -> Self {
        self.connect = true;
        self
    }

    /// define variable for use by custom script
    #[must_use]
    pub fn define<S: AsRef<OsStr>>(mut self, var: S) -> Self {
        self.define = Some(var.as_ref().to_os_string());
        self
    }

    /// number of threads (default: 1)
    #[must_use]
    pub fn jobs(mut self, num: usize) -> Self {
        self.jobs = Some(num);
        self
    }

    /// write transaction times to log file
    #[must_use]
    pub fn log(mut self) -> Self {
        self.log = true;
        self
    }

    /// count transactions lasting more than NUM ms as late
    #[must_use]
    pub fn latency_limit(mut self, num: usize) -> Self {
        self.latency_limit = Some(num);
        self
    }

    /// protocol for submitting queries (default: simple)
    #[must_use]
    pub fn protocol<S: AsRef<OsStr>>(mut self, protocol: S) -> Self {
        self.protocol = Some(protocol.as_ref().to_os_string());
        self
    }

    /// do not run VACUUM before tests
    #[must_use]
    pub fn no_vacuum_bench(mut self) -> Self {
        self.no_vacuum_bench = true;
        self
    }

    /// show thread progress report every NUM seconds
    #[must_use]
    pub fn progress(mut self, num: usize) -> Self {
        self.progress = Some(num);
        self
    }

    /// report latencies, failures, and retries per command
    #[must_use]
    pub fn report_per_command(mut self) -> Self {
        self.report_per_command = true;
        self
    }

    /// target rate in transactions per second
    #[must_use]
    pub fn rate(mut self, num: usize) -> Self {
        self.rate = Some(num);
        self
    }

    /// report this scale factor in output
    #[must_use]
    pub fn scale_bench(mut self, scale: usize) -> Self {
        self.scale_bench = Some(scale);
        self
    }

    /// number of transactions each client runs (default: 10)
    #[must_use]
    pub fn transactions(mut self, num: usize) -> Self {
        self.transactions = Some(num);
        self
    }

    /// duration of benchmark test in seconds
    #[must_use]
    pub fn time(mut self, num: usize) -> Self {
        self.time = Some(num);
        self
    }

    /// vacuum all four standard tables before tests
    #[must_use]
    pub fn vacuum_all(mut self) -> Self {
        self.vacuum_all = true;
        self
    }

    /// aggregate data over NUM seconds
    #[must_use]
    pub fn aggregate_interval(mut self, num: usize) -> Self {
        self.aggregate_interval = Some(num);
        self
    }

    /// report the failures grouped by basic types
    #[must_use]
    pub fn failures_detailed(mut self) -> Self {
        self.failures_detailed = true;
        self
    }

    /// prefix for transaction time log file
    #[must_use]
    pub fn log_prefix<S: AsRef<OsStr>>(mut self, prefix: S) -> Self {
        self.log_prefix = Some(prefix.as_ref().to_os_string());
        self
    }

    /// max number of tries to run transaction (default: 1)
    #[must_use]
    pub fn max_tries(mut self, num: usize) -> Self {
        self.max_tries = Some(num);
        self
    }

    /// use Unix epoch timestamps for progress
    #[must_use]
    pub fn progress_timestamp(mut self) -> Self {
        self.progress_timestamp = true;
        self
    }

    /// set random seed ("time", "rand", integer)
    #[must_use]
    pub fn random_seed<S: AsRef<OsStr>>(mut self, seed: S) -> Self {
        self.random_seed = Some(seed.as_ref().to_os_string());
        self
    }

    /// fraction of transactions to log (e.g., 0.01 for 1%)
    #[must_use]
    pub fn sampling_rate(mut self, rate: f64) -> Self {
        self.sampling_rate = Some(rate);
        self
    }

    /// show builtin script code, then exit
    #[must_use]
    pub fn show_script<S: AsRef<OsStr>>(mut self, name: S) -> Self {
        self.show_script = Some(name.as_ref().to_os_string());
        self
    }

    /// print messages of all errors
    #[must_use]
    pub fn verbose_errors(mut self) -> Self {
        self.verbose_errors = true;
        self
    }

    /// print debugging output
    #[must_use]
    pub fn debug(mut self) -> Self {
        self.debug = true;
        self
    }

    /// database server host or socket directory
    #[must_use]
    pub fn host<S: AsRef<OsStr>>(mut self, hostname: S) -> Self {
        self.host = Some(hostname.as_ref().to_os_string());
        self
    }

    /// database server port number
    #[must_use]
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// connect as specified database user
    #[must_use]
    pub fn username<S: AsRef<OsStr>>(mut self, username: S) -> Self {
        self.username = Some(username.as_ref().to_os_string());
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

impl CommandBuilder for PgBenchBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "pgbench".as_ref()
    }

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf> {
        &self.program_dir
    }

    /// Get the arguments for the command
    #[expect(clippy::too_many_lines)]
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if self.initialize {
            args.push("--initialize".into());
        }

        if let Some(steps) = &self.init_steps {
            args.push("--init-steps".into());
            args.push(steps.into());
        }

        if let Some(factor) = &self.fill_factor {
            args.push("--fillfactor".into());
            args.push(factor.to_string().into());
        }

        if self.no_vacuum {
            args.push("--no-vacuum".into());
        }

        if self.quiet {
            args.push("--quiet".into());
        }

        if let Some(scale) = &self.scale {
            args.push("--scale".into());
            args.push(scale.to_string().into());
        }

        if self.foreign_keys {
            args.push("--foreign-keys".into());
        }

        if let Some(tablespace) = &self.index_tablespace {
            args.push("--index-tablespace".into());
            args.push(tablespace.into());
        }

        if let Some(method) = &self.partition_method {
            args.push("--partition-method".into());
            args.push(method.into());
        }

        if let Some(num) = &self.partitions {
            args.push("--partitions".into());
            args.push(num.to_string().into());
        }

        if let Some(tablespace) = &self.tablespace {
            args.push("--tablespace".into());
            args.push(tablespace.into());
        }

        if self.unlogged_tables {
            args.push("--unlogged-tables".into());
        }

        if let Some(name) = &self.builtin {
            args.push("--builtin".into());
            args.push(name.into());
        }

        if let Some(filename) = &self.file {
            args.push("--file".into());
            args.push(filename.into());
        }

        if self.skip_some_updates {
            args.push("--skip-some-updates".into());
        }

        if self.select_only {
            args.push("--select-only".into());
        }

        if let Some(num) = &self.client {
            args.push("--client".into());
            args.push(num.to_string().into());
        }

        if self.connect {
            args.push("--connect".into());
        }

        if let Some(var) = &self.define {
            args.push("--define".into());
            args.push(var.into());
        }

        if let Some(num) = &self.jobs {
            args.push("--jobs".into());
            args.push(num.to_string().into());
        }

        if self.log {
            args.push("--log".into());
        }

        if let Some(num) = &self.latency_limit {
            args.push("--latency-limit".into());
            args.push(num.to_string().into());
        }

        if let Some(protocol) = &self.protocol {
            args.push("--protocol".into());
            args.push(protocol.into());
        }

        if self.no_vacuum_bench {
            args.push("--no-vacuum".into());
        }

        if let Some(num) = &self.progress {
            args.push("--progress".into());
            args.push(num.to_string().into());
        }

        if self.report_per_command {
            args.push("--report-per-command".into());
        }

        if let Some(num) = &self.rate {
            args.push("--rate".into());
            args.push(num.to_string().into());
        }

        if let Some(scale) = &self.scale_bench {
            args.push("--scale".into());
            args.push(scale.to_string().into());
        }

        if let Some(num) = &self.transactions {
            args.push("--transactions".into());
            args.push(num.to_string().into());
        }

        if let Some(num) = &self.time {
            args.push("--time".into());
            args.push(num.to_string().into());
        }

        if self.vacuum_all {
            args.push("--vacuum-all".into());
        }

        if let Some(num) = &self.aggregate_interval {
            args.push("--aggregate-interval".into());
            args.push(num.to_string().into());
        }

        if self.failures_detailed {
            args.push("--failures-detailed".into());
        }

        if let Some(prefix) = &self.log_prefix {
            args.push("--log-prefix".into());
            args.push(prefix.into());
        }

        if let Some(num) = &self.max_tries {
            args.push("--max-tries".into());
            args.push(num.to_string().into());
        }

        if self.progress_timestamp {
            args.push("--progress-timestamp".into());
        }

        if let Some(seed) = &self.random_seed {
            args.push("--random-seed".into());
            args.push(seed.into());
        }

        if let Some(rate) = &self.sampling_rate {
            args.push("--sampling-rate".into());
            args.push(rate.to_string().into());
        }

        if let Some(name) = &self.show_script {
            args.push("--show-script".into());
            args.push(name.into());
        }

        if self.verbose_errors {
            args.push("--verbose-errors".into());
        }

        if self.debug {
            args.push("--debug".into());
        }

        if let Some(hostname) = &self.host {
            args.push("--host".into());
            args.push(hostname.into());
        }

        if let Some(port) = &self.port {
            args.push("--port".into());
            args.push(port.to_string().into());
        }

        if let Some(username) = &self.username {
            args.push("--username".into());
            args.push(username.into());
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
        let command = PgBenchBuilder::new().program_dir(".").build();
        assert_eq!(
            PathBuf::from(".").join("pgbench"),
            PathBuf::from(command.to_command_string().replace('"', ""))
        );
    }

    #[test]
    fn test_builder_from() {
        let command = PgBenchBuilder::from(&TestSettings).build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#""./pgbench" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = r#"".\\pgbench" "#;

        assert_eq!(
            format!(
                r#"{command_prefix}"--host" "localhost" "--port" "5432" "--username" "postgres""#
            ),
            command.to_command_string()
        );
    }

    #[test]
    fn test_builder() {
        let command = PgBenchBuilder::new()
            .env("PGDATABASE", "database")
            .initialize()
            .init_steps("steps")
            .fill_factor(10)
            .no_vacuum()
            .quiet()
            .scale(10)
            .foreign_keys()
            .index_tablespace("tablespace")
            .partition_method("method")
            .partitions(10)
            .tablespace("tablespace")
            .unlogged_tables()
            .builtin("name")
            .file("filename")
            .skip_some_updates()
            .select_only()
            .client(10)
            .connect()
            .define("var")
            .jobs(10)
            .log()
            .latency_limit(10)
            .protocol("protocol")
            .no_vacuum_bench()
            .progress(10)
            .report_per_command()
            .rate(10)
            .scale_bench(10)
            .transactions(10)
            .time(10)
            .vacuum_all()
            .aggregate_interval(10)
            .failures_detailed()
            .log_prefix("prefix")
            .max_tries(10)
            .progress_timestamp()
            .random_seed("seed")
            .sampling_rate(10.0)
            .show_script("name")
            .verbose_errors()
            .debug()
            .host("localhost")
            .port(5432)
            .username("username")
            .version()
            .help()
            .build();
        #[cfg(not(target_os = "windows"))]
        let command_prefix = r#"PGDATABASE="database" "#;
        #[cfg(target_os = "windows")]
        let command_prefix = String::new();

        assert_eq!(
            format!(
                r#"{command_prefix}"pgbench" "--initialize" "--init-steps" "steps" "--fillfactor" "10" "--no-vacuum" "--quiet" "--scale" "10" "--foreign-keys" "--index-tablespace" "tablespace" "--partition-method" "method" "--partitions" "10" "--tablespace" "tablespace" "--unlogged-tables" "--builtin" "name" "--file" "filename" "--skip-some-updates" "--select-only" "--client" "10" "--connect" "--define" "var" "--jobs" "10" "--log" "--latency-limit" "10" "--protocol" "protocol" "--no-vacuum" "--progress" "10" "--report-per-command" "--rate" "10" "--scale" "10" "--transactions" "10" "--time" "10" "--vacuum-all" "--aggregate-interval" "10" "--failures-detailed" "--log-prefix" "prefix" "--max-tries" "10" "--progress-timestamp" "--random-seed" "seed" "--sampling-rate" "10" "--show-script" "name" "--verbose-errors" "--debug" "--host" "localhost" "--port" "5432" "--username" "username" "--version" "--help""#
            ),
            command.to_command_string()
        );
    }
}
