mod app;

use zeno_core::anyhow::Error;
use crate::app::run;

fn main() -> Result<(), Error> {
    run()?;
    Ok(())
}
