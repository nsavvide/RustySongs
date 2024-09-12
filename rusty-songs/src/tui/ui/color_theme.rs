use tui::style::Color;

pub struct ColorTheme {
    pub background: Color,
    pub text: Color,
    pub highlight: Color,
    pub accent1: Color,
    pub accent2: Color,
    pub error: Color,
    pub success: Color,
}

impl ColorTheme {
    pub fn catppuccin_mocha() -> Self {
        ColorTheme {
            background: Color::Rgb(30, 30, 46),   // #1E1E2E
            text: Color::Rgb(217, 224, 238),      // #D9E0EE
            highlight: Color::Rgb(245, 224, 220), // #F5E0DC
            accent1: Color::Rgb(242, 205, 205),   // #F2CDCD
            accent2: Color::Rgb(148, 226, 213),   // #94E2D5
            error: Color::Rgb(243, 139, 168),     // #F38BA8
            success: Color::Rgb(166, 227, 161),   // #A6E3A1
        }
    }
}
