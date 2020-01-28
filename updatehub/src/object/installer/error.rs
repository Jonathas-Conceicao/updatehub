// Copyright (C) 2020 O.S. Systems Sofware LTDA
//
// SPDX-License-Identifier: Apache-2.0

use derive_more::{Display, From};

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Display, From)]
pub enum Error {
    #[display(fmt = "Invalid path formed")]
    InvalidPath,
    #[display(fmt = "Unsupported target type: {:?}", _0)]
    InvalidTargetType(pkg_schema::definitions::TargetType),

    #[display(fmt = "Utils error: {}", _0)]
    Utils(crate::utils::Error),
    #[display(fmt = "Io error: {}", _0)]
    Io(std::io::Error),
    #[display(fmt = "Process error: {}", _0)]
    Process(easy_process::Error),
}
