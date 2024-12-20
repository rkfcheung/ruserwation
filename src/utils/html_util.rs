use chrono::Datelike;
use maud::{html, Markup, DOCTYPE};

use crate::{restaurant::models::Restaurant, utils::env_util::is_prod};

pub fn render_html(restaurant: &Restaurant, body_content: Markup) -> Markup {
    let rest_name = restaurant.name.as_str();
    let rest_loc = restaurant.location.as_str();

    // Asset selection based on environment
    let (apple_touch_icon, favicon) = if is_prod() {
        ("apple-touch-icon_prod.png", "favicon_prod.ico")
    } else {
        ("apple-touch-icon.webp", "favicon.ico")
    };

    // Current year for the footer
    let current_year = chrono::Local::now().year();

    // Render HTML
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                title { "Welcome to " (rest_name) }

                 // Metadata for SEO and responsiveness
                meta name="viewport" content="width=device-width, initial-scale=1";
                meta name="description" content={ "Discover the finest dining experience at " (rest_name) ", located at " (rest_loc) ". Book your reservations now!" };
                meta name="keywords" content={ "" (rest_name) ", " (rest_loc) "" };

                link rel="apple-touch-icon" href=((format!("/static/images/{}", apple_touch_icon)));
                link rel="icon" href=((format!("/static/{}", favicon))) type="image/x-icon";
                link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap@5.2.3/dist/css/bootstrap.min.css" integrity="sha384-rbsA2VBKQhggwzxH7pPCaAqO46MgnOM80zW1RWuH61DGLwZJEdK2Kadq2F9CUG65" crossorigin="anonymous";
            }

            body class="bg-black text-white" {
                div class="container mt-4" {
                    (body_content)
                }

                div class="footer bg-dark text-center py-3 mt-4" {
                    p class="mb-0 text-light" { "© " (current_year) " " (rest_name) ". All rights reserved." }
                }
            }
        }
    }
}
