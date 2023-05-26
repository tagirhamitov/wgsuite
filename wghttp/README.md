# WGBot

This is a HTTP server which exposes an API from the [WGLib](../wglib/README.md).

## Installation

Install this by running the following command:
```shell
$ cargo install wghttp
```

## Running

You can run the server with one simple command:
```shell
    $ wghttp --device wg0 --config-path ~/.wg
```
You can replace `device` and `config-path` with your own values in case you didn't use default values in wgtool.

## Endpoints

You can use the following endpoints:
* `POST /up` - starts VPN server
* `POST /down` - stops VPN server
* `POST /reboot` - restarts VPN server
* `POST /clients` - adds a client with given name. For this request you need to send the body in the following json format: `{ "name": "YourClientName" }`
* `DELETE /clients/:id` - removes client by id
* `GET /clients` - get list of all clients with their metrics.
* `GET /config/:id` - get a configuration of client by id which is used for connecting to the VPN.

The server is running on port 3000.
