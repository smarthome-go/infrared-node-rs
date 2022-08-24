# Infrared Node - Rust Rewrite
This is a new and improved version of [infrared-node](https://github.com/smarthome-go/infrared-node)

## Usage
```
pi@raspberrypi ~> ifrs --help
infrared-node-rs 0.1.0
Raspberry-Pi microservice for Smarthome that allows IR control

USAGE:
    ifrs [OPTIONS]

OPTIONS:
    -c, --config-path <CONFIG_PATH>
            The path where the configuration file should be located

    -d, --discover
            Discover mode is used to set up new buttons of a remote

    -h, --help
            Print help information

    -V, --version
            Print version information
pi@raspberrypi ~>
```

## Configuration
The default configuration file can be found [here](./src/default_config.toml).
```toml
# Smarthome server configuration
[smarthome]
url = "http://smarthome.box"        # Fully-qualified root URL of the main server
token = "your-token"                # Authentication token for (navigate to `http://url/profile` to obtain a token)

# Hardware configuration
[hardware]
enabled = false                     # If the infrared scanner should be initialized on startup
pin = 0                             # The BCM pin number on which the infrared scanner is attached

# Button actions can be configured here
[[actions]]
name = "default"                    # A name which describes your action well
triggers = [101]                    # A list of numbers (the codes which match the action), multiple codes can correspond to one action
homescript = "print('Homescript')"  # The Homescript code which is executed when the action was matched (validated on startup)
```
