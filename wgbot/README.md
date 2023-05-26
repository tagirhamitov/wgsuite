# WGBot

This is an example interface for WGLib which doesn't require to use terminal.

## Installation

Install this by running the following command:
```shell
$ cargo install wgbot
```

## Running

1.  Get your Telegram ID. You can see it from this [bot](https://t.me/getmyid_bot)
2.  Register your bot using official tool [BotFather](https://t.me/BotFather). You also should retreive a token for your bot
3.  Initialize a Wireguard server using [WGTool](../wgtool/README.md)
4.  Run the bot using command:
    ```shell
    $ wgbot --token "YOUR_TELEGRAM_TOKEN" --device wg0 --config-path ~/.wg --admin-id "YOUR_TELEGRAM_ID"
    ```
    You can replace `device` and `config-path` with your own values in case you didn't use default values in wgtool.

## Commands

You can use the following commands in your bot:
* `/up` - starts VPN server
* `/down` - stops VPN server
* `/reboot` - restarts VPN server
* `/addclient name` - adds a client with given name
* `/removeclient id` - removes client by id
* `/listclients` - lists all clients with their ids.
