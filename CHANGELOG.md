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
