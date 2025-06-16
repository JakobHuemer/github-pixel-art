use indoc::indoc;
use inquire::Select;
use std::error::Error;
use std::fmt::{Display, Formatter};

pub enum HomeOption {
    CreateProject,
    OpenProject,
    OpenRemoteProject,
    Exit,
}

struct SelectItem<T> {
    message: String,
    value: T,
}

impl<T> SelectItem<T> {
    fn new(message: &str, value: T) -> SelectItem<T> {
        SelectItem {
            message: message.to_string(),
            value,
        }
    }
}

impl<T> Display for SelectItem<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub struct InteractionManager {}

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

    pub fn home() -> Result<HomeOption, Box<dyn Error>> {
        let options = vec![
            SelectItem::new("Create New Project", HomeOption::CreateProject),
            SelectItem::new("Open Local Project", HomeOption::OpenProject),
            SelectItem::new("Open Remote Project", HomeOption::OpenRemoteProject),
            SelectItem::new("Exit", HomeOption::Exit),
        ];

        let choice = Select::new("Main Menu", options).prompt()?;

        Ok(choice.value)
    }
}

// TODO: maybe use this styling or reference it for own styles
// fn get_render_config() -> RenderConfig<'static> {
//     let mut render_config = RenderConfig::default();
//     render_config.prompt_prefix = Styled::new("$").with_fg(Color::LightRed);
//     render_config.highlighted_option_prefix = Styled::new("➠").with_fg(Color::LightYellow);
//     render_config.selected_checkbox = Styled::new("☑").with_fg(Color::LightGreen);
//     render_config.scroll_up_prefix = Styled::new("⇞");
//     render_config.scroll_down_prefix = Styled::new("⇟");
//     render_config.unselected_checkbox = Styled::new("☐");
//
//     render_config.error_message = render_config
//         .error_message
//         .with_prefix(Styled::new("❌").with_fg(Color::LightRed));
//
//     render_config.answer = StyleSheet::new()
//         .with_attr(Attributes::ITALIC)
//         .with_fg(Color::LightYellow);
//
//     render_config.help_message = StyleSheet::new().with_fg(Color::DarkYellow);
//
//     render_config
// }

/*
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
*/
