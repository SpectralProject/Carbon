use iced::{Text, Column, Settings, Sandbox, Align, Element};

fn main() {
    // uses rust-std library
    // writes to the screen line by line, up to the driver to display it properly
    // uses black font by default, cant change font size and colors through "userspace module" Display::FontAPI

    Terminal::run(Settings::default());
}

#[derive(Default)]
struct Terminal {
    // display a multiline string
    text: String,

    // other options, e.g. background color, text color
    background_color: String,
    text_color: String,
}


#[derive(Debug, Clone, Copy)]
enum TerminalMessage {
    ChangeText,
}

impl Sandbox for Terminal {
    type Message = TerminalMessage;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Terminal")
    }

    fn update(&mut self, message: Self::Message) {
        self.text += "> \n";
        match message {
            ChangeText => self.text += "> \n"
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        Column::new()
        .padding(10)
        .align_items(Align::Center)
        .push(
            Text::new(self.text.as_str()).size(50),
        )
        .into()
    }
}
