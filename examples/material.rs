use druid_components::material::*;
use druid_components::env::*;
use druid::{AppLauncher, Color, Data, Lens, Widget, WidgetExt, WindowDesc, text};
use druid::widget::{CrossAxisAlignment,Flex, Label};
#[derive(Clone,Lens,Data)]
struct AppData {
    text: String
}
fn main() {
    let window = WindowDesc::new(ui);
    AppLauncher::with_window(window).configure_env(|env, _| {
        druid_components::env::color::ColorEnv::default().setup_env(env);
    }).use_simple_logger().launch(AppData{text:"".to_string()}).expect("Failed to launch application.");
}

fn ui() -> impl Widget<AppData> {
    
    Flex::column()
        .with_child(Label::new("Name"))
        .with_child(TextBox::new().with_placeholder("Input Something").lens(AppData::text))
        .with_default_spacer()
        .with_child(Button::new("Btn"))
        .padding(40.0)
}