#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use rune::{Diagnostics, Source, Sources, Vm};
    use rune::termcolor::{ColorChoice, StandardStream};
    use std::sync::Arc;

    let mut context = rune::Context::with_default_modules()?;
    context.install(rune_modules::fs::module(true)?)?;
    let runtime = Arc::new(context.runtime()?);

    let mut sources = Sources::new();
    sources.insert(Source::from_path("src/scripts/aoc_2022_1_1.rn")?)?;

    let mut diagnostics = Diagnostics::new();

    let result = rune::prepare(&mut sources)
        .with_context(&context)
        .with_diagnostics(&mut diagnostics)
        .build();

    if !diagnostics.is_empty() {
        let mut writer = StandardStream::stderr(ColorChoice::Always);
        diagnostics.emit(&mut writer, &sources)?;
    }

    let unit = result?;
    let mut vm = Vm::new(runtime, Arc::new(unit));

    let output = vm.execute(["main"], ())?.async_complete().await.into_result()?;
    let output: i64 = rune::from_value(output)?;

    println!("{:?}", output);

    Ok(())
}
