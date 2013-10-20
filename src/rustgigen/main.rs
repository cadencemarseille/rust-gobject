// Copyright 2013 The rust-gobject authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern mod extra;
extern mod gi;
extern mod glib;

use extra::getopts::*;
use std::os;
use std::util;

fn print_usage(program: &str, opts: &[Opt]) {
    util::ignore(opts);
    println!("Usage: {} [general-options] command -- [command-options] [command-arguments]", program);
    println("General options:");
    println("    -h, --help          Print usage and exit");
    println("    --version           Print version information and exit");
    println("\nCommands:");
    println("    help                Print usage for a command and exit");
    println("    dump                Dump a GI namespace");
    println("    generate            Generate Rust bindings for a GI namespace");
}

fn print_version_info() {
    println("0.1");
}

fn print_dump_usage(program: &str, general_opts: &[Opt], command_opts: &[Opt]) {
    util::ignore(general_opts);
    util::ignore(command_opts);
    println!("Usage: {} [general-options] dump -- [command-options] namespace", program);
    println("Command options:");
    println("    -h, --help          Print usage and exit");
    println("    --version=VERS      (optional) Require version VERS of the namespace");
}

fn do_dump(program: &str, general_opts: &[Opt], args: ~[~str]) {
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
        print_dump_usage(program, general_opts, opts);
        return;
    }

    if matches.free.len() == 0 {
        println("Error: No GI namespace");
        os::set_exit_status(1);
        return;
    }

    let opt_version = matches.opt_str("version");
    let namespace = matches.free[0];

    let repository = gi::Repository::default();

    // TODO: `opt_version.as_ref()`
    let require_res = repository.require(namespace, opt_version.map(|s| s.as_slice()), []);
    let typelib = match require_res {
        Err(error) => {
            match opt_version {
                None => println!("Error: Failed to load namespace '{}': {}", namespace, error.message()),
                Some(version) => println!("Error: Failed to load namespace '{}' at version {}: {}", namespace, version, error.message())
            }
            os::set_exit_status(1);
            return;
        },
        Ok(typelib) => typelib
    };
    util::ignore(typelib);

    let namespace_cstring = namespace.to_c_str();
    let n_infos = repository.n_infos(&namespace_cstring);
    println!("n_infos = {}", n_infos);
    let mut i: glib::gint = 0;
    while i < n_infos {
        unsafe {
            let info = repository.info(&namespace_cstring, i);
            let info_type = info.type_();
            let opt_name = info.raw_name();
            match opt_name {
                None => println!("\n{}: ({})", i, info_type.to_str()),
                Some(name) => println!("\n{}: {} ({})", i, name, info_type.to_str()),
            }
            for (name, value) in info.attribute_iter() {
                println!("{} => {}", name, value);
            }
            i = i + 1;
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

    if command == ~"help" {
        println("Error: The 'help' command is unimplemented.");
        os::set_exit_status(1);
        return;
    } else if command == ~"dump" {
        do_dump(program, opts, matches.free.clone());
    } else if command == ~"gen" || command == ~"generate" {
        println("Error: The 'generate' command is unimplemented.");
        os::set_exit_status(1);
        return;
    } else {
        println!("Error: Unknown command '{}'", command);
        os::set_exit_status(1);
        return;
    }
}
