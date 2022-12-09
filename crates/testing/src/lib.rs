//! Utilities for testing bindings generated by Wasmer Pack.
//!
//! Typical.
//!
//! ```rust,no_run
//! # use wasmer_pack_testing::TestEnvironment;
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let env = TestEnvironment::for_crate("./path/to/Cargo.toml")?;
//! env.python("./my_tests.py")?;
//! env.javascript("./my_test.js")?;
//! env.typescript("./my_tests.ts")?;
//! # Ok(())
//! # }
//! ```
//!
//! Under the hood, this will use `cargo wapm` to compile a Rust crate to
//! WebAssembly and turn it into a WAPM package.
//!
//! You can tell it to run test scripts written in various languages.
//!
//! The [`TestEnvironment::python()`] method will create a Virtual Environment
//! in the script's directory and install the generated Python library. The
//! provided test script will then be run in that environment using
//! [py.test][pytest].
//!
//! The [`TestEnvironment::javascript()`] and [`TestEnvironment::typescript()`]
//! methods will generate JavaScript bindings for the Rust crate and use
//! `yarn link` to add them as a dependency. From there, the test script will
//! be run using [Jest][jest].
//!
//! [pytest]: https://docs.pytest.org/
//! [jest]: https://jestjs.io/

mod errors;
mod javascript;
mod python;
mod utils;

pub use crate::errors::{CommandFailed, LoadError, TestFailure};

use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct TestEnvironment {
    temp_dir: PathBuf,
    wapm_dir: PathBuf,
}

impl TestEnvironment {
    pub fn for_crate(
        manifest_path: impl AsRef<Path>,
        temp_dir: impl AsRef<Path>,
    ) -> Result<Self, LoadError> {
        let manifest_path = manifest_path.as_ref();
        let temp_dir = temp_dir.as_ref();

        let wapm_dir = utils::compile_rust_to_wapm_package(manifest_path, temp_dir.join("target"))?;

        Ok(TestEnvironment {
            temp_dir: temp_dir.to_path_buf(),
            wapm_dir,
        })
    }

    pub fn python(&self, script_path: impl AsRef<Path>) -> Result<(), TestFailure> {
        python::run(script_path.as_ref(), &self.wapm_dir, &self.temp_dir)
    }

    pub fn javascript(&self, script_path: impl AsRef<Path>) -> Result<(), TestFailure> {
        javascript::run(script_path.as_ref(), &self.wapm_dir, &self.temp_dir)
    }

    pub fn typescript(&self, _script_path: impl AsRef<Path>) -> Result<(), TestFailure> {
        todo!();
    }
}
