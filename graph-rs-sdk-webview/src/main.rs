use std::collections::HashSet;
use std::time::Instant;
use clap::Parser;

// `C:\Users\reeis\src\graph-rs\target\debug\graph-rs-sdk-webview.exe
// cargo run --bin graph-rs-sdk-webview -- --window-title my_window_title

fn main() {
    let args = Args::parse();

    println!("{:#?}", args.window_title);
}

#[derive(Debug, Parser)]
#[command(name = "graph-rs-sdk-webview")]
#[command(author, version)]
struct Args {
    #[arg(short, long)]
    window_title: Option<String>,

    #[arg(short, long)]
    ports: Option<String>,

    /// Add a timeout that will close the window and return an error
    /// when that timeout is reached. For instance, if your app is waiting on the
    /// user to log in and the user has not logged in after 20 minutes you may
    /// want to assume the user is idle in some way and close out of the webview window.
    ///
    /// Default is no timeout.
    //#[arg(short, long)]
    ///pub timeout: Option<Instant>,

    /// The webview can store the cookies that were set after sign in so that on the next
    /// sign in the user is automatically logged in through SSO. Or you can clear the browsing
    /// data, cookies in this case, after sign in when the webview window closes.
    ///
    /// Default is false
    #[arg(short, long)]
    pub clear_data: bool,
}
