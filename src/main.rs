#[macro_use]
extern crate serde_derive;
extern crate docopt;

use docopt::Docopt;

const USAGE: &'static str = "
rust_rep

Usage:
    rust_rep serve -f <filename> -c <connectionInfo>
    rust_rep seed -s <serverConnectionInfo> -c <connectionInfo> [-o <outputFile>]
    rust_rep leech -s <serverConnectionInfo> [-o <outputFile>]
    rust_rep (-h | --help)
    rust_rep (-v | --version)

Options:
    -h --help                   Show this screen
    -v --version                Show verison
    -f <filename>               Select file to serve
    -c <connectionInfo>         Set connection info to serve on
    -s <serverConnectionInfo>   Set connection info to retrieve from
    -o <outputFile>             choose the output of the retrieve [default: out.txt]
";

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Deserialize)]
struct Args {
    cmd_serve: bool,
    cmd_seed: bool,
    cmd_leech: bool,
    flag_version: bool,
    flag_f: String,
    flag_o: String,
    flag_c: String,
    flag_s: String,
}


fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.deserialize()).unwrap_or_else(|e| e.exit());
    if args.flag_version
    {
        print_version();
    }
    else
    {
        if args.cmd_serve
        {
            serve(&args.flag_f, &args.flag_c)
        }
        else if args.cmd_seed
        {
            seed(&args.flag_s, &args.flag_c, &args.flag_o)
        }
        else if args.cmd_leech
        {
            leech(&args.flag_s, &args.flag_o)
        }
    }
}

fn print_version() {
    println!("{}", VERSION);
}

fn serve(filename:&str, connection:&str) {

}
fn seed(server_info:&str, connection:&str, output:&str) {

}

fn leech(server_info:&str, output:&str) {

}
