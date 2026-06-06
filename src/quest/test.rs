use serde::{Serialize, Deserialize};
use std::process::Command;
use crate::quest::game::WORKSPACE_DIR;
use crate::quest::errors::print_error_hints;

#[derive(Serialize, Deserialize, Clone)]
pub enum TestKind {
    Compile,
    Output,
    Behavior,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Test {
    pub kind: TestKind,
    pub expected: String,
}

pub fn run_test(test: &Test) -> Result<(), anyhow::Error> {
    match &test.kind {
        TestKind::Compile => run_compile_test(test),
        TestKind::Output => run_output_test(test),
        TestKind::Behavior => run_behavior_test(test),
    }
}

fn run_compile_test(_test: &Test) -> Result<(), anyhow::Error> {
    let output = Command::new("cargo")
        .arg("check")
        .current_dir(WORKSPACE_DIR)
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        print_error_hints(String::from_utf8_lossy(&output.stderr).as_ref());
        Err(anyhow::anyhow!("Compile test failed"))
    }
}

fn run_output_test(test: &Test) -> Result<(), anyhow::Error> {
    let output = Command::new("cargo")
        .arg("run")
        .current_dir(WORKSPACE_DIR)
        .output()?;

    if !output.status.success() {
        print_error_hints(String::from_utf8_lossy(&output.stderr).as_ref());
        return Err(anyhow::anyhow!("Output test failed"));
    }

    let actual = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if actual != test.expected.trim() {
        return Err(anyhow::anyhow!("Output test failed: expected {}, got {}", test.expected, actual));
    }

    Ok(())
}

fn run_behavior_test(test: &Test) -> Result<(), anyhow::Error> {
    let output = Command::new("cargo")
        .arg("test")
        .current_dir(WORKSPACE_DIR)
        .output()?;

    if !output.status.success() {
        print_error_hints(String::from_utf8_lossy(&output.stderr).as_ref());
        return Err(anyhow::anyhow!("Behavior test failed"));
    }

    let actual = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if actual != test.expected {
        return Err(anyhow::anyhow!("Behavior test failed: expected {}, got {}", test.expected, actual));
    }

    Ok(())
}