use std::rc::Rc;
use yew::{html, Callback, InputData, Properties};
use yew_functional::{function_component, use_state};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub value: String,

    #[prop_or_default]
    pub label: String,
    
    #[prop_or("1".to_string())]
    pub step: String,

    pub on_focus_lost: Callback<String>,
    pub on_step_down: Callback<String>,
    pub on_step_up: Callback<String>,
    // pub on_focus_lost: Callback<String>,
    // pub on_step_down: Callback<String>,
    // pub on_step_up: Callback<String>,
}

#[function_component(UpDown)]
pub fn up_down(props: &Props) -> Html {
    let step = props.step.clone();
    let value = props.value.clone();
    let on_focus_lost = props.on_focus_lost.clone();
    let on_step_down = props.on_step_down.clone();
    let on_step_up = props.on_step_up.clone();

    let (value, set_value) = use_state(|| value);

    let on_input = {
        let set_value = Rc::clone(&set_value);
        Callback::from(move |data: InputData| set_value(data.value))
    };
    // let on_step_down = {
    //     let value = Rc::clone(&value);
    //     let set_value = Rc::clone(&set_value);
    //     let new_value = (value.parse::<i32>().unwrap() - step).to_string();
    //     Callback::from(move |_| {set_value(format!("{}", new_value)); on_step_down.emit(format!("{}", new_value))})
    //     // Callback::from(move |_| {set_value(format!("{}222", value)); on_step_down.emit(format!("{}222", value))})
    // };
    // let on_step_up = {
    //     let value = Rc::clone(&value);
    //     let set_value = Rc::clone(&set_value);
    //     let new_value = (value.parse::<i32>().unwrap() + step).to_string();
    //     Callback::from(move |_| {set_value(format!("{}", new_value)); on_step_up.emit(format!("{}", new_value))})
    //     // Callback::from(move |_| {set_value(format!("{}111", value)); on_step_up.emit(format!("{}111", value))})
    // };
    let on_click_down = {
        let value = Rc::clone(&value);
        let set_value = Rc::clone(&set_value);
        let new_value = "1".parse::<i32>();
        // let new_value = *value - step;
        // Callback::from(move |_| {set_value(new_value); on_step_down.emit(new_value)})
        Callback::from(move |_| {set_value(format!("{}222", value)); on_step_down.emit(format!("{}222", value))})
    };
    let on_click_up = {
        let value = Rc::clone(&value);
        let set_value = Rc::clone(&set_value);
        // let new_value = value + step;
        // Callback::from(move |_| {set_value(new_value); on_step_up.emit(new_value)})
        Callback::from(move |_| {set_value(format!("{}111", value)); on_step_up.emit(format!("{}111", value))})
    };

    let on_focus_lost = {
        let value = Rc::clone(&value);
        Callback::from(move |_| on_focus_lost.emit(value.as_ref().clone()))
    };

    html! {
        <div>
            {
                if !props.label.is_empty() {
                    html! {
                        <label class="block text-sm text-gray-700 font-medium">
                            {props.label.clone()}
                        </label>
                    }
                } else {
                    html! {}
                }
            }
            <div class="flex shadow-sm">
                <button
                    class="rounded-l border text-gray-500 hover:bg-gray-200 p-1 w-8"
                    // onclick=Callback::from(move |_| on_step_down.emit(()))
                    onclick=on_click_down
                >
                    <i class="fas fa-minus" />
                </button>
                // todo: adapt size to content
                <input
                    class="border-t border-b  px-2 py-1 flex-1"
                    type="number"
                    value=*value
                    oninput=on_input
                    onblur=on_focus_lost
                    step=step
                />
                <button
                    class="rounded-r border text-gray-500 hover:bg-gray-200 p-1 w-8"
                    // onclick=Callback::from(move |_| on_step_up.emit(()))
                    onclick=on_click_up
                >
                    <i class="fas fa-plus" />
                </button>
            </div>
        </div>
    }
}
