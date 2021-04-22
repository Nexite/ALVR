use crate::{
    basic_components::{
        Button, ButtonGroup, ButtonType, Select, Slider, Switch, TextField, UpDown,
    },
    translation::use_trans,
};
use alvr_common::{
    data::session_settings_default, data::SessionDesc, data::Settings, logging::Event, prelude::*,
};
use serde_json as json;
use settings_schema::{EntryType, SchemaNode};
use std::{cell::RefCell, rc::Rc};
use yew::{html, virtual_dom::VNode, Callback, Properties};
use yew_functional::{function_component, use_state};

fn extrapolate_session_settings_from_session_settings(schema: &SchemaNode) -> VNode {
    let sections: Vec<VNode> = Vec::new();
    match schema {
        SchemaNode::Section(entries) => {
            let (name, data) = &entries[0];
            log::info!("{}", name.to_string());
            let section = html! {
                <div class="flex flex-wrap overflow-hidden md:w-auto">
                    <div class="overflow-hidden bg-gray-300">
                        {name}
                    </div>
                    <div class="overflow-hidden bg-gray-300">
                        {name}
                    </div>
                    <div class="overflow-hidden bg-gray-300">
                        {name}
                    </div>
                </div>
            };

            return section;
        }
        SchemaNode::Choice {
            default,
            variants,
            gui,
        } => html! {},
        SchemaNode::Optional {
            default_set,
            content,
        } => html! {},
        SchemaNode::Switch {
            default_enabled,
            content_advanced,
            content,
        } => html! {},
        SchemaNode::Boolean { default } => html! {},
        SchemaNode::Integer {
            default,
            min,
            max,
            step,
            gui,
        } => html! {},
        SchemaNode::Float {
            default,
            min,
            max,
            step,
            gui,
        } => html! {},
        SchemaNode::Text { default } => html! {},
        SchemaNode::Array(_) => html! {},
        SchemaNode::Vector {
            default_element,
            default,
        } => html! {},
        SchemaNode::Dictionary {
            default_key,
            default_value,
            default,
        } => html! {},
    }
}
#[derive(Properties, Clone, PartialEq)]
pub struct SettingsProps {
    pub session: SessionDesc,
}

#[function_component(SettingsMenu)]
pub fn settings_menu(props: &SettingsProps) -> Html {
    let (label, set_label) = use_state(|| "Hello".to_owned());
    let schema = Settings::schema(session_settings_default());

    // let text_field_on_focus_lost = Callback::from(move |_| ());

    // let up_down_on_step = Callback::from(move |_| ());
    // let on_step_down = Callback::from(move |value| log::info!("step down, value: {}", value));
    // let on_step_up = Callback::from(move |value| log::info!("step up, value: {}", value));
    html! {
        <div style="border-bottom: 2px solid #eaeaea">
            <ul class="flex cursor-pointer">
                <li class="py-2 px-6 bg-white rounded-t-lg">{"Personal"}</li>
                <li class="py-2 px-6 bg-white rounded-t-lg text-gray-500 bg-gray-200">{"Akun"}</li>
                <li class="py-2 px-6 bg-white rounded-t-lg text-gray-500 bg-gray-200">{"Pengaturan"}</li>
            </ul>
        </div>
    }
}
