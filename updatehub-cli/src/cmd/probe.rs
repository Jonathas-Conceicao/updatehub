// Copyright (C) 2019 O.S. Systems Sofware LTDA
//
// SPDX-License-Identifier: Apache-2.0

extern crate reqwest;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub(crate) struct Probe {
    /// print with there any update avaliable
    update_avaliable: bool,
    /// print with tryed again
    try_again_in: u64,
}

pub(crate) fn run() {
    display_info().unwrap();
}

fn display_info() -> Result<(), failure::Error> {
    let body = reqwest::get("http://localhost:8080/probe")?.text()?;

    println!("body = {:?}", body);

    Ok(())
}
