use std::{
    borrow::Cow,
    env::current_dir,
    ffi::OsStr,
    fmt::Debug,
    fs::File,
    io::{Read, Write},
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    sync::Arc,
    time::{Duration, Instant},
};

use clap::Parser;
use color_eyre::{
    eyre::{self, bail, Context, ContextCompat},
    owo_colors::OwoColorize,
    Section,
};
use inquire::prompt_confirmation;
use reqwest::blocking::{Client, ClientBuilder};
use reqwest::cookie::Jar;

use config::{DeserFromStr, Output, Solution};

mod config;

pub trait InputProvider: Debug {
    fn fetch_input(&self, year: u16, day: u8) -> eyre::Result<Option<String>>;
    fn save_input(&mut self, year: u16, day: u8, input: &str) -> eyre::Result<bool>;
    fn clear_inputs(&mut self) -> eyre::Result<()>;
}

#[derive(Debug, Clone)]
pub struct FilesystemInputProvider {
    input_path: PathBuf,
}
impl FilesystemInputProvider {
    pub fn new(input_path: PathBuf) -> Self {
        Self { input_path }
    }
}
fn input_local_path(base_path: &Path, year: u16, day: u8) -> PathBuf {
    let mut path = base_path.to_path_buf();
    path.push(year.to_string());
    path.push(format!("{day}.txt"));
    path
}
impl InputProvider for FilesystemInputProvider {
    fn fetch_input(&self, year: u16, day: u8) -> eyre::Result<Option<String>> {
        let path = input_local_path(&self.input_path, year, day);
        let Ok(mut file) = File::open(path) else {
            return Ok(None);
        };
        let mut buf = String::with_capacity(4096);
        file.read_to_string(&mut buf)?;
        Ok(Some(buf))
    }
    fn save_input(&mut self, year: u16, day: u8, input: &str) -> eyre::Result<bool> {
        let path = input_local_path(&self.input_path, year, day);
        let dir = path.as_path().as_os_str().as_bytes();
        let end = dir
            .iter()
            .rposition(|&b| b == std::path::MAIN_SEPARATOR as u8)
            .unwrap_or(0);
        let dir = &dir[..end];
        let dir = Path::new(OsStr::from_bytes(dir));
        if !dir.exists() {
            if prompt_confirmation(format!(
                "The input folder at {dir:?} does not exist. Do you want to create it?"
            ))? {
                std::fs::create_dir_all(dir)?;
            }
        }
        let mut file = File::create(path)?;
        file.write_all(input.as_bytes())?;
        Ok(true)
    }
    fn clear_inputs(&mut self) -> eyre::Result<()> {
        let Ok(dir) = self.input_path.read_dir() else {
            return Ok(());
        };
        for path in dir {
            let path = path?.path();
            _ = std::fs::remove_dir_all(path);
        }
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct NetworkInputProvider {
    client: Client,
}
#[derive(Debug)]
pub struct MultipleInputProvider {
    providers: Vec<Box<dyn InputProvider>>,
}
impl MultipleInputProvider {
    pub fn new() -> Self {
        Self { providers: vec![] }
    }
    pub fn push(&mut self, provider: Box<dyn InputProvider>) {
        self.providers.push(provider)
    }
}
impl InputProvider for MultipleInputProvider {
    fn fetch_input(&self, year: u16, day: u8) -> eyre::Result<Option<String>> {
        let mut last_error =
            eyre::eyre!("No providers could find this task").suggestion("Did you set AOC_TOKEN?");
        for provider in &self.providers {
            match provider.fetch_input(year, day) {
                Ok(Some(input)) => return Ok(Some(input)),
                Ok(None) => (),
                Err(err) => last_error = err,
            };
        }
        Err(last_error)
    }
    fn save_input(&mut self, year: u16, day: u8, input: &str) -> eyre::Result<bool> {
        let mut out = Err(eyre::eyre!(""));
        for provider in &mut self.providers {
            match provider.save_input(year, day, input) {
                Ok(true) => out = Ok(true),
                Ok(false) => {
                    if out.is_err() {
                        out = Ok(false)
                    }
                }
                Err(err) => out = out.wrap_err(err),
            };
        }
        out
    }
    fn clear_inputs(&mut self) -> eyre::Result<()> {
        for provider in &mut self.providers {
            provider.clear_inputs()?;
        }
        Ok(())
    }
}
impl NetworkInputProvider {
    pub fn new(session: &str, timeout: Duration) -> Self {
        let jar = Jar::default();
        let jar = Arc::new(jar);
        jar.add_cookie_str(
            &format!("session={session}"),
            &"https://adventofcode.com".parse().unwrap(),
        );
        let client = ClientBuilder::new()
            .timeout(timeout)
            .cookie_provider(jar)
            .gzip(true)
            .brotli(true)
            .build()
            .unwrap();
        Self { client }
    }
}
impl InputProvider for NetworkInputProvider {
    fn save_input(&mut self, _year: u16, _day: u8, _input: &str) -> eyre::Result<bool> {
        Ok(false)
    }
    fn clear_inputs(&mut self) -> eyre::Result<()> {
        Ok(())
    }
    fn fetch_input(&self, year: u16, day: u8) -> eyre::Result<Option<String>> {
        let addr = format!("https://adventofcode.com/{year}/day/{day}/input");
        Ok(Some(
            self.client
                .get(&addr)
                .send()
                .wrap_err_with(|| format!("Failed to fetch input from {addr}"))
                .suggestion("Check if your internet connection seems fine")?
                .error_for_status()
                .wrap_err("The AoC server returned an error")
                .suggestion("Maybe your AOC session token is invalid?")?
                .text()
                .wrap_err("Failed to decode the AoC server's response as text")?,
        ))
    }
}

fn main() -> eyre::Result<()> {
    _ = dotenvy::from_path(".env");
    color_eyre::install()?;

    let args = config::Arguments::parse();

    let mut config = File::open(&args.config)
        .wrap_err_with(|| format!("Couldn't open the config file {file:?}", file = args.config))?;
    let mut buf = String::with_capacity(4096);
    config.read_to_string(&mut buf).wrap_err_with(|| {
        format!(
            "Failed to read the config file {file:?}",
            file = args.config
        )
    })?;
    let mut config: config::Config = toml::from_str(&buf).unwrap();
    let Some(mut year) = config.solutions.remove(&args.year) else {
        panic!("No solutions defined for {year}", year = args.year)
    };
    let Some(mut day) = year.remove(&args.day) else {
        panic!(
            "No solutions defined for {year} day {day}",
            year = args.year,
            day = args.day
        )
    };
    if let Some(part) = args.part {
        day.retain(|p, _| **p == part);
    }
    if day.is_empty() {
        panic!(
            "No solutions defined for {year} day {day}{part}",
            year = args.year,
            day = args.day,
            part = args.part.map(|p| format!(" part {p}")).unwrap_or_default()
        );
    }
    if !args.bench.is_empty() {
        day.iter_mut()
            .map(|(_, solutions)| solutions)
            .for_each(|sol| {
                sol.retain(|sol| {
                    args.bench
                        .iter()
                        .any(|enabled| enabled.eq_ignore_ascii_case(&sol.name))
                })
            });
        day.retain(|_, solutions| !solutions.is_empty());
    }
    let to_run = day;
    if to_run.is_empty() {
        panic!(
            "No solutions selected with -b for {year} day {day}{part}",
            year = args.year,
            day = args.day,
            part = args.part.map(|p| format!(" part {p}")).unwrap_or_default()
        );
    }

    let (year, day) = (args.year, args.day);
    if args.dry {
        for (DeserFromStr(part), solutions) in to_run {
            println!("Executing {year} day {day} part {part}");
            for sol in solutions {
                print_solution(&sol.name, &sol.language, sol.description.as_deref());
            }
        }
        return Ok(());
    }

    let mut input_provider = MultipleInputProvider::new();
    let filesystem_provider = FilesystemInputProvider::new(config.input_path.clone());
    input_provider.push(Box::new(filesystem_provider));
    if let Some(aoc_token) = args.aoc_token.as_ref() {
        let network_provider = NetworkInputProvider::new(aoc_token, config.req_timeout.clone());
        input_provider.push(Box::new(network_provider));
    }
    let mut state = State {
        input_provider: Box::new(input_provider),
        clean: args.clean,
        command_timeout: config.command_timeout,
    };
    for (DeserFromStr(part), solutions) in to_run {
        println!("Executing {year} day {day} part {part}");
        for sol in solutions {
            print_solution(&sol.name, &sol.language, sol.description.as_deref());
            match run_solution(&mut state, year, day, sol) {
                Ok((runtime, output)) => {
                    println!(
                        "Took {time:?}, output: {out}",
                        time = runtime.yellow(),
                        out = output.green()
                    );
                }
                Err(err) => println!("{} {err:?}", "Failed to run solution:".red().bold()),
            };
        }
    }
    Ok(())
}
fn print_solution(name: &str, language: &str, description: Option<&str>) {
    println!(
        "{name}, {language}",
        name = name,
        language = language.magenta(),
    );
    if let Some(desc) = &description {
        println!("{desc}", desc = desc.blue())
    }
}
#[derive(Debug)]
struct State {
    input_provider: Box<dyn InputProvider>,
    clean: bool,
    command_timeout: Duration,
}

struct PathGuard {
    restore_to: PathBuf,
}
impl Drop for PathGuard {
    fn drop(&mut self) {
        _ = std::env::set_current_dir(&self.restore_to)
    }
}

fn run_solution(
    state: &mut State,
    year: u16,
    day: u8,
    solution: Solution,
) -> eyre::Result<(Duration, String)> {
    let Solution {
        build,
        pre_hook,
        exec,
        post_hook,
        clean_hook,
        shell,
        output,
        time_regex,
        result_regex,
        time_externally,
        path,
        ..
    } = solution;
    let input = state
        .input_provider
        .fetch_input(year, day)
        .wrap_err_with(|| format!("Error fetching an input for {year}/{day}"))?
        .wrap_err_with(|| format!("Failed to fetch an input for {year}/{day}"))?;

    state.input_provider.save_input(year, day, &input)?;

    // Build step
    const DEFAULT_SHELL: &'static [Cow<'static, str>] = if cfg!(target_os = "windows") {
        &[Cow::Borrowed("cmd"), Cow::Borrowed("/C")]
    } else {
        &[Cow::Borrowed("sh"), Cow::Borrowed("-c")]
    };

    let _guard = if let Some(path) = path {
        let restore_to = current_dir().wrap_err(
            "Failed to get current working directory to later restore out directory to it.",
        )?;
        std::env::set_current_dir(&path).wrap_err_with(|| format!("Failed to change the current working directory to {path:?}, as specified in the config.")).suggestion("Make sure the path is valid in the current location")?;
        Some(PathGuard { restore_to })
    } else {
        None
    };
    let shell = shell
        .as_ref()
        .map(|s| s.as_slice())
        .unwrap_or(DEFAULT_SHELL);
    let (shell, args) = shell
        .split_first()
        .wrap_err_with(|| format!("The shell is empty"))
        .suggestion("Set the shell for the solution in your TOML config file")?;

    let cmd = |command: &str| -> eyre::Result<Child> {
        Command::new(shell.as_ref())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .args(args.into_iter().map(|a| a.as_ref()))
            .arg(command)
            .spawn()
            .wrap_err_with(|| format!("Failed to spawn shell {shell:?}"))
    };
    let verbose_on_failure = |command: &str, input: &[u8]| -> eyre::Result<std::process::Output> {
        let start = Instant::now();
        let mut child = cmd(command)?;
        let mut stdin = child
            .stdin
            .take()
            .wrap_err("Failed to get stdin of child")?;
        stdin
            .write_all(input)
            .wrap_err("Failed to write input into the stdin of the solution")?;
        std::mem::drop(stdin);
        let output = loop {
            if start.elapsed() > state.command_timeout {
                bail!(
                    "Command {command:?} timed out. Timeout is set to {timeout:?}",
                    timeout = state.command_timeout
                )
            }
            if child.try_wait()?.is_some() {
                break child.wait_with_output()?;
            }
            std::thread::sleep(Duration::from_millis(10));
        };
        if !output.status.success() {
            bail!(
                "Child process didn't exit with a non-zero status {status}. Command was {command:?}",
                status = output.status
            )
        }
        Ok(output)
    };

    let try_run = |command: Option<&str>| -> eyre::Result<()> {
        if let Some(command) = command {
            verbose_on_failure(&command, b"")?;
        }
        Ok(())
    };
    try_run(build.as_deref()).wrap_err("Error while executing build")?;
    try_run(pre_hook.as_deref()).wrap_err("Error while executing pre_hook")?;
    let (_, shell_overhead) = time(|| cmd(""));
    let (result, runtime) = time(|| {
        verbose_on_failure(&exec, input.as_bytes()).wrap_err("Failed executing main solution")
    });
    let result = result?;

    let output = match output {
        Output::Stdout => result.stdout,
        Output::Stderr => result.stderr,
    };
    let output = String::from_utf8(output).wrap_err("Solution output is invalid UTF-8")?;
    let result = result_regex
        .captures(&output)
        .and_then(|cap| cap.get(1))
        .wrap_err_with(|| format!("Failed to capture the result"))?
        .as_str();
    let runtime = if time_externally {
        // try to compensate for shell overhead
        runtime.saturating_sub(shell_overhead)
    } else {
        let nanos: u64 = time_regex
            .captures(&output)
            .and_then(|cap| cap.get(1))
            .wrap_err_with(|| format!("Failed to capture the timing"))?
            .as_str()
            .parse()
            .unwrap();
        Duration::from_nanos(nanos)
    };

    try_run(post_hook.as_deref()).wrap_err("Error while executing build")?;
    if state.clean {
        try_run(clean_hook.as_deref()).wrap_err("Error while executing pre_hook")?;
    }

    Ok((runtime, result.to_string()))
}
fn time<T, F: FnOnce() -> T>(cb: F) -> (T, Duration) {
    let start = Instant::now();
    let ret = cb();
    (ret, start.elapsed())
}
