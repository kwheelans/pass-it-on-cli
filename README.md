# pass-it-on-cli
[![MIT licensed](https://img.shields.io/crates/l/pass-it-on-cli)](./LICENSE)

A command line tool to send notifications to a [pass-it-on](https://github.com/kwheelans/pass-it-on) server

## Usage
By providing path to a valid pass-it-on client configuration file one of more messages can be sent with the provided notification name.

`pass-it-on-cli -c path/to/client/configuration/file -n my_notification_name -m "message 1" -m "A second message"`

### Client Configuration Example
```toml
[client]
key = "sdfsf4633ghf44dfhdfhQdhdfhewaasg"

[[client.interface]]
type = "pipe"
path = '/path/to/pipe.fifo'
group_read_permission = true
group_write_permission = true

[[client.interface]]
type = "http"
host = "192.168.1.2"
port = 8080
```
