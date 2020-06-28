use reqwest::Response;

#[derive(Default)]
pub struct IEX {
    client: reqwest::Client,
    base_url: &'static str,
}

impl IEX {
    pub fn new(sandbox: bool) -> Self {
        let url = if sandbox {
            "https://cloud.iexapis.com"
        } else {
            "https://sandbox.iexapis.com"
        };

        return IEX {
            client: reqwest::Client::new(),
            base_url: url,
        };
    }
}
