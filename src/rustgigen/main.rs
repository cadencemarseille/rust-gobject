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
    println!("Usage: {} [options] command [command-options] [command-arguments]", program);
    println("Options:");
    println("    -h, --help          Print usage and exit");
    println("    --version           Print version information and exit");
}

fn print_version_info() {
    println("0.1");
}

fn do_list_namespaces() {
    let repository = gi::Repository::default();
    let namespaces = repository.loaded_namespaces();
    for namespace in namespaces.iter() {
        println(*namespace);
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

    let opt_matches = match getopts(args.tail(), opts) {
        Err(f) => {
            println!("Error: {}", f.to_err_msg());
            os::set_exit_status(1);
            return;
        },
        Ok(m) => m
    };

    if opt_matches.opt_present("h") || opt_matches.opt_present("help") {
        print_usage(program, opts);
        return;
    }

    if opt_matches.opt_present("version") {
        print_version_info();
        return;
    }

    if opt_matches.free.len() == 0 {
        println("Error: No command");
        os::set_exit_status(1);
        return;
    }

    let command = opt_matches.free[0].clone();

    if command == ~"list-namespaces" {
        do_list_namespaces();
    } else {
        println!("Error: Unknown command '{}'", command);
        os::set_exit_status(1);
        return;
    }
}
