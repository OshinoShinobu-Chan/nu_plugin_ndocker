# Ndocker

Ndocker is a nushell plugin for docker. This plugin aims to do better on nushell comparing to default docker client. The core features of ndocker includes:
1. Ndocker has nushell-style outputs, so you can use it with other nushell features together easily, like `select`/`where` etc.
2. We add interactive mode for some of docker operations. For example, you can create a docker container interactively by answering a series of questions about the config, instead of typing those complexing flags.

## How to use ndocker

### Install from github repository (recommanded)

First, please clone this repository using,

    git clone git@github.com:OshinoShinobu-Chan/nu_plugin_ndocker.git

Second, please install this plugin using,

    cd nu_plugin_ndocker
    cargo install --path . --locked

Please make sure you have rust toolchain on your computer.

Third, add the plugin to your nushell using,

    plugin add nu_plugin_ndocker

Finally, you can open a new nushell session and use ndocker, or you can use it immidiately by using,

    plugin use ndocker

### Install from release on github

First, download the correct binary suitable for you platform.

Second, copy the binary to your plugin directory, usually is `$HOME/.cargo/bin`

Third, following the third instruction in the section above.

## License

This project is using MIT license. Please check the content of the license in `LICENSE`.