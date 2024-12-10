use plugin_api::Plugin;

pub struct Comp1Plugin;

impl Plugin for Comp1Plugin {
    fn name(&self) -> &'static str {
        "Component 1"
    }

    fn execute(&self) {
        println!("Executing Component 1!");
    }
}

#[no_mangle]
pub extern "C" fn create_plugin() -> Box<dyn Plugin> {
    Box::new(Comp1Plugin)
}
