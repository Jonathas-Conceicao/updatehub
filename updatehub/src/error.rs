// Copyright (C) 2020 O.S. Systems Sofware LTDA
//
// SPDX-License-Identifier: Apache-2.0

use actix_web::{http::StatusCode, HttpResponse};
use derive_more::{Display, From};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Display, From)]
pub enum Error {
    #[display(fmt = "Update package error: {}", _0)]
    UpdatePackage(crate::update_package::UpdatePackageError),
    #[display(fmt = "Runtime settings error: {}", _0)]
    RuntimeSettings(crate::runtime_settings::Error),
    #[display(fmt = "Settings error: {}", _0)]
    Settings(crate::settings::Error),
    #[display(fmt = "Firmware error: {}", _0)]
    Firmware(crate::firmware::Error),
    #[display(fmt = "Client error: {}", _0)]
    Client(crate::client::Error),

    #[display(fmt = "Mailbox error: {}", _0)]
    ActixMailbox(actix::MailboxError),
    #[display(fmt = "Io error: {}", _0)]
    Io(std::io::Error),
}

impl actix_web::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().finish()
    }
}
