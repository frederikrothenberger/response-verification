# Response Verification

Response verification on the [Internet Computer](https://dfinity.org) is the process of
verifying that a canister response from a replica has gone through consensus with other replicas
hosting the same canister.

This package encapsulates the protocol for such verification. It is used by the
[Service Worker](https://github.com/dfinity/ic/tree/master/typescript/service-worker) and
[ICX Proxy](https://github.com/dfinity/ic/tree/master/rs/boundary_node/icx_proxy) and may be
used by other implementations of the
[HTTP Gateway Protocol](https://internetcomputer.org/docs/current/references/ic-interface-spec/#http-gateway)
in the future.
