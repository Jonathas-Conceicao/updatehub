// Copyright (C) 2019 O.S. Systems Sofware LTDA
//
// SPDX-License-Identifier: Apache-2.0

extern crate reqwest;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub(crate) struct AgentInfo {
    /// the current version of the agent
    version: Option<String>,
    /// the current configuration of the agent
    config: Option<String>, // need to be changed, not sure the type.
    /// the current firmware of the agent
    firmware: Option<String>, // need to be changed, not sure the type.
}

pub(crate) fn run() {
    display_info().unwrap();
}

fn display_info() -> Result<(), failure::Error> {
    let body = reqwest::get("http://localhost:8080/info")?.text()?;

    println!("body = {:?}", body);

    Ok(())
}
