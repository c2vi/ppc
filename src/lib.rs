
use core::{clone::Clone, result::Result::Ok};
use std::thread;
use tracing::{info, error};

use mize::{mize_err, Instance, MizeError, MizeResult, Module};

mod web;


#[derive(Debug, Clone)]
pub struct Ppc {
    mize: Instance
}


#[no_mangle]
extern "C" fn get_mize_module_ppc(empty_module: &mut Box<dyn Module + Send + Sync>, mize: Instance) -> () {
    let new_box: Box<dyn Module + Send + Sync> = Box::new( Ppc { mize } );

    *empty_module = new_box
}

async fn webserver_wrapper(ppc: Ppc) {
    match web::run_webserver(ppc.clone()).await {
        Ok(_) => info!("webserver exited normally"),
        Err(e) => error!("webserver exited with err: {:?}", e),
    };
}

impl Module for Ppc {
    fn init(&mut self, instance: &Instance) -> MizeResult<()> {

        // TODO... loging does not work in moduless.... (the tracer is not registered)
        mize::platform::os::logging::MinimalTracer::register();

        println!("hello from ppc module hooooooooooooooo");

        let cloned_ppc = self.clone();
        thread::spawn(|| {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(webserver_wrapper(cloned_ppc));
        });
        


        Ok(())
    }

    fn exit(&mut self, instance: &Instance) -> MizeResult<()> {
        Ok(())
    }

    fn clone_module(&self) -> Box<dyn Module + Send + Sync> {
        Box::new(self.clone())
    }
}

