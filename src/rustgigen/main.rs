// Copyright 2013 The rust-gobject authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern mod extra;
extern mod gi;

use extra::getopts::*;
use std::os;
use std::util;

fn print_usage(program: &str, opts: &[Opt]) {
    util::ignore(opts);
    println!("Usage: {} [general-options] command [command-options] [command-arguments]", program);
    println("General options:");
    println("    -h, --help          Print usage and exit");
    println("    --version           Print version information and exit");
    println("\nCommands:");
    println("    help                Print usage for a command and exit");
    println("    generate            Generate Rust bindings for a GI namespace and optional version");
}

fn print_version_info() {
    println("0.1");
}

fn print_generate_usage(program: &str, general_opts: &[Opt], command_opts: &[Opt]) {
    util::ignore(general_opts);
    util::ignore(command_opts);
    println!("Usage: {} [general-options] generate [command-options] namespace output-module", program);
    println("Command options:");
    println("    -h, --help          Print usage and exit");
    println("    --version=VERS      (optional) Require version VERS of the namespace");
}

fn do_generate(program: &str, general_opts: &[Opt], args: ~[~str]) {
    let opts = ~[
        optflag("h"),
        optflag("help"),
        optopt("version")
    ];

    let matches = match getopts(args.tail(), opts) {
        Err(f) => {
            println!("Error: {}", f.to_err_msg());
            os::set_exit_status(1);
            return;
        },
        Ok(m) => m
    };

    if matches.opt_present("h") || matches.opt_present("help") {
        print_generate_usage(program, general_opts, opts);
        return;
    }

    if matches.free.len() == 0 {
        println("Error: No GI namespace");
        os::set_exit_status(1);
        return;
    } else if matches.free.len() == 1 {
        println("Error: No output module name");
        os::set_exit_status(1);
        return;
    }

    let opt_version = matches.opt_str("version");
    let namespace = matches.free[0];

    let repository = gi::Repository::default();

    // TODO: `opt_version.as_ref()`
    let require_res = repository.require(namespace, opt_version.map(|s| s.as_slice()), []);
    match require_res {
        Err(error) => {
            println!("Error: Failed to load namespace '{}': {}", namespace, error.message());
        },
        Ok(typelib) => {
        }
    }
}

fn main() {
    let args = os::args();
    let program = args[0].clone();

    let opts = ~[
        optflag("h"),
        optflag("help"),
        optflag("version")
    ];

    let matches = match getopts(args.tail(), opts) {
        Err(f) => {
            println!("Error: {}", f.to_err_msg());
            os::set_exit_status(1);
            return;
        },
        Ok(m) => m
    };

    if matches.opt_present("h") || matches.opt_present("help") {
        print_usage(program, opts);
        return;
    }

    if matches.opt_present("version") {
        print_version_info();
        return;
    }

    if matches.free.len() == 0 {
        println("Error: No command");
        os::set_exit_status(1);
        return;
    }

    let command = matches.free[0].clone();

    if command == ~"gen" || command == ~"generate" {
        do_generate(program, opts, matches.free.clone());
    } else {
        println!("Error: Unknown command '{}'", command);
        os::set_exit_status(1);
        return;
    }
}
