use maud::{html, Markup, DOCTYPE};
use warp::Filter;

use crate::utils::env_util::{var_as_str, var_as_str_or};

use super::models::Restaurant;

pub fn index_route(
    restaurant: Restaurant,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end().map(move || warp::reply::html(render_index(restaurant.clone()).into_string()))
}

fn render_index(restaurant: Restaurant) -> Markup {
    let app_env = var_as_str("APP_ENV");
    let poster = var_as_str_or("RW_POSTER", "poster.webp".to_string());
    let rest_details = format!("{}, {}", restaurant.name, restaurant.location);

    let (apple_touch_icon, favicon) = if app_env == "prod" {
        ("apple-touch-icon_prod.png", "favicon_prod.ico")
    } else {
        ("apple-touch-icon.webp", "favicon.ico")
    };

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
                    div class="row" {
                        div class="col-md-12 text-center" {
                            img src=(format!("/static/images/{}", poster)) alt=(rest_details) class="img-fluid" {}
                        }
                    }
                }
            }
        }
    }
}
