# tracing-etw

Emit [ETW](https://docs.microsoft.com/en-us/windows/win32/etw/about-event-tracing) events in [tracing](https://tracing.rs)-enabled Rust applications.

This crate depends on [rust_win_etw](https://github.com/microsoft/rust_win_etw).

There are four ETW events.
```rust
fn NewSpan(span_id: u64, name: &str, file: &str, line: u32);
fn EnterSpan(span_id: u64);
fn ExitSpan(span_id: u64);
fn Event(span_id: u64, message: &str);
```

## Collecting a Rust ETW trace

We are going to use [PerfView](https://github.com/microsoft/perfview) to collect a tracing-etw enabled Rust program.

0. Build your app with `cargo build`. To build the example, `cargo build --example test`.

1. Open **PerfView.exe**.

2. In the menu bar, **Collect > Run**.

3. Enter your program's location in the **Command** box

4. Choose Advanced options.

5. The upper part of the Advanced options area includes check boxes and fields that specify the providers from which to collect event trace data. For now, **Uncheck all**.

6. In **Additional Providers**, enter `9c211c60-a6bc-43c3-8d4d-232c121b1852`. This is the GUID of the `tracing-etw` provider.

7. Click **Run Command**

## Contributing

This project welcomes contributions and suggestions.  Most contributions require you to agree to a
Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us
the rights to use your contribution. For details, visit https://cla.opensource.microsoft.com.

When you submit a pull request, a CLA bot will automatically determine whether you need to provide
a CLA and decorate the PR appropriately (e.g., status check, comment). Simply follow the instructions
provided by the bot. You will only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct](https://opensource.microsoft.com/codeofconduct/).
For more information see the [Code of Conduct FAQ](https://opensource.microsoft.com/codeofconduct/faq/) or
contact [opencode@microsoft.com](mailto:opencode@microsoft.com) with any additional questions or comments.