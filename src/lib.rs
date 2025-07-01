
use core::clone::Clone;

use mize::{Instance, MizeError, MizeResult, Module};


#[derive(Debug, Clone)]
pub struct Ppc {}


#[no_mangle]
extern "C" fn get_mize_module_ppc(empty_module: &mut Box<dyn Module + Send + Sync>, mize: Instance) -> () {
    let new_box: Box<dyn Module + Send + Sync> = Box::new( Ppc {} );

    *empty_module = new_box
}

impl Module for Ppc {
    fn init(&mut self, instance: &Instance) -> MizeResult<()> {
        println!("hello from ppc module hooooooooooooooo");
        Ok(())
    }
    fn exit(&mut self, instance: &Instance) -> MizeResult<()> {
        Ok(())
    }

    fn clone_module(&self) -> Box<dyn Module + Send + Sync> {
        Box::new(self.clone())
    }
}

