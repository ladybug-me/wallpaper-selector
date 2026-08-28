use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender, channel};
use std::time::Instant;

use futures_channel::mpsc::UnboundedSender;

use super::Wake;

enum ClockCommand {
    Schedule(Option<Instant>),
    AcknowledgeRedraw,
}

#[derive(Clone)]
pub struct FrameClock {
    commands: Sender<ClockCommand>,
}

impl FrameClock {
    pub fn start(wake: UnboundedSender<Wake>) -> Self {
        let (commands, receiver) = channel();
        std::thread::Builder::new()
            .name(String::from("skwd-frame-clock"))
            .spawn(move || run(&receiver, &wake))
            .expect("spawn frame clock");
        Self { commands }
    }

    pub fn schedule(&self, deadline: Option<Instant>) {
        let _ = self.commands.send(ClockCommand::Schedule(deadline));
    }

    pub fn acknowledge_redraw(&self) {
        let _ = self.commands.send(ClockCommand::AcknowledgeRedraw);
    }
}

fn run(receiver: &Receiver<ClockCommand>, wake: &UnboundedSender<Wake>) {
    let mut deadline: Option<Instant> = None;
    let mut awaiting_redraw = false;
    loop {
        let received = match deadline.filter(|_| !awaiting_redraw) {
            Some(at) => match receiver.recv_timeout(at.saturating_duration_since(Instant::now())) {
                Ok(command) => Ok(command),
                Err(RecvTimeoutError::Timeout) => {
                    deadline = None;
                    awaiting_redraw = true;
                    if wake.unbounded_send(Wake::Frame(Instant::now())).is_err() {
                        return;
                    }
                    continue;
                }
                Err(RecvTimeoutError::Disconnected) => return,
            },
            None => receiver.recv().map_err(|_| RecvTimeoutError::Disconnected),
        };
        match received {
            Ok(ClockCommand::Schedule(next)) => deadline = next,
            Ok(ClockCommand::AcknowledgeRedraw) => awaiting_redraw = false,
            Err(_) => return,
        }
    }
}

#[cfg(test)]
mod tests;
