use std::process::Command;

fn main() {
    // 获取Git提交ID
    let commit_id = get_git_commit_id();
    println!("cargo:rustc-env=GIT_COMMIT_ID={}", commit_id);

    // 获取Git分支
    let branch = get_git_branch();
    println!("cargo:rustc-env=GIT_BRANCH={}", branch);

    // 获取构建时间
    let build_time = chrono::Local::now().to_rfc3339();
    println!("cargo:rustc-env=BUILD_TIME={}", build_time);

    // 获取构建者信息
    let build_user = whoami::username();
    println!("cargo:rustc-env=BUILD_USER={}", build_user);
}

fn get_git_commit_id() -> String {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok();

    if let Some(output) = output {
        if output.status.success() {
            return String::from_utf8_lossy(&output.stdout).trim().to_string();
        }
    }

    "unknown".to_string()
}

fn get_git_branch() -> String {
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok();

    if let Some(output) = output {
        if output.status.success() {
            return String::from_utf8_lossy(&output.stdout).trim().to_string();
        }
    }

    "unknown".to_string()
}
