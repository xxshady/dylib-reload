use dylib_reload_shared::Str;

#[unsafe(no_mangle)]
static __CRATE_COMPILATION_INFO__: Str = Str::const_from(crate_compilation_info::get!());
