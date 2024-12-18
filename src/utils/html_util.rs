use chrono::Datelike;
use maud::{html, Markup, DOCTYPE};

use crate::{restaurant::models::Restaurant, utils::env_util::is_prod};

pub fn render_html(restaurant: &Restaurant, body_content: Markup) -> Markup {
    let rest_details = format!("{}, {}", restaurant.name, restaurant.location);
    let (apple_touch_icon, favicon) = if is_prod() {
        ("apple-touch-icon_prod.png", "favicon_prod.ico")
    } else {
        ("apple-touch-icon.webp", "favicon.ico")
    };
    let current_year = chrono::Local::now().year();

    html! {
        (DOCTYPE)
        html {
            head {
                title { "Welcome to " (restaurant.name) }

                meta name="viewport" content="width=device-width, initial-scale=1";
                meta name="keywords" content=(rest_details);

                link rel="apple-touch-icon" href=((format!("/static/images/{}", apple_touch_icon)));
                link rel="icon" href=((format!("/static/{}", favicon))) type="image/x-icon";
                link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap@5.2.3/dist/css/bootstrap.min.css" integrity="sha384-rbsA2VBKQhggwzxH7pPCaAqO46MgnOM80zW1RWuH61DGLwZJEdK2Kadq2F9CUG65" crossorigin="anonymous";
            }

            body class="bg-black text-white" {
                div class="container" {
                    (body_content)
                }

                div class="footer" {
                    p class="text-center" { "© " (current_year) " " (restaurant.name) "." }
                }
            }
        }
    }
}
