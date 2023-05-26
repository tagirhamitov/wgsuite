# Tools for managing Wireguard VPN

This repository was created for my course work at HSE University and it contains tools for managing Wireguard server and its clients.

## Requirements

All tools are using wireguard, so it should be installed in your system.

You can find installation instructions [here](https://www.wireguard.com/install/)

Example for Ubuntu/Debian:
```shell
$ sudo apt update
$ sudo apt install wireguard
```

If you want to use Dashboard, you should also install [Docker](https://docs.docker.com/engine/install/)

## WGLib

All tools represented in this repository use this library under the hood. It contains classes and methods for managing Wireguard configuration, running and halting a server and for exporting user metrics.

More in this [readme](wglib/README.md)

## WGTool

This tool allows you to manage Wireguard server in shell terminal.

More in this [readme](wgtool/README.md)

## WGBot

This is an implementation of chat bot using Telegram Bot API. It allows to modify Wireguard configuration and server status.

More in this [readme](wgbot/README.md)

## WGHTTP

This is a HTTP server which exposes WGLib API for network usage. It also supports exporting user metrics. This backend is used in the [Dashboard](dashboard/README.md)

More in this [readme](wghttp/README.md)

## Dashboard

This is a simple frontend created for WGHTTP. It allows listing all VPN users and their metrics, downloading their .conf files for connecting, deleting them and adding new users.

More in this [readme](dashboard/README.md)
