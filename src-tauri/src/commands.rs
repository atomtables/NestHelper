use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use serde::Serialize;
use tauri::AppHandle;
use tauri::ipc::Channel;

#[tauri::command]
pub fn helloworld() -> String {
    println!("Hello, world!");
    "Hello, world!".into()
}

#[tauri::command]
pub async fn verifyssh(username: String) -> Result<bool, String> {
    let needed = Command::new("ssh")
        .arg(format!("{}@hackclub.app", username))
        .args(["-o", "LogLevel=ERROR"])
        .args(["-C", "uname -a"])
        .output();
    println!("{:?}", needed);
    if needed.is_err() {
        return Err(format!(
            "Failed to run command: {}",
            needed.err().unwrap().to_string()
        ));
    };
    let output = needed.ok().unwrap();
    println!("{:?}", output);
    if output.status.code() != Some(0) {
        return Err(output
            .stderr
            .into_iter()
            .map(|c| c as char)
            .collect::<String>());
    }
    Ok(true)
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "event", content = "data")]
pub(crate) enum ProcessEvent<'a> {
    Started {
        command: &'a str,
        process_id: u32
    },
    Progress {
        file: String,
        output: String
    },
    Finished {
        complete_output: String,
        return_code: i32
    },
}

#[tauri::command]
pub fn verifySSH(username: String, on_event: Channel<ProcessEvent>) -> Result<(), String> {
    let mut command = Command::new("ssh")
        .arg(format!("{}@hackclub.app", username))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn().expect("Unable to spawn in an instance of SSH");

    on_event.send(ProcessEvent::Started {
        command: &format!("ssh {}@hackclub.app -o LogLevel=ERROR -C uname -a", username),
        process_id: command.id()
    }).unwrap();

    let stdout = command.stdout.take().expect("Was unable to get output from SSH");
    let stderr = command.stderr.take().expect("Was unable to get output from SSH");

    let reader = BufReader::new(stdout);
    reader
        .lines()
        .for_each(|line| on_event.send(ProcessEvent::Progress {
            file: "stdout".to_string(),
            output: "".to_string(),
        }).unwrap());

    let reader = BufReader::new(stderr);
    reader
        .lines()
        .for_each(|line| on_event.send(ProcessEvent::Progress {
            file: "stderr".to_string(),
            output: "".to_string(),
        }).unwrap());

    let status = command.wait().unwrap();
    on_event.send(ProcessEvent::Finished {
        complete_output: "".to_string(),
        return_code: status.code().unwrap()
    }).unwrap();

    Ok(())
}