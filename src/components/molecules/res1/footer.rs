use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::GlobalState;

#[function_component(Footer)]
pub fn footer() -> Html {
    let (state, _) = use_store::<GlobalState>();

    let socials = state.socials.iter().map(|social| {
        let link = format!("{}", social.link);
        let name = format!("{}", social.name);
        html! {
            <a href={link} class="text-white-50">{name}</a>
        }
    });

    let len = socials.len();

    let with_seperator: Vec<Html> = socials
        .flat_map(|elem| [elem, html! {<span class="mx-2">{"|"}</span>}])
        .take(len * 2 - 1)
        .collect();

    html! {
      <footer class="bg-dark text-white-50 text-center mt-5 p-3">
      {"\u{00a9} 2023 "}{&state.username}{" - "}
        { for with_seperator }
      </footer>
    }
}
