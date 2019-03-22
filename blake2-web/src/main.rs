use std::string::String;

extern crate yew;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

extern crate blake2;
use blake2::VarBlake2b;
use blake2::digest::{Input, VariableOutput};

extern crate hex;

struct Model {
    text: String,
    hash: String,
    digest_size: usize,
}

enum Msg {
    UpdateText(String),
    UpdateDigestSize(String),
}

impl Component for Model {
    // Some details omitted. Explore the examples to see more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model { 
            text: "".into(),
            hash: "".into(),
            digest_size: 32,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateText(val) => {
                self.text = val;
                self.update_hash();
                true
            }
            Msg::UpdateDigestSize(val) => {
                if val.is_empty() {
                    self.digest_size = 32
                } else {
                    self.digest_size = val.parse().unwrap();
                    if self.digest_size == 0 {
                        self.digest_size = 1;
                    } else if self.digest_size > 64 {
                        self.digest_size = 64;
                    }
                }
                self.update_hash();
                true
            }
        }
    }
}

impl Model {
    fn update_hash(&mut self) {
        let mut hasher = VarBlake2b::new(self.digest_size).unwrap();
        hasher.input(&self.text);
        hasher.variable_result(|res|{
             self.hash = hex::encode(res);
        });
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
            <header class="header",>
                <h1>{ "blake2 + WASM" }</h1>
            </header>
            <section class="main",>
                <div>
                    <p>{"Blake2b Digest Size (1-64)"}</p>
                    <input type="number", min=1, max=64,
                        value=&self.digest_size,
                        oninput=|e| Msg::UpdateDigestSize(e.value),/>
                    <p>{"Please enter a text to hash"}</p>
                    <textarea rows=5, cols=64,
                        value=&self.text,
                        oninput=|e| Msg::UpdateText(e.value),
                        placeholder={"Please enter a text to hash."},
                    ></textarea>
                </div>
                <div>
                    <textarea rows=5, cols=64,
                        placeholder={"Please enter a text to hash above."},
                    >{&self.hash}</textarea>
                </div>
            </section>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
