pub unsafe extern "system" fn vulkan_debug_callback(
    _: vk::DebugUtilsMessageSeverityFlagsEXT,
    _: vk::DebugUtilsMessageTypeFlagsEXT,
    callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _: *mut c_void,
) -> vk::Bool32 {
    println!("{:?}", (*callback_data).p_message);
    vk::FALSE
}

pub struct ValidationInfo {
    pub is_enabled: bool,
    pub required_validation_layers: Vec<String>,
}

impl ValidationInfo {
    pub fn check_validation_layer_support(entry: &ash::Entry) -> bool {
        let layer_properties = entry
            .enumerate_instance_layer_properties()
            .expect("Failed to enumerate Instance Layer properties!");

        if layer_properties.len() <= 0 {
            eprintln!("No available layers.");
            return false;
        } else {
            for required_layer in required_validation_layers
        }

        // TODO: Replace this
        false
    }
}

pub fn setup_debug_callback(
    entry: &ash::Entry,
    instance: &ash::Instance,
) -> (ash::extensions::DebugUtils, vk::DebugUtilsMessengerEXT) {
    let debug_utils_loader = ash::extensions::DebugUtils::new(entry, instance);

    let debug_create_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
        .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::all())
        .message_type(vk::DebugUtilsMessageTypeFlagsEXT::all())
        .pfn_user_callback(Some(vulkan_debug_callback))
        .build();

    let debug_callback = unsafe {
        debug_utils_loader
            .create_debug_utils_messenger_ext(&debug_create_info, None)
            .expect("Failed to setup Debug callback!")
    };

    (debug_utils_loader, debug_callback)
}
