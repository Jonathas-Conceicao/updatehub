// Copyright (C) 2018 O.S. Systems Sofware LTDA
//
// SPDX-License-Identifier: Apache-2.0

macro_rules! create_state_step {
    ($source:ident => $dest:ident) => {
        impl From<State<$source>> for State<$dest> {
            fn from(from: State<$source>) -> State<$dest> {
                Self {
                    inner: $dest {},
                    shared_state: from.shared_state,
                }
            }
        }
    };
    ($source:ident => $dest:ident($field:ident)) => {
        impl From<State<$source>> for State<$dest> {
            fn from(from: State<$source>) -> State<$dest> {
                Self {
                    inner: $dest {
                        $field: from.inner.$field,
                    },
                    shared_state: from.shared_state,
                }
            }
        }
    };
}

macro_rules! for_any_state {
    ($machine:ident, $state:ident, $code:block) => {
        match $machine {
            crate::states::StateMachine::Park($state) => $code,
            crate::states::StateMachine::Idle($state) => $code,
            crate::states::StateMachine::Poll($state) => $code,
            crate::states::StateMachine::Probe($state) => $code,
            crate::states::StateMachine::Download($state) => $code,
            crate::states::StateMachine::Install($state) => $code,
            crate::states::StateMachine::Reboot($state) => $code,
        }
    };
}

#[cfg(test)]
macro_rules! assert_state {
    ($machine:ident, $state:ident) => {
        assert!(
            if let Ok(StateMachine::$state(_)) = $machine {
                true
            } else {
                false
            },
            "Failed to get to {} state.",
            stringify!($state),
        );
    };
}
