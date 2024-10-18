use maud::{html, Markup};
use warp::Filter;

use crate::utils::{
    env_util::{var_as_bool_or, var_as_str_or},
    html_util::render_html,
};

use super::models::Restaurant;

pub fn index_route(
    restaurant: Restaurant,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end().map(move || warp::reply::html(render_index(restaurant.clone()).into_string()))
}

fn render_index(restaurant: Restaurant) -> Markup {
    let poster = var_as_str_or("RW_POSTER", "poster.webp".to_string());
    let is_und_constr = var_as_bool_or("RW_UNDER_CONSTRUCTION", false);
    let rest_details = format!("{}, {}", restaurant.name, restaurant.location);

    let div_content = if is_und_constr {
        html! {
            h1 class="mt-5" { ">> Under Construction <<" }
            p { "We're currently working hard to launch something awesome. Stay tuned!" }
        }
    } else {
        html! {
            img src=(format!("/static/images/{}", poster)) alt=(rest_details) class="img-fluid" {}
        }
    };
    let body_content = html! {
        div class="row" {
            div class="col-md-12 text-center" {
                (div_content)
            }
        }
    };

    render_html(restaurant, body_content)
}
