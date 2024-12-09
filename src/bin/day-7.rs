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
        // hints used
        self.logs
            .iter()
            .filter(|log| log.contains(keyword))
            .collect()
    }
}
