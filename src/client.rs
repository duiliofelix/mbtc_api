use hmac::{Hmac, Mac, NewMac};
use reqwest::header::HeaderValue;
use serde::ser::Serialize;
use sha2::Sha512;
use crate::entities::{APIResponse, ListOrderResponse};

static MBTC_COMPLTE_URL: &str = "https://www.mercadobitcoin.net/tapi/v3/";
static MBTC_ENDPOINT: &str = "/tapi/v3/";

type BoxedError = Box<dyn std::error::Error>;
type Hmac512 = Hmac::<Sha512>;

pub struct MbtcClient {
    tapi_id: String,
    tapi_secret: Vec<u8>,
    http_client: reqwest::Client,
}

impl MbtcClient {
    pub fn new(tapi_id: String, tapi_secret: String) -> MbtcClient {
        MbtcClient {
            tapi_id,
            tapi_secret: tapi_secret.into_bytes(),
            http_client: reqwest::Client::new(),
        }
    }

    fn get_hmac_header<T: Serialize + ?Sized>(&self, params: &T) -> HeaderValue {
        let mut mac_builder = Hmac512::new_varkey(&self.tapi_secret).expect("deu ruim1");
        let mut msg = String::from(MBTC_ENDPOINT);

        msg.push_str(&serde_qs::to_string(&params).expect("nooooo"));
        mac_builder.update(&msg.into_bytes());

        let mac = mac_builder.finalize().into_bytes();
        
        format!("{:x}", mac).parse().unwrap()
    }

    async fn call_api<T: Serialize + ?Sized>(&self, params: &T) -> Result<ListOrderResponse, BoxedError> {
        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert("TAPI-ID", self.tapi_id.parse().unwrap());
        headers.insert("TAPI-MAC", self.get_hmac_header(params));

        let response = self.http_client
           .post(MBTC_COMPLTE_URL) 
           .headers(headers)
           .form(&params)
           .send()
           .await?
           .json::<APIResponse>()
           .await?;

        Ok(response.response_data)
    }

    pub async fn query_mbtc_api(&self) -> Result<ListOrderResponse, BoxedError> {
        let params = [("tapi_method", "list_orders"), ("tapi_nonce", "1"), ("coin_pair", "BRLBTC")];

        let response = self.call_api(&params).await?;

        Ok(response)
    }
}
