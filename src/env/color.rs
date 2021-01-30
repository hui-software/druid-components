//! Material Color Pre-Definition
//! NOTE:Without Alpha
pub const RED_50: u32 = 0xFFEBEEFF;
pub const RED_100: u32 = 0xFFCDD2FF;
pub const RED_200: u32 = 0xEF9A9AFF;
pub const RED_300: u32 = 0xE57373FF;
pub const RED_400: u32 = 0xEF5350FF;
pub const RED_500: u32 = 0xF44336FF;
pub const RED_600: u32 = 0xE53935FF;
pub const RED_700: u32 = 0xD32F2FFF;
pub const RED_800: u32 = 0xC62828FF;
pub const RED_900: u32 = 0xB71C1CFF;

use druid::{Color, Env};
use super::key::*;
pub struct ColorEnv {
    primary: Color,
    primary_variant: Color,
    secondary: Color,
    secondary_variant: Color,
    background: Color,
    surface: Color,
    error: Color,
    on_primary: Color,
    on_secondary: Color,
    on_background: Color,
    on_surface:Color,
    on_error: Color,
}
impl ColorEnv {
    pub fn new(config:(Color,Color,Color,Color))-> Self {
        Self {
            primary: config.0,
            primary_variant: config.1,
            secondary: config.2,
            secondary_variant: config.3,
            background: Color::Rgba32(0xFFFFFFFF),
            surface: Color::Rgba32(0xFFFFFFFF),
            error: Color::Rgba32(0xB00020FF),
            on_primary: Color::Rgba32(0xFFFFFFFF),
            on_secondary: Color::Rgba32(0x000000FF),
            on_background: Color::Rgba32(0x000000FF),
            on_surface: Color::Rgba32(0x000000FF),
            on_error: Color::Rgba32(0xFFFFFFFF),
        }
    }
    pub fn default() -> Self {
        Self {
            primary: Color::Rgba32(0x6200EEFF),
            primary_variant: Color::Rgba32(0x3700B3FF),
            secondary: Color::Rgba32(0x03DAC6FF),
            secondary_variant: Color::Rgba32(0x018786FF),
            background: Color::Rgba32(0xFFFFFFFF),
            surface: Color::Rgba32(0xFFFFFFFF),
            error: Color::Rgba32(0xB00020FF),
            on_primary: Color::Rgba32(0xFFFFFFFF),
            on_secondary: Color::Rgba32(0x000000FF),
            on_background: Color::Rgba32(0x000000FF),
            on_surface: Color::Rgba32(0x000000FF),
            on_error: Color::Rgba32(0xFFFFFFFF),
        }
    }
    pub fn with_surface_color(mut self, config: Color) -> Self{
        self.surface = config;
        self
    }
    pub fn with_background_color(mut self, config: Color) -> Self{
        self.background = config;
        self
    }
    
        pub fn with_error_color(mut self, config: Color) -> Self {
            self.error = config;
            self
        }
    pub fn with_on_color(mut self,config:(Color,Color,Color,Color,Color)) -> Self{
        self.on_primary = config.0;
        self.on_secondary = config.1;
        self.on_background = config.2;
        self.on_surface = config.3;
        self.on_error = config.4;
        self
    }

    pub fn setup_env(self,env: &mut Env) {
        env.set(druid::theme::BORDER_DARK, Color::Rgba32(0x222222FF));
        env.set(druid::theme::BORDER_LIGHT, Color::Rgba32(0x444444FF));
        env.set(druid::theme::WINDOW_BACKGROUND_COLOR, Color::Rgba32(0xFFFFFFFF));
        if env.get(druid::theme::WINDOW_BACKGROUND_COLOR) == Color::Rgba32(0xFFFFFFFF) {
            env.set(druid::theme::CURSOR_COLOR, Color::BLACK)
        } else { env.set(druid::theme::CURSOR_COLOR, Color::WHITE)}
        env.set(druid::theme::BACKGROUND_DARK, Color::Rgba32(0xFFFFFFFF));
        env.set(druid::theme::BACKGROUND_LIGHT, Color::Rgba32(0x000000FF));
        env.set(druid::theme::LABEL_COLOR, Color::Rgba32(0x000000FF));
        env.set(PRIMARY, self.primary.clone());
        env.set(PRIMARY_VARIANT, self.primary_variant.clone());
        env.set(SECONDARY, self.secondary.clone());
        env.set(SECONDARY_VARIANT, self.secondary_variant.clone());
        env.set(BACKGROUND, self.background.clone());
        env.set(SURFACE, self.surface.clone());
        env.set(ERROR, self.error.clone());
        env.set(ON_PRIMARY, self.on_primary.clone());
        env.set(ON_SECONDARY, self.on_secondary.clone());
        env.set(ON_BACKGROUND, self.on_background.clone());
        env.set(ON_SURFACE, self.on_surface.clone());
        env.set(ON_ERROR, self.on_error.clone());

    }
}