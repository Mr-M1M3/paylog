pub struct Client<'a> {
    pub name: &'a str,
    log: Vec<LogEntry>,
}
impl<'a> Client<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            log: Vec::new(),
        }
    }
    pub fn from_string(client_name: &'a str, string: String) -> Self {
        let entries = string
            .split(";")
            .filter_map(|entry| {
                // ATTENTION: For some reason, rust adds an extra empty string the returned iterator. so, I gotta filter it down
                if entry == "" {
                    return None;
                }
                let e = entry.trim().split(":").collect::<Vec<_>>();
                if e.len() < 2 {
                    panic!("corrupted data");
                }
                Some(LogEntry::new(
                    e[0].to_string(),
                    e[1].trim().parse::<isize>().expect("corrupted data"),
                ))
            })
            .collect();
        Self {
            name: client_name,
            log: entries,
        }
    }

    pub fn calculate(&self) -> isize {
        let mut acc: isize = 0;
        self.log.iter().for_each(|entry| acc += entry.amount);
        acc
    }
    pub fn log(&mut self, date: &str, amount: isize) {
        self.log.push(LogEntry::new(date.to_string(), amount));
    }
    pub fn display(&self) -> String {
        let mut acc = String::from(self.name.to_uppercase());
        acc.push('\n');
        self.name.chars().for_each(|_| acc.push('>'));
        self.log.iter().for_each(|entry| {
            acc.push('\n');
            acc.push_str(entry.date.as_str());
            acc.push_str(": ");
            if entry.amount < 0 {
                acc.push_str("gave ");
                acc.push_str(entry.amount.abs().to_string().as_str());
            } else {
                acc.push_str("took ");
                acc.push_str(entry.amount.to_string().as_str());
            }
        });
        acc.push('\n');
        let total_val = self.calculate();
        if total_val < 0 {
            acc.push_str("I owe ");
            acc.push_str(self.name);
            acc.push(' ');
            acc.push_str(total_val.abs().to_string().as_str());
        } else {
            acc.push_str(self.name);
            acc.push_str(" owes me ");
            acc.push_str(total_val.abs().to_string().as_str());
        }
        acc.push_str("tk");
        acc
    }
    pub fn export(&self) -> String {
        let mut acc = String::new();
        self.log.iter().for_each(|entry| {
            acc.push_str(entry.date.as_str());
            acc.push(':');
            acc.push_str(entry.amount.to_string().as_str());
            acc.push(';');
        });
        acc
    }
}

struct LogEntry {
    date: String,
    amount: isize,
}

impl LogEntry {
    fn new(date: String, amount: isize) -> Self {
        Self { date, amount }
    }
}
