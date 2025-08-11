use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU32, Ordering};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::{Command, ChildStdout, ChildStderr},
};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Event, Listener};
use tauri::ipc::Channel;
use tokio::io::AsyncWriteExt;

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

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "event")]
pub(crate) struct FrontendCommandArg {
    command: Option<String>,
    frontend: bool
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

pub(crate) fn format_commands(username: &str, commands: &Vec<String>) -> String {
    let mut formatted_commands = format!("echo ---STAGE1-{}-COMPLETE--", username);
    for (i, command) in commands.iter().enumerate() {
        formatted_commands += &format!(" && {}", command);
        formatted_commands += &format!(" && echo ---STAGE{}-{}-COMPLETE--", i + 2, username);
    }
    formatted_commands
}

#[tauri::command]
pub async fn run_ssh_flow(app: AppHandle, username: String, commands: Vec<FrontendCommandArg>, on_event: Channel<ProcessEvent>) -> Result<(), String> {
    let mut parsed_commands: Vec<String> = vec![];
    let mut max_num_stdin: u32 = 0;
    let mut written_num_stdin: Arc<AtomicU32> = Arc::new(AtomicU32::new(0));
    for command in commands {
        let command = command.clone();
        if let Some(c) = command.command {
            parsed_commands.push(c);
        } else {
            if command.frontend { parsed_commands.push("read".to_string()); max_num_stdin += 1; }
        }
    }

    let commands = parsed_commands;

    let mut child = Arc::new(tokio::sync::Mutex::new(Command::new("ssh")
        .arg(format!("{}@hackclub.app", username))
        .arg(format_commands(&username, &commands))
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Unable to spawn SSH")
    ));

    let mut child_lock = child.lock().await;
    on_event.send(ProcessEvent::Started {
        command: format_commands(&username, &commands),
        process_id: child_lock.id().expect("Unable to get process ID")
    }).unwrap();
    let stdout: ChildStdout = child_lock.stdout.take().expect("Was unable to get output from SSH");
    let stderr: ChildStderr = child_lock.stderr.take().expect("Was unable to get output from SSH");
    let stdin = child_lock.stdin.take().expect("Was unable to get input from SSH");
    drop(child_lock); // release lock

    let on_event_stdout = on_event.clone();
    let on_event_stderr = on_event.clone();
    let app_handle_stdin = app.clone();

    let current_stage: Arc<tokio::sync::Mutex<u32>> = Arc::new(tokio::sync::Mutex::new(0u32));
    let current_stage_stdout = current_stage.clone();

    let stdout_handler = tokio::spawn(async move {
        let stdout = BufReader::new(stdout);
        let mut lines = stdout.lines();
        while let Some(line) = lines.next_line().await.expect("Unable to read line from stdout") {
            println!("{:?}", line);
            let mut current_stage = *current_stage_stdout.lock().await;
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

    let written_num_stdin_clone = written_num_stdin.clone();
    let stdin = Arc::new(tokio::sync::Mutex::new(stdin));
    let app_handle_stdin_2 = app_handle_stdin.clone();
    let stdin_handler = tokio::spawn(async move {
        let stdin = stdin.clone();
        let app_handle_stdin_3 = app_handle_stdin_2.clone();
        let eventid = app_handle_stdin_2.listen("ready_to_move_on",  move |event: Event| {
            let stdin = stdin.clone();
            let written_num_stdin = written_num_stdin_clone.clone();
            let app_handle_stdin_4 = app_handle_stdin_3.clone();
            tauri::async_runtime::spawn(async move {
                let mut stdin = stdin.lock().await;
                stdin.write_all(b"\n").await.expect("Failed to write to stdin");
                let count = written_num_stdin.fetch_add(1, Ordering::SeqCst) + 1;
                if count >= max_num_stdin {
                    println!("All stdin writes done");
                    app_handle_stdin_4.unlisten(event.id());
                }
            });
        });
        if max_num_stdin == 0 {
            app_handle_stdin_2.unlisten(eventid);
        }
    });

    if max_num_stdin == 0 {
        drop(stdin_handler);
    }

    let app_handle_stdin_5 = app_handle_stdin.clone();
    let frontend_error_child = child.clone();
    let kill_if_frontend_error = app.listen("error_on_the_frontend", move |event| {
        let frontend_error_child = frontend_error_child.clone();
        let app_handle_stdin_5 = app_handle_stdin_5.clone();
        let event_id = event.id();
        tauri::async_runtime::spawn(async move {
            let mut child = frontend_error_child.lock().await;
            child.start_kill().expect("did not quit ssh after error on the frontend");
            app_handle_stdin_5.unlisten(event_id);
        });
    });

    let mut new_child_lock = child.lock().await;
    let status = new_child_lock.wait().await.unwrap();
    let current_stage = *current_stage.lock().await;
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
    app.unlisten(kill_if_frontend_error);

    Ok(())
}