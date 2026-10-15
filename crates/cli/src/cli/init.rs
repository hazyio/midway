use dialoguer::Confirm;
pub fn init() {
    let ok = Confirm::new()
        .with_prompt(t!("errors.config_exists"))
        .default(false)
        .interact();
}
