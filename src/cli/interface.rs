use super::utils::naive_join_slash;

pub trait Module {
    fn execute(&self, base: &Option<String>);
}

pub trait PanelBaseUrl {
    const BASE_URL: &'static str;

    fn panel_join(&self, custom: &Option<String>, to: &str) -> String {
        naive_join_slash(
            custom
                .as_ref()
                .map(String::as_str)
                .unwrap_or(Self::BASE_URL),
            to,
        )
    }
}
