# WGTool

Installation:
```shell
$ cargo install wgtool
```

Suported commands:
* `init` - initialize server with default parameters. Configuration is written to `$HOME/.wg`. If needed, all parameters can be customized with flags.
* `add-client name` - add client with given name. Server doesn't need to be restarted for this change.
* `remove-client id` - remove client by its id. Server doesn't need to be restarted for this change.
* `list-clients [name]` - list existing clients. If `name` is specified, it's used to filter result.
* `start` - starts VPN server.
* `stop` - stops VPN server.
* `restart` - restarts VPN server.

In `wg init` commands there're some default hardcoded parameters:
* `--subnet` is 10.0.0.0/24 by default.
* `--port` is 51820.

Other parameters are derived from the system network interfaces.

You can see all available commands and their parameters by running:
```shell
$ wgtool --help
```
