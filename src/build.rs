use std::ffi;
use std::process;

use crate::cargo::Cargo;
use crate::cargo::CURRENT_TARGET;
use crate::error::*;
use crate::msg::*;
use crate::run::CargoRun;
#[cfg(feature = "test_unstable")]
use crate::test::CargoTest;

/// The `build` subcommand.
///
/// # Example
///
/// ```rust
/// extern crate escargot;
/// extern crate assert_fs;
///
/// let temp = assert_fs::TempDir::new().unwrap();
/// escargot::CargoBuild::new()
///     .bin("bin")
///     .current_release()
///     .current_target()
///     .manifest_path("tests/fixtures/bin/Cargo.toml")
///     .target_dir(temp.path())
///     .exec()
///     .unwrap();
/// ```
pub struct CargoBuild {
    cmd: process::Command,
    bin: bool,
    example: bool,
}

impl CargoBuild {
    /// Shortcut to create a `build` subcommand.
    ///
    /// See also [`Cargo`].
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate escargot;
    /// extern crate assert_fs;
    ///
    /// let temp = assert_fs::TempDir::new().unwrap();
    /// escargot::CargoBuild::new()
    ///     .bin("bin")
    ///     .manifest_path("tests/fixtures/bin/Cargo.toml")
    ///     .target_dir(temp.path())
    ///     .exec()
    ///     .unwrap();
    /// ```
    ///
    pub fn new() -> Self {
        Cargo::new().build()
    }

    pub(crate) fn with_command(cmd: process::Command) -> Self {
        Self {
            cmd,
            bin: false,
            example: false,
        }
    }

    /// Build from `name` package in workspaces.
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate escargot;
    /// extern crate assert_fs;
    ///
    /// let temp = assert_fs::TempDir::new().unwrap();
    /// escargot::CargoBuild::new()
    ///     .package("bin")
    ///     .bin("bin")
    ///     .manifest_path("tests/fixtures/bin/Cargo.toml")
    ///     .target_dir(temp.path())
    ///     .exec()
    ///     .unwrap();
    /// ```
    pub fn package<S: AsRef<ffi::OsStr>>(self, name: S) -> Self {
        self.arg("--package").arg(name)
    }
    /// Build only `name` binary.
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate escargot;
    /// extern crate assert_fs;
    ///
    /// let temp = assert_fs::TempDir::new().unwrap();
    /// escargot::CargoBuild::new()
    ///     .bin("bin")
    ///     .manifest_path("tests/fixtures/bin/Cargo.toml")
    ///     .target_dir(temp.path())
    ///     .exec()
    ///     .unwrap();
    /// ```
    pub fn bin<S: AsRef<ffi::OsStr>>(mut self, name: S) -> Self {
        self.bin = true;
        self.arg("--bin").arg(name)
    }

    /// Build only `name` example.
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate escargot;
    /// extern crate assert_fs;
    ///
    /// let temp = assert_fs::TempDir::new().unwrap();
    /// escargot::CargoBuild::new()
    ///     .example("example_fixture")
    ///     .manifest_path("tests/fixtures/example/Cargo.toml")
    ///     .target_dir(temp.path())
    ///     .exec()
    ///     .unwrap();
    /// ```
    pub fn example<S: AsRef<ffi::OsStr>>(mut self, name: S) -> Self {
        self.example = true;
        self.arg("--example").arg(name)
    }

    /// Build all tests
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate escargot;
    /// extern crate assert_fs;
    ///
    /// let temp = assert_fs::TempDir::new().unwrap();
    /// escargot::CargoBuild::new()
    ///     .tests()
    ///     .manifest_path("tests/fixtures/test/Cargo.toml")
    ///     .target_dir(temp.path())
    ///     .exec()
    ///     .unwrap();
    /// ```
    pub fn tests(self) -> Self {
        self.arg("--tests")
    }

