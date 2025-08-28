use crate::kill_pid::kill_process_by_pid;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use tauri::ipc::Channel;
use tauri::{AppHandle, Event, Listener};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::{ChildStderr, ChildStdout, Command},
};

// this is all for running an SSH flow.
#[derive(Clone, Serialize)]
#[serde(
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    tag = "event",
    content = "data"
)]
pub(crate) enum ProcessEvent {
    Started {
        command: String,
        process_id: u32,
    },
    Output {
        file: String, // as in stdout vs stderr
        output: String,
    },
    NextStage {
        stage: u32,
        file: String,
        output: String,
    },
    Error {
        stage: u32,
        return_code: i32,
    },
    Finished {
        complete_output: String,
        return_code: i32,
    },
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "event")]
pub(crate) struct FrontendCommandArg {
    command: Option<String>,
    cwd: Option<String>,
    frontend: bool,
    delay: Option<String>
}
pub(crate) fn parse_output(username: &str, output: &str, current: u32) -> (bool, u32) {
    if output.starts_with("---STAGE")
        && output.ends_with(format!("-{}-COMPLETE--", username).as_str())
    {
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
pub(crate) fn format_commands(username: &str, commands: &Vec<FrontendCommandArg>) -> String {
    let mut formatted_commands = format!("CCWD=/home/{} && cd $CCWD && echo && echo ---STAGE1-{}-COMPLETE--", username, username);
    for (i, command) in commands.iter().enumerate() {
        if let Some(cwd) = command.cwd.clone() {
            formatted_commands += &format!(" && CCWD=$(pwd) && cd {}", cwd);
        }
        if let Some(comm) = command.command.clone() {
            formatted_commands += &format!(" && {}", comm);
        }
        if let Some(delay) = command.delay.clone() {
            if (delay.parse::<u32>().is_ok()) {
                formatted_commands += &format!(" && sleep {}", delay);
            }
        }
        formatted_commands += &format!(" && cd $CCWD && echo && echo ---STAGE{}-{}-COMPLETE--", i + 2, username);
    }
    formatted_commands
}
#[tauri::command]
pub async fn run_ssh_flow(
    app: AppHandle,
    username: String,
    commands: Vec<FrontendCommandArg>,
    on_event: Channel<ProcessEvent>,
) -> Result<(), String> {
    let mut parsed_commands: Vec<FrontendCommandArg> = vec![];
    let mut max_num_stdin: u32 = 0;
    let written_num_stdin: Arc<AtomicU32> = Arc::new(AtomicU32::new(0));
    for command in commands {
        let command = command.clone();
        if let Some(_c) = command.command.clone() {
            parsed_commands.push(command);
        } else {
            if command.frontend {
                parsed_commands.push(
                    FrontendCommandArg {
                        command: Some("read error && if [ \"$error\" = \"fail\" ]; then exit 1; fi".to_string()),
                        cwd: None,
                        frontend: true,
                        delay: None
                    }
                );
                max_num_stdin += 1;
            }
        }
    }

    let commands = parsed_commands;

    let child = Arc::new(tokio::sync::Mutex::new(
        Command::new("ssh")
            .arg(format!("{}@hackclub.app", username))
            .arg(format_commands(&username, &commands))
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Unable to spawn SSH"),
    ));
    let pid = {
        let child_lock = child.lock().await;
        child_lock.id().expect("Unable to get process ID")
    };

    let mut child_lock = child.lock().await;
    on_event
        .send(ProcessEvent::Started {
            command: format_commands(&username, &commands),
            process_id: child_lock.id().expect("Unable to get process ID"),
        })
        .unwrap();
    let stdout: ChildStdout = child_lock
        .stdout
        .take()
        .expect("Was unable to get output from SSH");
    let stderr: ChildStderr = child_lock
        .stderr
        .take()
        .expect("Was unable to get output from SSH");
    let stdin = child_lock
        .stdin
        .take()
        .expect("Was unable to get input from SSH");
    drop(child_lock); // release lock

    let on_event_stdout = on_event.clone();
    let on_event_stderr = on_event.clone();
    let app_handle_stdin = app.clone();

    let current_stage: Arc<tokio::sync::Mutex<u32>> = Arc::new(tokio::sync::Mutex::new(0u32));
    let current_stage_stdout = current_stage.clone();

    let stdout_handler = tokio::spawn(async move {
        let stdout = BufReader::new(stdout);
        let mut lines = stdout.lines();
        while let Some(line) = lines
            .next_line()
            .await
            .expect("Unable to read line from stdout")
        {
            println!("{:?}", line);
            let mut current_stage = *current_stage_stdout.lock().await;
            let parsed = parse_output(&username, &line, current_stage);
            println!("Parsed: {:?}, Current Stage: {}", parsed, current_stage);
            if parsed.0 {
                current_stage = parsed.1;
                on_event_stdout
                    .send(ProcessEvent::NextStage {
                        stage: current_stage,
                        file: "stdout".to_string(),
                        output: line.to_string(),
                    })
                    .unwrap();
            } else {
                on_event_stdout
                    .send(ProcessEvent::Output {
                        file: "stdout".to_string(),
                        output: line.to_string(),
                    })
                    .unwrap();
            }
        }
    });
    let stderr_handler = tokio::spawn(async move {
        let stderr = BufReader::new(stderr);
        let mut lines = stderr.lines();
        while let Some(line) = lines
            .next_line()
            .await
            .expect("Unable to read line from stderr")
        {
            println!("{:?}", line);
            on_event_stderr
                .send(ProcessEvent::Output {
                    file: "stderr".to_string(),
                    output: line.to_string(),
                })
                .unwrap();
        }
    });

    let written_num_stdin_clone = written_num_stdin.clone();
    let stdin = Arc::new(tokio::sync::Mutex::new(stdin));
    let stdin_3 = stdin.clone();
    let app_handle_stdin_2 = app_handle_stdin.clone();
    let stdin_handler = tokio::spawn(async move {
        let stdin = stdin_3.clone();
        let app_handle_stdin_3 = app_handle_stdin_2.clone();
        let eventid = app_handle_stdin_2.listen("ready_to_move_on", move |event: Event| {
            let stdin = stdin.clone();
            let written_num_stdin = written_num_stdin_clone.clone();
            let app_handle_stdin_4 = app_handle_stdin_3.clone();
            tauri::async_runtime::spawn(async move {
                let mut stdin = stdin.lock().await;
                stdin
                    .write_all(b"\n")
                    .await
                    .expect("Failed to write to stdin");
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
    let kill_if_frontend_error = app.listen("error_on_the_frontend", move |event| {
        let app_handle_stdin_5 = app_handle_stdin_5.clone();
        let event_id = event.id();
        let stdin_2 = stdin.clone();
        tauri::async_runtime::spawn(async move {
            let mut stdin_2 = stdin_2.lock().await;
            stdin_2
                .write_all(b"fail\n")
                .await
                .expect("Failed to write to stdin");
            app_handle_stdin_5.unlisten(event_id);
        });
    });

    let app1234ilostcountbecauseIgiveup = app.clone();
    let kill_because_i_said_so = app.listen("abort_ssh_flow", move |event| {
        println!("Attempting to kill process with ID: {}", pid);

        if let Err(e) = kill_process_by_pid(pid) {
            eprintln!("Failed to kill process {}: {}", pid, e);
        } else {
            println!("Successfully killed process {}", pid);
        }

        app1234ilostcountbecauseIgiveup.unlisten(event.id());
    });

    let mut new_child_lock = child.lock().await;
    let status = new_child_lock.wait().await.unwrap();
    let current_stage = *current_stage.lock().await;
    app.unlisten(kill_because_i_said_so);

    if !status.success() {
        on_event
            .send(ProcessEvent::Error {
                stage: current_stage,
                return_code: status.code().unwrap_or(-1),
            })
            .unwrap();
        return Err(format!(
            "Process exited with code: {} at stage {}.",
            status.code().unwrap_or(-1),
            current_stage
        ));
    }
    on_event
        .send(ProcessEvent::Finished {
            complete_output: "".to_string(),
            return_code: status.code().unwrap(),
        })
        .unwrap();

    println!("Process finished with code: {}", status.code().unwrap());
    stdout_handler.await.expect("stdout handler failed");
    stderr_handler.await.expect("stderr handler failed");
    app.unlisten(kill_if_frontend_error);

    Ok(())
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "event")]
pub(crate) struct CommandOutput {
    code: u32,
    stdout: String,
    stderr: String,
}
#[tauri::command]
pub async fn run_ssh_command(
    app: AppHandle,
    username: String,
    command: String,
) -> Result<CommandOutput, CommandOutput> {
    let child = Command::new("ssh")
        .arg(format!("{}@hackclub.app", username))
        .arg(command)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .stdin(std::process::Stdio::null())
        .spawn()
        .map_err(|e| CommandOutput {
            code: 255,
            stdout: "".parse().unwrap(),
            stderr: e.to_string(),
        })?;

    let pid = child.id().expect("Failed to get process ID");
    let app1 = app.clone();
    let abort = app.listen("abort_ssh_command", move |event| {
        println!("Attempting to kill process with ID: {}", pid);

        if let Err(e) = kill_process_by_pid(pid) {
            eprintln!("Failed to kill process {}: {}", pid, e);
        } else {
            println!("Successfully killed process {}", pid);
        }

        app1.unlisten(event.id());
    });

    let child = child
        .wait_with_output()
        .await
        .expect("Failed to wait for child process");
    let stdout = String::from_utf8_lossy(&child.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&child.stderr).into_owned();

    app.unlisten(abort);

    if !child.status.success() {
        return Err(CommandOutput {
            code: child.status.code().unwrap_or(0) as u32,
            stdout,
            stderr,
        });
    }

    println!("{}", stdout);
    eprintln!("{}", stderr);

    Ok(CommandOutput {
        code: child.status.code().unwrap_or(0) as u32,
        stdout,
        stderr,
    })
}

#[tauri::command]
pub async fn run_ssh_command_with_stream(
    app: AppHandle,
    username: String,
    command: String,
    on_event: Channel<ProcessEvent>,
) -> Result<(), String> {
    let mut child = Command::new("ssh")
        .arg(format!("{}@hackclub.app", username)).arg(command.clone())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .stdin(std::process::Stdio::piped())
        .spawn().map_err(|e| e.to_string())?;

    let pid = child.id().expect("Failed to get process ID");
    let app1 = app.clone();
    let abort = app.listen("abort_ssh_command", move |event| {
        if let Err(e) = kill_process_by_pid(pid) {
            eprintln!("Failed to kill process {}: {}", pid, e);
        } else {
            println!("Successfully killed process {}", pid);
        }
        app1.unlisten(event.id());
    });
    on_event
        .send(ProcessEvent::Started {
            command: command.clone(),
            process_id: pid,
        })
        .unwrap();

    let stdout = child
        .stdout
        .take()
        .ok_or("Failed to capture standard output")?;
    let stderr = child
        .stderr
        .take()
        .ok_or("Failed to capture standard error")?;

    let on_event_stdout = on_event.clone();
    let stdout_handler = tokio::spawn(async move {
        let stdout = BufReader::new(stdout);
        let mut lines = stdout.lines();
        while let Some(line) = lines
            .next_line()
            .await
            .expect("Unable to read line from stdout")
        {
            on_event_stdout
                .send(ProcessEvent::Output {
                    file: "stdout".to_string(),
                    output: line.to_string(),
                })
                .unwrap();
        }
    });

    let on_event_stderr = on_event.clone();
    let stderr_handler = tokio::spawn(async move {
        let stderr = BufReader::new(stderr);
        let mut lines = stderr.lines();
        while let Some(line) = lines
            .next_line()
            .await
            .expect("Unable to read line from stderr")
        {
            on_event_stderr
                .send(ProcessEvent::Output {
                    file: "stderr".to_string(),
                    output: line.to_string(),
                })
                .unwrap();
        }
    });
    let status = child.wait().await.map_err(|e| e.to_string())?;
    app.unlisten(abort);
    if !status.success() {
        on_event
            .send(ProcessEvent::Error {
                stage: 0,
                return_code: status.code().unwrap_or(-1),
            })
            .unwrap();
        return Err(format!(
            "Process exited with code: {}.",
            status.code().unwrap_or(-1)
        ));
    }
    on_event
        .send(ProcessEvent::Finished {
            complete_output: "".to_string(),
            return_code: status.code().unwrap(),
        })
        .unwrap();

    stdout_handler.await.expect("stdout handler failed");
    stderr_handler.await.expect("stderr handler failed");

    Ok(())
}

#[tauri::command]
pub async fn ssh_edit_file(
    app: AppHandle,
    username: String,
    remote_path: String,
    new_content: Box<[u8]>
) -> Result<(), String> {
    let mut ssh = Command::new("ssh")
        .arg(format!("{}@hackclub.app", username))
        .arg(format!("dd of={}", remote_path))
        .stdin(std::process::Stdio::piped())
        .spawn().expect("Unable to spawn SSH");
    let mut stdin = ssh.stdin.take().expect("Failed to open stdin");
    stdin.write_all(&new_content).await.expect("Failed to write to stdin");
    drop(stdin); // Close stdin to signal EOF
    let output = ssh.wait_with_output().await.expect("Failed to wait on SSH");
    if !output.status.success() {
        return Err(format!(
            "SSH command failed with status: {}. Stderr: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(())
}

// only supports OS with DD, so no windows
// #[tauri::command]
// pub async fn ssh_upload_file(
//     app: AppHandle,
//     username: String,
//     local_path: String,
//     remote_path: String
// ) -> Result<(), String> {
//     let mut dd = std::process::Command::new("dd")
//         .arg(format!("if={}", local_path))
//         .stdout(std::process::Stdio::piped())
//         .spawn().map_err(|err| err.to_string())?;
//     let _ = dd.wait();
//     let mut ssh_output = std::process::Command::new("ssh")
//         .arg("atomtables@hackclub.app")
//         .arg(format!("dd of={}", remote_path))
//         .stdin(std::process::Stdio::from(dd.stdout.take().expect("failed to take")))  // pipe dd's stdout into ssh's stdin
//         .spawn().map_err(|err| err.to_string())?;
//     let status = ssh_output.wait().expect("failed to wait on ssh");
//     if !status.success() {
//         return Err(format!("SSH command failed with status: {}", status));
//     }
//     Ok(())
// }