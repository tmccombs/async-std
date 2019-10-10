//! Reads a line from stdin, or exits with an error if nothing is read in 5 seconds.

use std::time::Duration;

use async_std::io;
use async_std::thread;

fn main() -> io::Result<()> {
    // This async scope times out after 5 seconds.
    thread::spawn_task(io::timeout(Duration::from_secs(5), async {
        let stdin = io::stdin();

        // Read a line from the standard input and display it.
        let mut line = String::new();
        stdin.read_line(&mut line).await?;
        dbg!(line);

        Ok(())
    }))
}
