use maud::{html,Markup};

pub async fn index() -> Markup {
    html! {
        link rel="stylesheet" href="style.css";
        h1 { "Hello, World!" }
    }
}