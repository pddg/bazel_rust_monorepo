load("@crate_index//:defs.bzl", "all_crate_deps")
load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "mono",
    srcs = glob(["src/**/*.rs"]),
    deps = all_crate_deps(
        normal = True,
    ),
)

