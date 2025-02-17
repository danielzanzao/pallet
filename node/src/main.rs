fn main() -> sc_cli::Result<()> {
    let version = sc_cli::VersionInfo {
        name: "Substrate Node",
        commit: env!("VERGEN_GIT_COMMIT_HASH"),
        version: env!("CARGO_PKG_VERSION"),
        executable_name: "node-template",
        author: "Substrate DevHub",
        description: "A Substrate node",
        support_url: "https://substrate.io/",
    };

    node_template::command::run(version)
}
