use clap::{ AppSettings, Clap };

#[derive(Clap)]
#[clap(version = "1.0.0", author = "Denys Vuika <denys.vuika@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long, default_value = "default.conf")]
    config: String,

    /// Some input. Because this isn't an Option<T> it's required to be used
    input: String,

    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,

    #[clap(subcommand)]
    subcmd: Option<SubCommand>,
}

#[derive(Clap)]
enum SubCommand {
    /// Sub-command
    #[clap(version = "1.0", author = "Denys Vuika <denys.vuika@gmail.com>")]
    Test(Test),
}

#[derive(Clap)]
struct Test {
    /// Print debug info
    #[clap(short)]
    debug: bool,
}

fn main() {
    let opts: Opts = Opts::parse();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    println!("Value for config: {}", opts.config);
    println!("Using input file: {}", opts.input);

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match opts.verbose {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(opts) = opts.subcmd {
        match opts {
            SubCommand::Test(t) => {
                if t.debug {
                    println!("Printing debug info");
                } else {
                    println!("Printing normally");
                }
            }
        }
    }

    // more program logic goes here...
}
