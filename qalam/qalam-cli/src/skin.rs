use termimad::crossterm::style::Attribute::Underlined;
use termimad::crossterm::style::Color::Yellow;
use termimad::MadSkin;

pub fn wrapper() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.bold.set_fg(Yellow);
    skin.italic.add_attr(Underlined);
    skin
}
