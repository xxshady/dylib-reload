# dylib-reload (temp name)

TODO: non dev mode?

TODO: check workspace version between host and module impl in example

TODO: document thread-locals destructors behavior on library unload,
      whats allowed and whats not, for example allocations in destructors on windows

TODO: what to do with dead locks, leaked file handles, net sockets

after release:
TODO: windows: check spawned threads & successful unloading of library,
      lift alloc restriction of thread-local destructors in main thread of module