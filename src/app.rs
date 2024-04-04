use egui::Id;

use crate::wallet_api::{get_network, AccountTokensResponse, ApiService, Network, Token};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    pub wallet_address: String,
    pub api_key: String,
    pub tokens: Vec<Token>,
    pub network: Network,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            wallet_address: "".to_owned(),
            api_key: "".to_owned(),
            tokens: vec![],
            network: Network::Mainnet,
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>, api_key: String) -> Self {
        if let Some(storage) = cc.storage {
            if let Some(app) = eframe::get_value::<TemplateApp>(storage, eframe::APP_KEY) {
                if app.api_key.is_empty() {
                    return TemplateApp {
                        wallet_address: "".to_owned(),
                        api_key,
                        tokens: vec![],
                        network: Network::Mainnet,
                    };
                }

                return app;
            }
        }

        return TemplateApp {
            wallet_address: "".to_owned(),
            api_key,
            tokens: vec![],
            network: Network::Mainnet,
        };
    }
}

impl std::fmt::Display for TemplateApp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Wallet address: {}\n API key: {}",
            self.wallet_address, self.api_key
        )
    }
}

impl eframe::App for TemplateApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("Settings", |ui| {
                        egui::widgets::global_dark_light_mode_buttons(ui);
                        ui.separator();

                        ui.horizontal(|ui| {
                            ui.label("API key: ");
                            ui.text_edit_singleline(&mut self.api_key);
                        });

                        ui.add_space(30.0);

                        let reset_button = ui.button("Reset").on_hover_text("Reset the app state");

                        if reset_button.clicked() {
                            *self = Default::default();
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::warn_if_debug_build(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Check your wallet ballance");

                ui.menu_button(get_network(&self.network), |ui| {
                    ui.selectable_value(&mut self.network, Network::Mainnet, "Mainnet");
                    ui.selectable_value(&mut self.network, Network::Devnet, "Devnet");
                });
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Wallet address: ");
                ui.text_edit_singleline(&mut self.wallet_address);
                let check_button = ui.button("Check");
                let clear_button = ui.button("Clear").labelled_by(Id::new("clear_button"));

                if clear_button.clicked() {
                    self.wallet_address.clear();
                }

                if check_button.clicked() {
                    let tokens = fetch_wallet_balance(
                        &self.network,
                        self.wallet_address.clone(),
                        self.api_key.clone(),
                    );

                    self.tokens = tokens;
                }
            });

            ui.add_space(15.0);

            ui.separator();

            egui::ScrollArea::vertical()
                .id_source("scroll_area")
                .show(ui, |ui| {
                    for token in self.tokens.iter() {
                        render_token_card(ui, token);
                    }
                });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                powered_by_egui_and_eframe(ui);
                ui.separator();
                powered_by_moralis_api(ui);
            });
            ui.add_space(5.0);
        });
    }
}

fn render_token_card(ui: &mut egui::Ui, token: &Token) {
    ui.add_space(20.0);
    ui.horizontal(|ui| {
        ui.horizontal(|ui| {
            ui.heading(if token.name.is_empty() {
                &token.symbol
            } else {
                if token.symbol.is_empty() {
                    "Unknown token"
                } else {
                    &token.symbol
                }
            });
        });
    });
    ui.add_space(10.0);
    ui.horizontal(|ui| {
        ui.label("Amount: ");
        ui.label(&token.amount).highlight();
    });
    ui.separator();
    ui.add_space(10.0);
}

fn fetch_wallet_balance(network: &Network, wallet_address: String, api_key: String) -> Vec<Token> {
    let api = ApiService::new(api_key);

    let resp = api.get_account_tokens(network, wallet_address);

    match resp {
        Ok(AccountTokensResponse { tokens }) => tokens,
        Err(e) => {
            log::error!("Failed to fetch wallet balance: {:?}", e);
            vec![]
        }
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

fn powered_by_moralis_api(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("API powered by ");
        ui.hyperlink_to("moralis", "https://moralis.io/");

        ui.label(".");
    });
}
