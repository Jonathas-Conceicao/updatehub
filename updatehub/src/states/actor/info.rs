// Copyright (C) 2019 O.S. Systems Sofware LTDA
//
// SPDX-License-Identifier: Apache-2.0

use crate::{firmware::Metadata, settings::Settings};
use actix::{Context, Handler, Message, MessageResult};
use serde::Serialize;

pub(crate) struct Request;

#[derive(Serialize)]
pub(crate) struct Payload {
    version: String,
    config: Settings,
    firmware: Metadata,
}

impl Message for Request {
    type Result = Payload;
}

impl Handler<Request> for super::Machine {
    type Result = MessageResult<Request>;

    fn handle(&mut self, _: Request, _: &mut Context<Self>) -> Self::Result {
        if let Some(machine) = self.0.take() {
            let payload = for_any_state!(machine, s, {
                Payload {
                    version: crate::version().to_string(),
                    config: s.shared_state.settings.clone(),
                    firmware: s.shared_state.firmware.clone(),
                }
            });

            return MessageResult(payload);
        }

        unreachable!("Failed to take StateMachine's ownership");
    }
}
