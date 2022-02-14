fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::configure()
    //     .build_client(true)
    //     .build_server(false)
    //     .out_dir("src")
    //     // .client_mod_attribute("package", "tcs")
    //     .compile(
    //         &[
    //             "contracts/instruments.proto",
    //             "contracts/marketdata.proto",
    //             "contracts/operations.proto",
    //             "contracts/orders.proto",
    //             "contracts/sandbox.proto",
    //             "contracts/stoporders.proto",
    //             "contracts/users.proto",
    //         ],
    //         &["contracts/"],
    //     )?;

    Ok(())
}
