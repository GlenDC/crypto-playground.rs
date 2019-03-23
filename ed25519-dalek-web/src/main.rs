use std::string::String;

extern crate yew;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

extern crate bip39;
use bip39::{Mnemonic, Language};

extern crate rand;
use rand::{SeedableRng, StdRng};

extern crate ed25519_dalek;
use ed25519_dalek::{Keypair};

extern crate hex;

struct Model {
    mnemonic: Option<Mnemonic>,
    seed: String,
    error: String,
    key_pair: Option<Keypair>,
}

enum Msg {
    UpdateMnemonic(String),
    UpdateSeed(String),
}

impl Component for Model {
    // Some details omitted. Explore the examples to see more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model { 
            mnemonic: None,
            seed: "".into(),
            error: "".into(),
            key_pair: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateMnemonic(val) => {
                //update seed and mnemonic
                if val.is_empty() {
                    self.seed = "".into();
                    self.mnemonic = None;
                    self.error = "".into();
                    self.update_key_pair();
                } else {
                    match Mnemonic::from_phrase(val, Language::English) {
                        Ok(b) => {
                            self.seed = hex::encode(b.entropy());
                            self.mnemonic = Some(b);
                            self.error = "".into();
                            self.update_key_pair();
                        },
                        Err(e) => {
                            self.error = e.to_string();
                        },
                    };
                };
            }
            Msg::UpdateSeed(val) => {
                //update seed and mnemonic
                if val.is_empty() {
                    self.seed = "".into();
                    self.mnemonic = None;
                    self.error = "".into();
                    self.update_key_pair();
                } else {
                    match hex::decode(&val) {
                        Ok(entropy) => match Mnemonic::from_entropy(entropy.as_slice(), Language::English) {
                            Ok(b) => {
                                self.seed = val;
                                self.mnemonic = Some(b);
                                self.error = "".into();
                                self.update_key_pair();
                            },
                            Err(e) => {
                                self.error = e.to_string();
                            },
                        },
                        Err(e) => {
                            self.error = e.to_string();
                        },
                    };
                };
            }
        };
        true
    }
}

impl Model {
    fn update_key_pair(&mut self) {
        if self.seed.is_empty() {
            return;
        }
        let data = hex::decode(self.seed.as_bytes()).unwrap();
        let mut array = [0; 32];
        array.copy_from_slice(data.as_slice()); 
        let mut rng: StdRng = SeedableRng::from_seed(array);
        let kp = Keypair::generate(&mut rng);
        self.key_pair = Some(kp);
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
            <header class="header",>
                <h1>{ "BIP39 + ED25519_dalek + WASM" }</h1>
            </header>
            <section class="main",>
                <div>
                    <p>{"Mnemonic (24 words):"}</p>
                    <textarea rows=5, cols=64,
                        oninput=|e| Msg::UpdateMnemonic(e.value),
                        placeholder={"Please enter your 24 words here or a seed in hex-format below."},
                    >{match &self.mnemonic {
                        Some(b) => b.phrase(),
                        None => "",
                    }}</textarea><br/>
                    <p>{"Seed (hex-encoded):"}</p>
                    <textarea rows=5, cols=64,
                        value=&self.seed,
                        oninput=|e| Msg::UpdateSeed(e.value),
                        placeholder={"Please enter a seed in hex-format here or a mnemonic above."},
                    >{&self.seed}</textarea>
                </div>
                <div>
                    <p>{"Error:"}</p>
                    <textarea rows=5, cols=64,
                        placeholder={"All Good!"},
                    >{&self.error}</textarea>
                </div>
            </section>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
