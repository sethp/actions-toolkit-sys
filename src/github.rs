#![allow(missing_docs)]

use js_sys::{JsString, Object, Promise};
use wasm_bindgen::prelude::*;

pub mod context {
    #![allow(missing_docs)]

    use js_sys::{JsString, Number, Object};
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern {
        pub type Context;

        #[wasm_bindgen(method, getter)]
        pub fn payload(this: &Context) -> Object;

        #[wasm_bindgen(method, getter, js_name = "eventName")]
        pub fn event_name(this: &Context) -> JsString;

        #[wasm_bindgen(method, getter)]
        pub fn sha(this: &Context) -> JsString;

        #[wasm_bindgen(method, getter, js_name = "ref")]
        pub fn _ref(this: &Context) -> JsString;

        #[wasm_bindgen(method, getter)]
        pub fn workflow(this: &Context) -> JsString;

        #[wasm_bindgen(method, getter)]
        pub fn action(this: &Context) -> JsString;

        #[wasm_bindgen(method, getter)]
        pub fn actor(this: &Context) -> JsString;

        #[wasm_bindgen(method, getter)]
        pub fn job(this: &Context) -> JsString;

        #[wasm_bindgen(method, getter, js_name = "runNumber")]
        pub fn run_number(this: &Context) -> Number;

        #[wasm_bindgen(method, getter, js_name = "runId")]
        pub fn run_id(this: &Context) -> Number;

        #[wasm_bindgen(method, getter, catch)]
        pub fn issue(this: &Context) -> Result<Issue, JsValue>;

        #[wasm_bindgen(method, getter, catch)]
        pub fn repo(this: &Context) -> Result<Repo, JsValue>;
    }

    #[wasm_bindgen]
    extern {
        pub type Issue;

        #[wasm_bindgen(method, getter)]
        pub fn owner(this: &Issue) -> JsString;

        #[wasm_bindgen(method, getter)]
        pub fn repo(this: &Issue) -> JsString;

        #[wasm_bindgen(method, getter)]
        pub fn number(this: &Issue) -> Number;
    }

    #[wasm_bindgen]
    extern {
        pub type Repo;

        #[wasm_bindgen(method, getter)]
        pub fn owner(this: &Repo) -> JsString;

        #[wasm_bindgen(method, getter)]
        pub fn repo(this: &Repo) -> JsString;
    }
}
pub use context::Context;

#[wasm_bindgen(module = "@actions/github")]
extern {
    #[wasm_bindgen(extends = Object)] // FIXME: extends Octokit
    pub type GitHub;

    #[wasm_bindgen(method)]
    pub fn graphql(this: &GitHub, query: &JsString, variables: Option<&Object>) -> Promise;

    #[wasm_bindgen(constructor)]
    pub fn new(token: &JsString, options: Option<Object>) -> GitHub;

    pub static context: Context;
}