    /// Build only `name` test.
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate escargot;
    /// extern crate assert_fs;
    ///
    /// let temp = assert_fs::TempDir::new().unwrap();
    /// escargot::CargoBuild::new()
    ///     .test("test")
    ///     .manifest_path("tests/fixtures/test/Cargo.toml")
    ///     .target_dir(temp.path())
    ///     .exec()
    ///     .unwrap();
    /// ```
    pub fn test<S: AsRef<ffi::OsStr>>(self, name: S) -> Self {
        self.arg("--test").arg(name)
    }

    /// Path to Cargo.toml
    pub fn manifest_path<S: AsRef<ffi::OsStr>>(self, path: S) -> Self {
        self.arg("--manifest-path").arg(path)
    }

    /// Build artifacts in release mode, with optimizations.
    pub fn release(self) -> Self {
        self.arg("--release")
    }

    /// Build artifacts in release mode if the current process has, with optimizations.
    #[cfg(debug_assertions)]
    pub fn current_release(self) -> Self {
        self
    }

    /// Build artifacts in release mode if the current process has, with optimizations.
    #[cfg(not(debug_assertions))]
    pub fn current_release(self) -> Self {
        self.release()
    }

    /// Build for the target triplet.
    pub fn target<S: AsRef<ffi::OsStr>>(self, triplet: S) -> Self {
        self.arg("--target").arg(triplet)
    }

    /// Build for the current process' triplet.
    pub fn current_target(self) -> Self {
        self.target(CURRENT_TARGET)
    }

    /// Directory for all generated artifacts
    pub fn target_dir<S: AsRef<ffi::OsStr>>(self, dir: S) -> Self {
        self.arg("--target-dir").arg(dir)
    }

    /// Activate all available features
    pub fn all_features(self) -> Self {
        self.arg("--all-features")
    }

    /// Do not activate the `default` feature
    pub fn no_default_features(self) -> Self {
        self.arg("--no-default-features")
    }

    /// Space-separated list of features to activate
    pub fn features<S: AsRef<ffi::OsStr>>(self, features: S) -> Self {
        self.arg("--features").arg(features)
    }

    /// Manually pass an argument that is unsupported.
    ///
    /// Caution: Passing in `--` can throw off the API.
    pub fn arg<S: AsRef<ffi::OsStr>>(mut self, arg: S) -> Self {
        self.cmd.arg(arg);
        self
    }

    /// Build the configured target, returning compiler messages.
    pub fn exec(self) -> CargoResult<CommandMessages> {
        CommandMessages::with_command(self.cmd)
    }

    /// Provide a proxy for running the built target.
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate escargot;
    /// extern crate assert_fs;
    ///
    /// let temp = assert_fs::TempDir::new().unwrap();
    /// let run = escargot::CargoBuild::new()
    ///     .bin("bin")
    ///     .current_release()
    ///     .current_target()
    ///     .manifest_path("tests/fixtures/bin/Cargo.toml")
    ///     .target_dir(temp.path())
    ///     .run()
    ///     .unwrap();
    /// println!("artifact={}", run.path().display());
    /// ```
    pub fn run(self) -> CargoResult<CargoRun> {
        let msgs = CommandMessages::with_command(self.cmd)?;
        CargoRun::from_message(msgs, self.bin, self.example)
    }

    /// Provide a proxy for running the built target.
    ///
    /// Required feature: `test_unstable` since the format parsed is unstable.
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate escargot;
    /// extern crate assert_fs;
    ///
    /// let temp = assert_fs::TempDir::new().unwrap();
    /// let run = escargot::CargoBuild::new()
    ///     .test("test")
    ///     .current_release()
    ///     .current_target()
    ///     .manifest_path("tests/fixtures/test/Cargo.toml")
    ///     .target_dir(temp.path())
    ///     .run_tests().unwrap()
    ///     .next().unwrap().unwrap();
    /// println!("artifact={}", run.path().display());
    /// ```
    #[cfg(feature = "test_unstable")]
    pub fn run_tests(self) -> CargoResult<impl Iterator<Item = Result<CargoTest, CargoError>>> {
        let msgs = CommandMessages::with_command(self.cmd)?;
        Ok(CargoTest::with_messages(msgs))
    }
}

impl Default for CargoBuild {
    fn default() -> Self {
        Self::new()
    }
}
