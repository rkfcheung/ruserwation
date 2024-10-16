use maud::{html, Markup, DOCTYPE};
use std::env;
use warp::Filter;

use super::models::Restaurant;

pub fn index_route(
    restaurant: Restaurant,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end().map(move || warp::reply::html(render_index(restaurant.clone()).into_string()))
}

fn render_index(restaurant: Restaurant) -> Markup {
    let poster = env::var("RW_POSTER").unwrap_or_else(|_| "poster.webp".to_string());
    let rest_details = format!("{}, {}", restaurant.name, restaurant.location);

    html! {
        (DOCTYPE)
        html {
            head {
                title { "Welcome to " (restaurant.name) }

                meta name="keywords" content=(rest_details);

                link rel="stylesheet" href="https://stackpath.bootstrapcdn.com/bootstrap/4.5.2/css/bootstrap.min.css";
            }

            body {
                div class="container" {
                    div class="row" {
                        div class="col-md-12 text-center" {
                            img src=(format!("/static/{}", poster)) alt=(rest_details) class="img-fluid" {}
                        }
                    }
                }
            }
        }
    }
}
