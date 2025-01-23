use std::env::args;

fn help(bin_name: &str) {
    println!("{bin_name}: [OPTIONS...] ...\n");
    println!("Remote-display Management-daemon.\n");
    println!("  -h --help             Show this help");
    println!("     --version          Show package version");
    println!("     --log-level <lvl>  Maximum level for log messages");
    println!("     --log-time         Prefix log-messages with timestamp");
    println!("     --log-date-time    Prefix log-messages with date time");

}

fn parse_cmd() {
    let args: Vec<String> = args().collect();
    let mut iter = args.iter().enumerate();

    while let Some((index,arg)) = iter.next() {
        println!("Cmd{index} is [{arg}]");

        if arg == "help" {
            help(&args[0]);
        } else if arg == "version" {
            let package_name = env!("CARGO_PKG_NAME");
            let package_version = env!("CARGO_PKG_VERSION");
            println!("This is {} version {}", package_name, package_version);
        }   
    }   
}
fn main() {
}
