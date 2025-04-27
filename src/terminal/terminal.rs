use std::io::{Write, Read};
use std::process::{Command, Stdio};
use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};

pub struct EmbeddedTerminal {
    input_tx: Sender<String>,
    output_rx: Receiver<String>,
}

impl EmbeddedTerminal {
    pub fn new() -> Self {
        let (input_tx, input_rx) = mpsc::channel::<String>();
        let (output_tx, output_rx) = mpsc::channel::<String>();

        // Spawn a thread to run the shell process
        thread::spawn(move || {
            let mut child = Command::new("sh")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to spawn shell");

            let mut stdout = child.stdout.take().expect("Failed to open stdout");
            let mut stdin = child.stdin.take().expect("Failed to open stdin");

            // Thread to read from shell stdout and send to output channel
            let output_tx_clone = output_tx.clone();
            thread::spawn(move || {
                let mut buffer = [0; 1024];
                loop {
                    match stdout.read(&mut buffer) {
                        Ok(n) if n > 0 => {
                            let output = String::from_utf8_lossy(&buffer[..n]).to_string();
                            output_tx_clone.send(output).unwrap();
                        }
                        _ => break,
                    }
                }
            });

            // Read input commands from input_rx and write to shell stdin
            while let Ok(cmd) = input_rx.recv() {
                writeln!(stdin, "{}", cmd).unwrap();
                stdin.flush().unwrap();
            }
        });

        EmbeddedTerminal {
            input_tx,
            output_rx,
        }
    }

    pub fn send_command(&self, cmd: String) {
        self.input_tx.send(cmd).unwrap();
    }

    pub fn try_receive_output(&self) -> Option<String> {
        self.output_rx.try_recv().ok()
    }
}
