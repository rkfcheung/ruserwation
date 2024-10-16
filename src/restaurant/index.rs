use maud::{html, Markup, DOCTYPE};
use warp::Filter;

use super::models::Restaurant;

pub fn index_route(
    restaurant: Restaurant,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end().map(move || warp::reply::html(render_index(restaurant.clone()).into_string()))
}

fn render_index(restaurant: Restaurant) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                title { "Welcome to " (restaurant.name) }

                link rel="stylesheet" href="https://stackpath.bootstrapcdn.com/bootstrap/4.5.2/css/bootstrap.min.css";
            }

            body {
                div class="container" {
                    div class="row" {
                        div class="col-md-12" {
                            img src="/static/poster.webp" alt=(restaurant.name) class="img-fluid" {}
                        }
                    }
                }
            }
        }
    }
}
