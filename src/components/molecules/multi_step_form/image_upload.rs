use gloo_file::{callbacks::FileReader, File};
use std::collections::HashMap;
use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;
use yewdux::prelude::Dispatch;

use crate::GlobalState;
use base64::{engine::general_purpose, Engine as _};

pub enum Msg {
    LoadedBytes(String, Vec<u8>),
    Files(Vec<File>),
}
pub struct FileDataComponent {
    readers: HashMap<String, FileReader>,
    dispatch: Dispatch<GlobalState>,
}

impl Component for FileDataComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            readers: HashMap::default(),
            dispatch: Dispatch::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Files(files) => {
                for file in files.into_iter() {
                    let file_name = file.name();
                    let task = {
                        let file_name = file_name.clone();
                        let link = ctx.link().clone();
                        gloo_file::callbacks::read_as_bytes(&file, move |res| {
                            link.send_message(Msg::LoadedBytes(
                                file_name,
                                res.expect("failed to read file"),
                            ))
                        })
                    };
                    self.readers.insert(file_name, task);
                }
                true
            }
            Msg::LoadedBytes(file_name, data) => {
                let image_data = general_purpose::STANDARD.encode(data);
                self.dispatch
                    .reduce_mut(move |state| state.image_data = image_data);
                self.readers.remove(&file_name);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change = ctx.link().callback(move |e: Event| {
            let mut selected_files = Vec::new();
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(files) = input.files() {
                let files = js_sys::try_iter(&files)
                    .unwrap()
                    .unwrap()
                    .map(|v| web_sys::File::from(v.unwrap()))
                    .map(File::from);
                selected_files.extend(files);
            }
            Msg::Files(selected_files)
        });

        html! {
            <div>
                <div class="mb-3">
                  <label class="form-label">{"Please select your profile picture!"}</label>
                  <input class="form-control" type="file" accept="image/png, image/gif, image/jpeg" onchange={on_change} multiple=false/>
                </div>
            </div>
        }
    }
}
