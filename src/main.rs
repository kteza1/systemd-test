extern crate systemd;
use systemd::journal::{Journal, JournalFiles, JournalRecord, JournalSeek};
use std::thread;
use std::time::Duration;

fn main() {
    let mut journal = Journal::open(JournalFiles::All, false, false)
        .expect("Failed to open journal");

    let mut count: u64 = 0;
    loop {
        let record = match journal.next_record().unwrap() {
            Some(record) => {
                count += 1;
                println!("{:?}", count);
            }
            None => {
                println!("Done .........");
                journal.wait(10);
                thread::sleep(Duration::new(3, 0));
            }
        };
    }

}
