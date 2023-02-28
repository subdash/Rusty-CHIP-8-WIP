use cursive::views::{ Dialog, LinearLayout, Panel };
use crate::interpreter::Interpreter;

mod interpreter;

fn main()
{
    let mut interpreter = Interpreter::new();
    interpreter.initialize();

    let mut siv = cursive::default();
    siv.set_fps(16);
    siv.add_global_callback('.', |s| s.quit());

    siv.add_layer(
        Dialog::new()
            .title("CHIP-8")
            .content(LinearLayout::vertical().child(Panel::new(interpreter)))
    );

    siv.run();
}
