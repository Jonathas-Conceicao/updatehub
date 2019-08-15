// Copyright (C) 2019 O.S. Systems Sofware LTDA
//
// SPDX-License-Identifier: Apache-2.0

use structopt::StructOpt;
use exitfailure::ExitFailure;

mod cmd;

#[derive(StructOpt)]
#[structopt(
	name = "updatehub-cli",
	about = "The updatehub Command Line Interface",
	rename_all = "kebab-case",
)]
enum Command {
	/// The logs produced from system
	Log(cmd::log::Log),

	/// The agent information
	Agent(cmd::agentinfo::AgentInfo),

	/// The probe information
	Probe(cmd::probe::Probe),	
}

fn main() -> Result<(), ExitFailure> {
	match Command::from_args() {
		Command::Log(cmd) => cmd::log::run(),
		Command::Agent(cmd) => cmd::agentinfo::run(),
		Command::Probe(cmd) => cmd::probe::run(),
	};

    Ok(())
}

