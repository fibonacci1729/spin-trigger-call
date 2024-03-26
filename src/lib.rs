use anyhow::{Context, Result};
use async_trait::async_trait;
use clap::Args;
use serde::{Deserialize, Serialize};
use spin_trigger::{EitherInstance, TriggerAppEngine, TriggerExecutor};
use wasm_wave::{untyped::UntypedFuncCall, wasm::{DisplayFuncArgs, DisplayFuncResults}};
use wasmtime::component::Val;

pub struct CallTrigger {
    engine: TriggerAppEngine<Self>,
}

#[derive(Args, Debug)]
pub struct CallTriggerArgs {
    /// The component id to call.
    #[clap(long)]
    pub id: String,
    /// The call expression to execute.
    #[clap(long)]
    pub call: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CallTriggerConfig {
    pub component: String,
}

#[async_trait]
impl TriggerExecutor for CallTrigger {
    const TRIGGER_TYPE: &'static str = "call";
    type RuntimeData = ();
    type TriggerConfig = CallTriggerConfig;
    type RunConfig = CallTriggerArgs;

    async fn new(engine: TriggerAppEngine<Self>) -> Result<Self> {
        Ok(Self { engine })
    }

    async fn run(self, config: Self::RunConfig) -> Result<()> {
        let (instance, mut store) = self
            .engine
            .prepare_instance(&config.id)
            .await?;

        let EitherInstance::Component(instance) = instance else {
            unreachable!()
        };

        let func_call = UntypedFuncCall::parse(&config.call)?;
        let func_name = func_call.name().to_string();

        let func = instance
            .get_func(&mut store, &func_name)
            .with_context(|| format!("instance has no func export {func_name:?}"))?;

        let func_type = wasm_wave::wasmtime::get_func_type(&func, &store);
        let params = func_call.to_wasm_params(func_type.params.iter())?;

        let mut results = vec![Val::U32(0xfefefefe); func_type.results.len()];

        func.call_async(&mut store, &params, &mut results).await?;

        println!(
            "{}{} -> {}",
            func_name,
            DisplayFuncArgs(&params),
            DisplayFuncResults(&results),
        );

        Ok(())
    }    
}