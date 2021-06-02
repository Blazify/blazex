/*
 * Copyright 2020 to 2021 BlazifyOrg
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *    http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
*/

#![allow(dead_code)]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use blazex::compile;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::process::exit;
use std::sync::mpsc::channel;
use std::time::Duration;
use structopt::StructOpt;

/*
* Arguments Struct for CLI Argument Parsing
*/
#[derive(StructOpt)]
struct CmdParams {
    /*
     * Path to the BlazeX Source code
     */
    #[structopt(parse(from_os_str))]
    pub path: PathBuf,

    /*
     * Name of compiled file (Default: input_file.bze)
     */
    #[structopt(parse(from_os_str), long, short = "o")]
    pub out: Option<PathBuf>,

    /*
     * Whether there should be any logging in console (Default: false)
     */
    #[structopt(long, short = "q")]
    pub quiet: bool,

    /*
     * Whether the compiler should compile/run on file changes (Default: false)
     */
    #[structopt(long, short = "w")]
    pub watch: bool,
}

/*
* Entry Point of the Compiler
*/
fn main() {
    let cmd_params = CmdParams::from_args();
    let file_name = cmd_params.path.as_os_str().to_str().unwrap().to_string();
    if !file_name.ends_with(".bzx") {
        eprintln!("Unexpected file {}", file_name);
        exit(1);
    }
    let is_quiet = cmd_params.quiet;
    let out_file = if let Some(out) = cmd_params.out {
        if out.ends_with(".o") {
            out.as_os_str().to_str().unwrap().to_string()
        } else {
            out.as_os_str().to_str().unwrap().to_string() + ".o"
        }
    } else {
        file_name.clone().replace(".bzx", ".o")
    };
    let watch = cmd_params.watch;

    /*
     * Compiling to Object File
     */
    let compile_with_config = || {
        let cnt = std::fs::read_to_string(file_name.clone()).expect("could not read script");
        compile(
            file_name.clone(),
            cnt,
            is_quiet,
            watch,
            out_file.clone(),
            false,
        )
    };

    let init = compile_with_config();
    if !watch {
        exit(init)
    };

    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();

    watcher
        .watch(file_name.clone(), RecursiveMode::Recursive)
        .unwrap();

    /*
     * Triggering the compiler on file change
     */
    loop {
        match rx.recv() {
            Ok(DebouncedEvent::Write(_)) => {
                println!("\u{001b}[32;1mChange Detected!\u{001b}[0m");
                compile_with_config();
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("Watch error: {:?}", e);
                if !watch {
                    exit(1);
                }
            }
        }
    }
}
