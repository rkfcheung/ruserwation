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

                meta name="viewport" content="width=device-width, initial-scale=1";
                meta name="keywords" content=(rest_details);

                link rel="icon" href="/static/favicon.ico" type="image/x-icon";
                link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap@5.2.3/dist/css/bootstrap.min.css" integrity="sha384-rbsA2VBKQhggwzxH7pPCaAqO46MgnOM80zW1RWuH61DGLwZJEdK2Kadq2F9CUG65" crossorigin="anonymous";
            }

            body {
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
