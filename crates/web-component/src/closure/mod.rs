#[macro_export]
macro_rules! capture {
    ($($name:ident),* => $body:expr) => {
        {
            $(let $name = $name.clone();)*
            $body
        }
    };
}

#[macro_export]
macro_rules! event {
    ($($name:ident),* => async move |$input:ident: $type_:ty| $body:block) => {
        {
            $(let $name = $name.clone();)*
            move |$input: $type_| {
                $(let $name = $name.clone();)*
                wasm_bindgen_futures::spawn_local(async move {
                    $body
                })
            }
        }
    };
    ($($name:ident),* => async move $body:block) => {
        {
            $(let $name = $name.clone();)*
            move |_| {
                $(let $name = $name.clone();)*
                wasm_bindgen_futures::spawn_local(async move {
                    $body
                })
            }
        }
    };
}