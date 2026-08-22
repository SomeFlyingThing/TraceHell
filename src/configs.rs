use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
struct Configs{
    terminal: Terminals,
}

#[derive(Serialize,Deserialize)]
enum Terminals{
    Alacritty,
    Kitty,
    GnomeTerminal,
}