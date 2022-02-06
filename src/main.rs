use log::{error, info};
use structopt::StructOpt;

use playwith as lib;

use lib::Controller;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Parse arguments
    let flags = Flags::from_args();

    // Log
    lib::set_logger(flags.verbose);

    // Adapter
    let adapter = match flags.adapter {
        Some(adapter) => adapter,
        None => {
            let adapters = lib::adapters().await;
            match adapters {
                Ok(adapters) => {
                    if adapters.is_empty() {
                        error!("Cannot find available adapter");

                        return;
                    }
                    if adapters.len() > 1 {
                        error!("Cannot determine the adapter. Available adapters are listed below, and please use -a <ADAPTER> to designate:");
                        for adapter in adapters.iter() {
                            info!("    {}", adapter);
                        }

                        return;
                    }

                    adapters.first().unwrap().to_string()
                }
                Err(ref e) => {
                    error!("{}", e);

                    return;
                }
            }
        }
    };

    // Controller
    info!("Use adapter {} for {} emulation", adapter, flags.controller);
    let mut controller = match Controller::new(&adapter, flags.controller).await {
        Ok(controller) => controller,
        Err(ref e) => {
            error!("{}", e);

            return;
        }
    };

    // Pair
    match controller.pair().await {
        Ok(addr) => info!("Device {} paired", addr),
        Err(ref e) => {
            error!("{}", e);

            return;
        }
    };
}

#[derive(StructOpt, Clone, Debug, Eq, Hash, PartialEq)]
#[structopt(about)]
struct Flags {
    #[structopt(long, short, help = "Adapter", value_name = "ADAPTER")]
    pub adapter: Option<String>,

    #[structopt(
        long,
        short,
        help = "Controller (JOY_CON_L, JOY_CON_R or PRO_CONTROLLER)",
        value_name = "CONTROLLER",
        default_value = "PRO_CONTROLLER"
    )]
    pub controller: lib::protocol::ControllerType,

    #[structopt(
        long,
        short,
        help = "Prints verbose information (-vv for vverbose)",
        parse(from_occurrences)
    )]
    pub verbose: usize,
}
