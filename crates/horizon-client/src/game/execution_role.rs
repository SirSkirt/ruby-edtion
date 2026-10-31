//! A Game's execution role is fixed at construction. Connecting or disconnecting
//! changes `remote`, not this role. A client may simulate a world in-frame.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ExecutionRole {
    Client,
    IntegratedServer,
    DedicatedServer,
}

impl ExecutionRole {
    /// Only a client owns presentation work (audio, loading UI, chunk meshes).
    pub fn is_client(self) -> bool {
        self == Self::Client
    }

    /// The optional in-frame simulation remains a client execution mode.
    pub fn starts_integrated_server(self, server_thread: bool) -> bool {
        self.is_client() && server_thread
    }

    /// The single-player owner belongs to the world save, including on the
    /// integrated server thread. Only a real dedicated server manages player
    /// files separately. Integrated servers set options.dedicated for their
    /// headless startup, so that option must NOT control owner persistence.
    pub fn writes_singleplayer_player(self) -> bool {
        self != Self::DedicatedServer
    }
}

#[cfg(test)]
mod tests {
    use super::ExecutionRole::*;

    #[test]
    fn world_loading_keeps_in_frame_clients_and_server_roles_separate() {
        for (role, client, threaded) in [
            (Client, true, true),
            (IntegratedServer, false, false),
            (DedicatedServer, false, false),
        ] {
            assert_eq!(role.is_client(), client);
            assert_eq!(role.starts_integrated_server(true), threaded);
            assert!(!role.starts_integrated_server(false));
        }
    }

    #[test]
    fn integrated_server_persists_its_owner_but_dedicated_does_not() {
        assert!(Client.writes_singleplayer_player());
        assert!(IntegratedServer.writes_singleplayer_player());
        assert!(!DedicatedServer.writes_singleplayer_player());
    }
}
