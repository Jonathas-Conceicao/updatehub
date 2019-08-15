// Copyright (C) 2019 O.S. Systems Sofware LTDA
//
// SPDX-License-Identifier: Apache-2.0

extern crate reqwest;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub(crate) struct Log {
    /// print the log data
    data: Option<String>,
    /// print the log level
    level: Option<String>,
    /// print the log message
    message: Option<String>,
    /// print the log time
    time: Option<String>,
}

pub(crate) fn run() {
    display_info();
}
fn display_info() -> Result<(), failure::Error> {
    let body = reqwest::get("http://localhost:8080/log")?.text()?;

    println!("body = {:?}", body);

    Ok(())
}
