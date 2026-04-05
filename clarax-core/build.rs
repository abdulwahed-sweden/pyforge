fn main() {
    // When building as extension module (maturin), allow undefined Python symbols
    // to be resolved at load time on macOS
    clarax_build_config::add_extension_module_link_args();
}
