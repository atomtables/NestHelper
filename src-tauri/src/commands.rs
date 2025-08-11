use std::sync::{Arc, Mutex};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::{Command, ChildStdout, ChildStderr},
};
use serde::Serialize;
use tauri::AppHandle;
use tauri::ipc::Channel;

#[tauri::command]
pub fn helloworld() -> String {
    println!("Hello, world!");
    "Hello, world!".into()
}

// #[tauri::command]
// pub async fn verifyssh(username: String) -> Result<bool, String> {
//     let needed = Command::new("ssh")
//         .arg(format!("{}@hackclub.app", username))
//         .args(["-o", "LogLevel=ERROR"])
//         .args(["-C", "uname -a"])
//         .output();
//     println!("{:?}", needed);
//     if needed.is_err() {
//         return Err(format!(
//             "Failed to run command: {}",
//             needed.err().unwrap().to_string()
//         ));
//     };
//     let output = needed.ok().unwrap();
//     println!("{:?}", output);
//     if output.status.code() != Some(0) {
//         return Err(output
//             .stderr
//             .into_iter()
//             .map(|c| c as char)
//             .collect::<String>());
//     }
//     Ok(true)
// }

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "event", content = "data")]
pub(crate) enum ProcessEvent {
    Started {
        command: String,
        process_id: u32
    },
    Output {
        file: String, // as in stdout vs stderr
        output: String
    },
    NextStage {
        stage: u32,
        file: String,
        output: String
    },
    Error {
        stage: u32,
        return_code: i32
    },
    Finished {
        complete_output: String,
        return_code: i32
    },
}

pub(crate) fn parse_output(username: &str, output: &str, current: u32) -> (bool, u32) {
    if output.starts_with("---STAGE") && output.ends_with(format!("-{}-COMPLETE--", username).as_str()) {
        if let Some(stage_str) = output.split("--STAGE").nth(1) {
            if let Some(num_str) = stage_str.chars().next() {
                if let Some(stage_num) = num_str.to_digit(10) {
                    return (true, stage_num);
                }
            }
        }
    }
    (false, current)
}

fn format_commands(username: &str, commands: &Vec<String>) -> String {
    let mut formatted_commands = format!("echo ---STAGE1-{}-COMPLETE--", username);
    for (i, command) in commands.iter().enumerate() {
        formatted_commands += &format!(" && {}", command);
        formatted_commands += &format!(" && echo ---STAGE{}-{}-COMPLETE--", i + 2, username);
    }
    formatted_commands
}

#[tauri::command]
pub async fn run_ssh_flow(app: AppHandle, username: String, commands: Vec<String>, on_event: Channel<ProcessEvent>) -> Result<(), String> {
    let mut child = Command::new("ssh")
        .arg(format!("{}@hackclub.app", username))
        .arg(format_commands(&username, &commands))
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn().expect("Unable to spawn in an instance of SSH");

    on_event.send(ProcessEvent::Started {
        command: format_commands(&username, &commands),
        process_id: child.id().expect("Unable to get process ID")
    }).unwrap();

    let stdout: ChildStdout = child.stdout.take().expect("Was unable to get output from SSH");
    let stderr: ChildStderr = child.stderr.take().expect("Was unable to get output from SSH");
    let on_event_stdout = on_event.clone();
    let on_event_stderr = on_event.clone();

    let current_stage: Arc<Mutex<u32>> = Arc::new(Mutex::new(0u32));
    let current_stage_stdout = current_stage.clone();

    let stdout_handler = tokio::spawn(async move {
        let stdout = BufReader::new(stdout);
        let mut lines = stdout.lines();
        while let Some(line) = lines.next_line().await.expect("Unable to read line from stdout") {
            println!("{:?}", line);
            let mut current_stage = *current_stage_stdout.lock().unwrap();
            let parsed = parse_output(&username, &line, current_stage);
            if parsed.0 {
                current_stage = parsed.1;
                on_event_stdout.send(ProcessEvent::NextStage {
                    stage: current_stage,
                    file: "stdout".to_string(),
                    output: line
                }).unwrap();
            } else {
                on_event_stdout.send(ProcessEvent::Output {
                    file: "stdout".to_string(),
                    output: line
                }).unwrap();
            }
        }
    });

    let stderr_handler = tokio::spawn(async move {
        let stderr = BufReader::new(stderr);
        let mut lines = stderr.lines();
        while let Some(line) = lines.next_line().await.expect("Unable to read line from stderr") {
            println!("{:?}", line);
            on_event_stderr.send(ProcessEvent::Output {
                file: "stderr".to_string(),
                output: line
            }).unwrap();
        }
    });

    let status = child.wait().await.unwrap();
    let current_stage = *current_stage.lock().unwrap();
    if !status.success() {
        on_event.send(ProcessEvent::Error {
            stage: current_stage,
            return_code: status.code().unwrap_or(-1)
        }).unwrap();
        return Err(format!("Process exited with code: {} at stage {}.", status.code().unwrap_or(-1), current_stage));
    }
    on_event.send(ProcessEvent::Finished {
        complete_output: "".to_string(),
        return_code: status.code().unwrap()
    }).unwrap();

    println!("Process finished with code: {}", status.code().unwrap());
    stdout_handler.await.expect("stdout handler failed");
    stderr_handler.await.expect("stderr handler failed");

    Ok(())
}