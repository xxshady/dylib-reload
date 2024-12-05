# dylib-reload (temp name)

TODO: non dev mode?
TODO: check crate (workspace?) version as well
TODO: windows support
TODO: add API to add user exports and imports
TODO: document what main fn of module can return

TODO: document or even add trait for types which are safe to move from module
      to host, since module memory is freed when it's unloaded, so
      for example: moving String from module to host will cause double free
