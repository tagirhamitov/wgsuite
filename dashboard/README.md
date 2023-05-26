# Dashboard

This folder contains a frontend for [WGHTTP](../wghttp/README.md)

## Description

The dashboard consists of different parts:
* The table of all clients with their metrics. Data usage is displayed with progress bars. It's maximum value is defaulted to 100GB but you can change it in the top left text field.
* On the right side of each client you can find two buttons. First redirects you to this client's configuration. Second is used for deleting the client.
* On the top you can find the form for adding new clients.

All components don't require you to reload the page. Data is retreived every 1 second.

## Running

You can run the dashboard on the same host with the wghttp by running the following command from the /dashboard directory:
```shell
$ docker-compose up --build
```

Then you can connect by visitng `http://localhost`. In case you have different hostname or IP address, you can also use it.
