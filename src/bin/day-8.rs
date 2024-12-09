use std::{fs::File, io::Write};

fn main() {
    let logs = vec![
        "Error: something went wrong".to_string(),
        "Info: all systems go".to_string(),
        "Warning: low battery".to_string(),
    ];

    let query = LogQuery::new(&logs);
    let results = query.search("Info");

    for log in results {
        println!("{}", log);
    }
}
pub struct LogQuery<'a> {
    logs: &'a Vec<String>,
}

impl<'a> LogQuery<'a> {
    pub fn new(logs: &'a Vec<String>) -> LogQuery<'a> {
        LogQuery { logs }
    }
    pub fn search(&self, keyword: &str) -> Vec<&'a String> {
        self.logs
            .iter()
            .filter(|log| log.contains(keyword))
            .collect()
    }
    pub fn export_to_file(&self, keyword: &str, path: &str) -> std::io::Result<()> {
        let logs = self.search(keyword);
        let mut file = File::create(&path)?;
        for log in logs {
            writeln!(file, "{}", log)?;
        }
        Ok(())
    }
}
