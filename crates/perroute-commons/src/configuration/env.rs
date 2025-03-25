use strum::EnumString;
use tap::TapFallible;

const RUN_ENV: &str = "RUN_ENV";

#[derive(
    Debug, Default, PartialEq, Eq, EnumString, strum::Display, Clone, Copy,
)]
#[strum(ascii_case_insensitive)]
pub enum Environment {
    #[default]
    Development,
    Production,
    Sandbox,
}

impl Environment {
    pub fn which() -> Self {
        std::env::var(RUN_ENV)
            .tap_err(|e| {
                log::warn!("Failed to fetch [{}] from environment. Error: {e:?}", RUN_ENV);
            })
            .map_or_else(
                |_| Environment::default(),
                |v| {
                    v.parse()
                        .tap_err(|e| {
                            log::warn!("Failed to parse {} to Environment. Error: {e:?}", v);
                        })
                        .unwrap_or_default()
                },
            )
    }
}
