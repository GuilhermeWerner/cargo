// See src/cargo/lib.rs for notes on these lint settings.
#![warn(rust_2018_idioms)]
#![allow(clippy::all)]

#[macro_use]
extern crate cargo_test_macro;

mod advanced_env;
mod alt_registry;
mod artifact_dep;
mod bad_config;
mod bad_manifest_path;
mod bench;
mod binary_name;
mod build;
mod build_plan;
mod build_script;
mod build_script_env;
mod build_script_extra_link_arg;
mod cache_messages;
mod cargo_add;
mod cargo_alias_config;
mod cargo_command;
mod cargo_config;
mod cargo_env_config;
mod cargo_features;
mod cargo_remove;
mod cargo_targets;
mod cfg;
mod check;
mod check_cfg;
mod clean;
mod collisions;
mod concurrent;
mod config;
mod config_cli;
mod config_include;
mod corrupt_git;
mod credential_process;
mod cross_compile;
mod cross_publish;
mod custom_target;
mod death;
mod dep_info;
mod direct_minimal_versions;
mod directory;
mod doc;
mod docscrape;
mod edition;
mod error;
mod features;
mod features2;
mod features_namespaced;
mod fetch;
mod fix;
mod freshness;
mod future_incompat_report;
mod generate_lockfile;
mod git;
mod git_auth;
mod git_gc;
mod glob_targets;
mod help;
mod https;
mod inheritable_workspace_fields;
mod init;
mod install;
mod install_upgrade;
mod jobserver;
mod list_availables;
mod local_registry;
mod locate_project;
mod lockfile_compat;
mod login;
mod logout;
mod lto;
mod member_discovery;
mod member_errors;
mod message_format;
mod messages;
mod metabuild;
mod metadata;
mod minimal_versions;
mod multitarget;
mod net_config;
mod new;
mod offline;
mod old_cargos;
mod out_dir;
mod owner;
mod package;
mod package_features;
mod patch;
mod path;
mod paths;
mod pkgid;
mod plugins;
mod proc_macro;
mod profile_config;
mod profile_custom;
mod profile_overrides;
mod profile_targets;
mod profiles;
mod progress;
mod pub_priv;
mod publish;
mod publish_lockfile;
mod read_manifest;
mod registry;
mod registry_auth;
mod rename_deps;
mod replace;
mod required_features;
mod run;
mod rust_version;
mod rustc;
mod rustc_info_cache;
mod rustdoc;
mod rustdoc_extern_html;
mod rustdocflags;
mod rustflags;
mod search;
mod shell_quoting;
mod source_replacement;
mod ssh;
mod standard_lib;
mod test;
mod timings;
mod tool_paths;
mod tree;
mod tree_graph_features;
mod unit_graph;
mod update;
mod vendor;
mod verify_project;
mod version;
mod warn_on_failure;
mod weak_dep_features;
mod workspaces;
mod yank;

#[cargo_test]
fn aaa_trigger_cross_compile_disabled_check() {
    // This triggers the cross compile disabled check to run ASAP, see #5141
    cargo_test_support::cross_compile::disabled();
}

// This is placed here as running tests in `cargo-test-support` would rebuild it
#[cargo_test]
fn check_test_dir() {
    let tests = vec![
        (
            "tests/testsuite/workspaces.rs",
            "workspace_in_git",
            "testsuite/workspaces/workspace_in_git",
        ),
        (
            "tests/testsuite/cargo_remove/invalid_arg/mod.rs",
            "case",
            "testsuite/cargo_remove/invalid_arg/case",
        ),
        (
            "tests/build-std/main.rs",
            "cross_custom",
            "build-std/main/cross_custom",
        ),
        (
            "src/tools/cargo/tests/testsuite/build.rs",
            "cargo_compile_simple",
            "src/tools/cargo/testsuite/build/cargo_compile_simple",
        ),
        (
            "src/tools/cargo/tests/testsuite/cargo_add/add_basic/mod.rs",
            "case",
            "src/tools/cargo/testsuite/cargo_add/add_basic/case",
        ),
        (
            "src/tools/cargo/tests/build-std/main.rs",
            "cross_custom",
            "src/tools/cargo/build-std/main/cross_custom",
        ),
        (
            "workspace/more/src/tools/cargo/tests/testsuite/build.rs",
            "cargo_compile_simple",
            "src/tools/cargo/testsuite/build/cargo_compile_simple",
        ),
    ];
    for (path, name, expected) in tests {
        assert_eq!(
            cargo_test_support::paths::test_dir(path, name),
            std::path::PathBuf::from(expected)
        );
    }
}
