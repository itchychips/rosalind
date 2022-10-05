extern crate getopts;
extern crate log;
extern crate simple_logger;

mod nucleotides;
mod problems;

use std::fs::read_to_string;
use std::env;
use std::process::ExitCode;

use getopts::Options;
use simple_logger::SimpleLogger;
use log::{error, info, trace};

/// Print usage for program.
fn print_usage(program: &str, opts: Options) {
    let brief = format!(r#"
        Usage: {0} [-v] -p PROBLEM INPUT_FILE
               {0} -l

        Run rosalind PROBLEM on input FILE."#, program);
    print!("{}", opts.usage(&brief));
}

fn normalize_problem_alias(problem: String) -> String {
    if problem == "0" {
        "counting_dna_nucleotides".to_owned()
    }
    else if problem == "counting-nucleotides" {
        "counting_dna_nucleotides".to_owned()
    }
    else {
        problem
    }
}

fn main() -> ExitCode {
    SimpleLogger::new()
        .without_timestamps()
        .with_level(log::LevelFilter::Trace)
        .init()
        .unwrap();
    log::set_max_level(log::LevelFilter::Warn);
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflagmulti("h", "help", "print this help menu");
    opts.optflagmulti("l", "list-problems", "list problems implemented");
    opts.optflagmulti("v", "verbose", "increase verbosity level to info");
    opts.optflagmulti("t", "trace", "increase verbosity level to trace (overrides -v)");

    opts.optmulti("p", "problem", "use selected problem", "PROBLEM");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(e) => {
            error!("parsing command line: {}", e);
            return ExitCode::from(2);
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return ExitCode::SUCCESS;
    }

    if matches.opt_present("l") {
        println!(r#"
            Problems known:
                counting_dna_nucleotides (AKA 0, counting-nucleotides)

            To execute a problem, provide the --problem switch with one of the above options.  Aliases are provided for convenience."#);
        return ExitCode::SUCCESS;
    }

    // --trace overrides --verbose
    if matches.opt_present("t") {
        log::set_max_level(log::LevelFilter::Trace);
        trace!("Setting log level to trace.");
    }
    else if matches.opt_present("v") {
        log::set_max_level(log::LevelFilter::Info);
        info!("Setting log level to info.");
    }


    let problem = if matches.opt_present("p") {
        matches.opt_str("p").unwrap()
    }
    else {
        error!("Must provide -p|--problem");
        print_usage(&program, opts);
        return ExitCode::from(2);
    };

    let input_path = if matches.free.len() == 1 {
        matches.free[0].clone()
    }
    else {
        error!("Error parsing arguments.");
        print_usage(&program, opts);
        return ExitCode::from(2);
    };

    let input = match read_to_string(&input_path) {
        Ok(text) => text,
        Err(e) => {
            error!("{}", e);
            return ExitCode::FAILURE;
        }
    };

    let problem = normalize_problem_alias(problem);

    if problem == "counting_dna_nucleotides" {
        problems::counting_dna_nucleotides(&input);
    }
    else {
        error!("Unknown problem description: {}", problem);
        return ExitCode::FAILURE;
    }

    return ExitCode::SUCCESS;
}
