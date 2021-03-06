use crate::{
    basic_components::{
        Button, ButtonGroup, ButtonType, Select, Slider, Switch, TextField, UpDown,
    },
    components::{About, Connections, Installation, Logs, Settings, Statistics},
    translation::use_trans,
};
use alvr_common::{data::SessionDesc, prelude::*};
use std::rc::Rc;
use yew::{html, Callback, Properties};
use yew_functional::{function_component, use_state};

#[derive(Properties, Clone, PartialEq)]
pub struct DashboardProps {
    pub session: Rc<SessionDesc>,
}

#[function_component(Dashboard)]
pub fn dashboard(props: &DashboardProps) -> Html {
    let (selected_tab, set_selected_tab) = use_state(|| "connections".to_owned());

    let on_tab_click = Callback::from(move |name| set_selected_tab(name));

    let translation_on_click = Callback::from(move |_| {});

    html! {
        <div class="flex h-full">
            <aside class="w-44 bg-gray-100">
                <nav class="flex flex-col items-start h-full py-4 space-y-2">
                    <MenuIcon
                        name="connections"
                        icon="fas fa-plug"
                        on_click=on_tab_click.clone()
                        selected=*selected_tab=="connections"
                    />
                    <MenuIcon
                        name="statistics"
                        icon="fas fa-chart-bar"
                        on_click=on_tab_click.clone()
                        selected=*selected_tab=="statistics"
                    />
                    // <MenuIcon
                    //     name="presets"
                    //     icon="fas fa-th-large"
                    //     on_click=on_tab_click.clone()
                    //     selected=*selected_tab=="presets"
                    // />
                    <MenuIcon
                        name="settings"
                        icon="fas fa-cog"
                        on_click=on_tab_click.clone()
                        selected=*selected_tab=="settings"
                    />
                    <MenuIcon
                        name="installation"
                        icon="fas fa-hdd"
                        on_click=on_tab_click.clone()
                        selected=*selected_tab=="installation"
                    />
                    <MenuIcon
                        name="logs"
                        icon="fas fa-th-list"
                        on_click=on_tab_click.clone()
                        selected=*selected_tab=="logs"
                    />
                    <MenuIcon
                        name="about"
                        icon="fas fa-info-circle"
                        on_click=on_tab_click.clone()
                        selected=*selected_tab=="about"
                    />
                    <MenuIcon
                        name="test"
                        icon="fas fa-asterisk"
                        on_click=on_tab_click.clone()
                        selected=*selected_tab=="test"
                    />
                    <MenuIcon
                        name="language"
                        icon="fas fa-globe"
                        on_click=translation_on_click
                        selected=false
                        class="mt-auto"
                    />
                </nav>
            </aside>
            <div class="flex-grow">
                <div hidden=*selected_tab!="connections">
                    <Connections session=Rc::clone(&props.session) />
                </div>
                <div hidden=*selected_tab!="statistics">
                    <Statistics />
                </div>
                <div hidden=*selected_tab!="settings">
                    <Settings session=Rc::clone(&props.session) />
                </div>
                <div hidden=*selected_tab!="installation">
                    <Installation session=Rc::clone(&props.session)/>
                </div>
                <div hidden=*selected_tab!="logs">
                    <Logs />
                </div>
                <div hidden=*selected_tab!="about">
                    <About session=Rc::clone(&props.session) />
                </div>
                <div hidden=*selected_tab!="test">
                    <Test />
                </div>
            </div>
        </div>

    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct MenuIconProps {
    pub name: String,
    pub icon: String,
    pub on_click: Callback<String>,
    pub selected: bool,

    #[prop_or_default]
    pub class: String,
}

#[function_component(MenuIcon)]
pub fn menu_icon(props: &MenuIconProps) -> Html {
    let (tooltip_visible, set_tooltip_visible) = use_state(|| false);

    let on_enter = {
        let set_tooltip_visible = Rc::clone(&set_tooltip_visible);
        Callback::from(move |_| set_tooltip_visible(true))
    };

    let on_leave = Callback::from(move |_| set_tooltip_visible(false));

    let on_click = {
        let on_click = props.on_click.clone();
        let name = props.name.clone();
        Callback::from(move |_| on_click.emit(name.clone()))
    };

    html! {
        <div
            class=format!(
                "w-40 flex items-center rounded-r-lg pr-3 pl-7 py-1 space-x-2 {} {} {} {}",
                "bg-gray-300 cursor-pointer transition transform -translate-x-4 overflow-hidden",
                "whitespace-nowrap hover:bg-gray-400 hover:translate-x-0 hover:shadow-md",
                if props.selected {
                    format!(
                        "w-44 bg-gradient-to-tr from-blue-700 via-blue-700 to-blue-600 {}",
                        "hover:bg-blue-800 text-white shadow-md"
                    )
                } else {
                    "".into()
                },
                props.class.clone()
            )
            onmouseenter=on_enter
            onmouseleave=on_leave
            onclick=on_click
        >
            <i
                class=format!(
                    "w-8 opacity-75 {} {}",
                    if props.selected { "opacity-90" } else { "" },
                    props.icon.clone()
                )
            />
            <span class="font-medium truncate">{use_trans(&props.name)}</span>
        </div>
    }
}

#[function_component(Test)]
pub fn test() -> Html {
    let (label, set_label) = use_state(|| "Hello".to_owned());

    let on_click = {
        let label = Rc::clone(&label);
        Callback::from(move |_| set_label(format!("{} world", label)))
    };

    let default_string = use_trans("default");

    let switch_on_click = Callback::from(move |_| ());

    let slider_on_change = Callback::from(move |_| ());

    let on_select = Callback::from(move |_| ());

    let text_field_on_focus_lost = Callback::from(move |_| ());

    let up_down_on_step = Callback::from(move |_| ());

    html! {
        <div class="px-4 py-3">
            <div class="flex flex-col space-y-2 items-start">
                <Button on_click=on_click.clone() button_type=ButtonType::None>
                    {label.clone()}
                </Button>
                <Button on_click=on_click.clone() button_type=ButtonType::Primary>
                    {label.clone()}
                </Button>
                <Button on_click=on_click.clone() button_type=ButtonType::Secondary>
                    {label.clone()}
                </Button>
                <Button on_click=on_click button_type=ButtonType::Danger>
                    {label}
                </Button>
            </div>
            <Switch on_click=switch_on_click checked=true/>
            <Slider value="0" default="30" min="-1" max="40" step="0.5" on_change=slider_on_change/>
            <ButtonGroup
                options=vec!["hello1".into(), "hello2".into()]
                selected="hello1"
                on_select=on_select.clone()
            />
            <Select
                options=vec!["hello1".into(), "hello2".into()]
                selected="hello1"
                on_select=on_select
            />
            <div class="space-y-2">
                <TextField
                    value=default_string.clone()
                    on_focus_lost=text_field_on_focus_lost.clone()
                />
                <TextField
                    label="Hi there"
                    value=default_string
                    on_focus_lost=text_field_on_focus_lost.clone()
                />
            </div>
            <div class="py-2 space-y-2">
                <UpDown
                    label="Bitrate"
                    value="123"
                    on_focus_lost=text_field_on_focus_lost.clone()
                    on_step_down=up_down_on_step.clone()
                    on_step_up=up_down_on_step.clone()
                />
                <UpDown
                    value="123"
                    on_focus_lost=text_field_on_focus_lost
                    on_step_down=up_down_on_step.clone()
                    on_step_up=up_down_on_step
                />
            </div>
        </div>
    }
}
