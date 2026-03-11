use std::{path::PathBuf, sync::mpsc::Sender};

pub enum Command {
    Save(PathBuf),
    NewPoints(Vec<f32>),
    NewNames(Vec<String>),
}

pub fn read_stdin_task(sender: Sender<Command>) -> ! {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let mut splitted = input.split(",");

        let first = splitted.next();

        match first {
            Some("!save") => {
                let filename = splitted.next().unwrap_or("output.png");
                sender.send(Command::Save(PathBuf::from(filename))).unwrap();
            }
            Some(_) => {
                match first.and_then(|time| time.parse::<f32>().ok()) {
                    Some(time) => {
                        let values = splitted
                            .map(|s| s.trim().parse::<f32>())
                            .filter_map(Result::ok)
                            .collect::<Vec<_>>();

                        let mut data = vec![time as f32];
                        data.extend(values);
                        sender.send(Command::NewPoints(data)).unwrap();
                    }
                    None => {
                        // If the first value isn't a valid float, treat the entire line as signal names.
                        let names = input.split(",").map(|s| s.trim().to_string()).collect();
                        sender.send(Command::NewNames(names)).unwrap();
                        continue;
                    }
                }
            }
            None => {}
        }
    }
}
