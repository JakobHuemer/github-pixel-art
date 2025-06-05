use indoc::indoc;
use inquire::ui::{Attributes, Color, RenderConfig, StyleSheet, Styled};

#[derive(Debug, Clone, PartialEq)]
pub enum HomeOption {
    ChooseProject,
    DeleteProject,
    OpenRemoteProject,
    Exit,
}

pub struct InteractionManager;

impl InteractionManager {
    pub fn welcome() {
        let welcome_message = indoc! {"

             ██████╗ ██╗████████╗██╗  ██╗██╗   ██╗██████╗
            ██╔════╝ ██║╚══██╔══╝██║  ██║██║   ██║██╔══██╗
            ██║  ███╗██║   ██║   ███████║██║   ██║██████╔╝
            ██║   ██║██║   ██║   ██╔══██║██║   ██║██╔══██╗
            ╚██████╔╝██║   ██║   ██║  ██║╚██████╔╝██████╔╝
             ╚═════╝ ╚═╝   ╚═╝   ╚═╝  ╚═╝ ╚═════╝ ╚═════╝
            ██████╗ ██╗██╗  ██╗███████╗██╗          █████╗ ██████╗ ████████╗
            ██╔══██╗██║╚██╗██╔╝██╔════╝██║         ██╔══██╗██╔══██╗╚══██╔══╝
            ██████╔╝██║ ╚███╔╝ █████╗  ██║         ███████║██████╔╝   ██║
            ██╔═══╝ ██║ ██╔██╗ ██╔══╝  ██║         ██╔══██║██╔══██╗   ██║
            ██║     ██║██╔╝ ██╗███████╗███████╗    ██║  ██║██║  ██║   ██║
            ╚═╝     ╚═╝╚═╝  ╚═╝╚══════╝╚══════╝    ╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝

        "};
        let indented_message = welcome_message
            .lines()
            .map(|line| format!("    {}", line))
            .collect::<Vec<_>>()
            .join("\n");
        println!("{}", indented_message);
    }

    pub fn home() {}
}

fn get_render_config() -> RenderConfig<'static> {
    let mut render_config = RenderConfig::default();
    render_config.prompt_prefix = Styled::new("$").with_fg(Color::LightRed);
    render_config.highlighted_option_prefix = Styled::new("➠").with_fg(Color::LightYellow);
    render_config.selected_checkbox = Styled::new("☑").with_fg(Color::LightGreen);
    render_config.scroll_up_prefix = Styled::new("⇞");
    render_config.scroll_down_prefix = Styled::new("⇟");
    render_config.unselected_checkbox = Styled::new("☐");

    render_config.error_message = render_config
        .error_message
        .with_prefix(Styled::new("❌").with_fg(Color::LightRed));

    render_config.answer = StyleSheet::new()
        .with_attr(Attributes::ITALIC)
        .with_fg(Color::LightYellow);

    render_config.help_message = StyleSheet::new().with_fg(Color::DarkYellow);

    render_config
}
