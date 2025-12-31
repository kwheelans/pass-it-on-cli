# Unreleased

# v0.7.0
## Breaking Changes
-  remove rustls-tls-native-roots feature
- 
## Changes
- Update rust edition to 2024
- Update pass-it-on to 0.17

# v0.6.3
## Changes
- update github actions dependencies

# v0.6.2
## Changes
- update directories to 6.0

# v0.6.1
## Changes
- update `thiserror` dependency  to version 2.x.y

# v0.6.0
## Breaking changes
- removed vendored-tls feature

## Changes
- updated pass-it-on 0.16.0
- added rustls-tls-native-roots feature
- switch to tracing from log for logging

# v0.5.1
## Changes
- Update pass-it-on to v0.15.1

# v0.5.0
## Changes
- Update pass-it-on to v0.15.0

# v0.4.2
## Changes
- Update simple_logger dependency to version 5.0
- Update pass-it-on to 0.14.4

# v0.4.1
## Fixes
- Check that configuration file exists before trying to read
- Return ExitCode to show correct exit code on error

# v0.4.0
## Changes
- default wait before shutdown increased to 2 seconds to receive response
- make `--client_config` optional and use default path if not provided
- update to pass-it-on 0.14.3

# v0.3.2
## Changes
- add `vendored-tls` feature that enables the `vendored-tls` feature for the `pass-it-on` create

# v0.3.1
## Changes
- minor changes related to crate name change

# v0.3.0
## Breaking Changes
- bump pass-it-on dependency to 0.14.0

## Crate Changes
- change crate name to `pass-it-on-cli`

# v0.2.1
## Changes
- bump pass-it-on dependency to 0.13.0

# v0.2.0
## Breaking Changes
- bump pass-it-on dependency to 0.11.0 which is not backwards compatible

# v0.1.1
## Changes
- bump pass-it-on dependency to 0.9.0

# v0.1.0
- Basic functionality specify client configuration, notification name to use and one or more messages to send
