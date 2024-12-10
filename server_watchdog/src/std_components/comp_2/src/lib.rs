use plugin_api::Plugin;

pub struct Comp2Plugin;

impl Plugin for Comp2Plugin {
    fn name(&self) -> &'static str {
        "Component 2"
    }

    fn execute(&self) {
        println!("Executing Component 2!");
    }
}

#[no_mangle]
pub extern "C" fn create_plugin() -> Box<dyn Plugin> {
    Box::new(Comp2Plugin)
}
