#[allow(unused)]
use ethers::providers::{Authorization, Http, Ipc, Provider, QuorumProvider, Ws};
use serde::Deserialize;
use url::Url;

/// RpcProvider supports deserialization from yaml file to construct ethers-rs provider type.
#[derive(Debug)]
pub enum RpcProvider {
    Http(Provider<Http>),
    HttpWithBasicAuth(Provider<Http>),
    HttpWithBearerAuth(Provider<Http>),
    Websocket(Provider<Ws>),
    WebsocketWithBasicAuth(Provider<Ws>),
    WebsocketWithBearerAuth(Provider<Ws>),
    Ipc(Ipc),
    // Quorum(QuorumProvider),
}

impl<'de> Deserialize<'de> for RpcProvider {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct RpcProviderHelper {
            #[serde(default)]
            pub provider_type: RpcProviderTypes,
            pub chain_rpc_url: String,
            #[serde(alias = "chain_rpc_username")]
            pub username: Option<String>,
            #[serde(alias = "chain_rpc_password")]
            pub password: Option<String>,
            #[serde(alias = "chain_rpc_bearer")]
            pub bearer: Option<String>,
        }

        let helper = RpcProviderHelper::deserialize(deserializer)?;
        match helper.provider_type {
            RpcProviderTypes::Http => {
                let provider = Provider::<Http>::try_from(helper.chain_rpc_url)
                    .map_err(serde::de::Error::custom)?;
                Ok(RpcProvider::Http(provider))
            }
            RpcProviderTypes::HttpWithBasicAuth => {
                let username = helper
                    .username
                    .ok_or("missing `username` field")
                    .map_err(serde::de::Error::custom)?;
                let password = helper
                    .password
                    .ok_or("missing `password` field")
                    .map_err(serde::de::Error::custom)?;

                let url = Url::parse(&helper.chain_rpc_url).map_err(serde::de::Error::custom)?;
                let provider = Http::new_with_auth(url, Authorization::basic(username, password))
                    .map_err(serde::de::Error::custom)?;
                Ok(RpcProvider::HttpWithBasicAuth(Provider::new(provider)))
            }
            RpcProviderTypes::HttpWithBearerAuth => {
                let bearer = helper
                    .bearer
                    .ok_or("missing `bearer field`")
                    .map_err(serde::de::Error::custom)?;
                let url = Url::parse(&helper.chain_rpc_url).map_err(serde::de::Error::custom)?;
                let provider = Http::new_with_auth(url, Authorization::bearer(bearer))
                    .map_err(serde::de::Error::custom)?;
                Ok(RpcProvider::HttpWithBearerAuth(Provider::new(provider)))
            }
            RpcProviderTypes::Websocket => {
                let async_conn = Provider::<Ws>::connect(helper.chain_rpc_url);
                let rt = tokio::runtime::Runtime::new().map_err(serde::de::Error::custom)?;
                let provider = rt.block_on(async_conn).map_err(serde::de::Error::custom)?;
                Ok(RpcProvider::Websocket(provider))
            }
            RpcProviderTypes::WebsocketWithBasicAuth => {
                let username = helper
                    .username
                    .ok_or("missing `username` field")
                    .map_err(serde::de::Error::custom)?;
                let password = helper
                    .password
                    .ok_or("missing `password` field")
                    .map_err(serde::de::Error::custom)?;

                let async_conn = Provider::<Ws>::connect_with_auth(
                    helper.chain_rpc_url,
                    Authorization::basic(username, password),
                );

                let rt = tokio::runtime::Runtime::new().map_err(serde::de::Error::custom)?;
                let provider = rt.block_on(async_conn).map_err(serde::de::Error::custom)?;
                Ok(RpcProvider::WebsocketWithBasicAuth(provider))
            }
            RpcProviderTypes::WebsocketWithBearerAuth => {
                let bearer = helper
                    .bearer
                    .ok_or("missing `bearer field`")
                    .map_err(serde::de::Error::custom)?;
                let async_conn = Provider::<Ws>::connect_with_auth(
                    helper.chain_rpc_url,
                    Authorization::bearer(bearer),
                );

                let rt = tokio::runtime::Runtime::new().map_err(serde::de::Error::custom)?;
                let provider = rt.block_on(async_conn).map_err(serde::de::Error::custom)?;
                Ok(RpcProvider::WebsocketWithBearerAuth(provider))
            }
            RpcProviderTypes::Ipc => {
                let async_conn = Ipc::connect(helper.chain_rpc_url);

                let rt = tokio::runtime::Runtime::new().map_err(serde::de::Error::custom)?;
                let provider = rt.block_on(async_conn).map_err(serde::de::Error::custom)?;
                Ok(RpcProvider::Ipc(provider))
            } // RpcProviderTypes::Quorum => {
              //     struct ChainRpcHelper {
              //         pub providers: Vec<
              //     }
              // }
        }
    }
}

#[derive(Debug, Deserialize, strum::Display)]
pub enum RpcProviderTypes {
    Http,
    HttpWithBasicAuth,
    HttpWithBearerAuth,
    Websocket,
    WebsocketWithBasicAuth,
    WebsocketWithBearerAuth,
    Ipc,
    // Quorum,
}

impl Default for RpcProviderTypes {
    fn default() -> Self {
        RpcProviderTypes::Http
    }
}

#[cfg(test)]
mod tests {
    use claims::assert_matches;
    use ethers::utils::Anvil;

    use super::*;

    #[test]
    fn can_parse_default_rpc_provider() {
        let yaml = r#"
            chain_rpc_url: "https://eth.llamarpc.com"
        "#;

        let rpc_provider: RpcProvider = serde_yaml::from_str(yaml).unwrap();
        assert_matches!(rpc_provider, RpcProvider::Http(_));
    }

    #[test]
    fn can_parse_http_rpc_provider() {
        let yaml = r#"
            chain_rpc_url: "https://eth.llamarpc.com"
            provider_type: Http
        "#;

        let rpc_provider: RpcProvider = serde_yaml::from_str(yaml).unwrap();
        assert_matches!(rpc_provider, RpcProvider::Http(_));

        let yaml = r#"
            chain_rpc_url: "https://eth.llamarpc.com"
            provider_type: HttpWithBasicAuth
            username: test
            password: test
        "#;

        let rpc_provider: RpcProvider = serde_yaml::from_str(yaml).unwrap();
        assert_matches!(rpc_provider, RpcProvider::HttpWithBasicAuth(_));

        let yaml = r#"
            chain_rpc_url: "https://eth.llamarpc.com"
            provider_type: HttpWithBearerAuth
            bearer: thisisabearertoken
        "#;

        let rpc_provider: RpcProvider = serde_yaml::from_str(yaml).unwrap();
        assert_matches!(rpc_provider, RpcProvider::HttpWithBearerAuth(_));
    }

    #[test]
    fn can_parse_ws_rpc_provider() {
        let _anvil = Anvil::new().port(8545u16).spawn();

        let yaml = r#"
            chain_rpc_url: "ws://localhost:8545"
            provider_type: Websocket
        "#;

        let rpc_provider: RpcProvider = serde_yaml::from_str(yaml).unwrap();
        assert_matches!(rpc_provider, RpcProvider::Websocket(_));

        let yaml = r#"
            chain_rpc_url: "ws://localhost:8545"
            provider_type: WebsocketWithBasicAuth
            username: test
            password: test
        "#;

        let rpc_provider: RpcProvider = serde_yaml::from_str(yaml).unwrap();
        assert_matches!(rpc_provider, RpcProvider::WebsocketWithBasicAuth(_));

        let yaml = r#"
            chain_rpc_url: "ws://localhost:8545"
            provider_type: WebsocketWithBearerAuth
            bearer: thisisabearertoken
        "#;
        let rpc_provider: RpcProvider = serde_yaml::from_str(yaml).unwrap();
        assert_matches!(rpc_provider, RpcProvider::WebsocketWithBearerAuth(_));
    }

    // #[test]
    // fn can_parse_ipc_provider() {
    //     let yaml: &str = r#"
    //         chain_rpc_url: "~/.ethereum/geth.ipc"
    //         provider_type: Ipc
    //     "#;

    //     let rpc_provider: RpcProvider = serde_yaml::from_str(yaml).unwrap();
    //     assert_matches!(rpc_provider, RpcProvider::Ipc(_));
    // }

    // #[test]
    // fn can_parse_quorum_rpc_provider() {
    //     let yaml: &str = r#"
    //         provider_type: Quorum
    //         providers:
    //         - chain_rpc_url: "https://eth.llamarpc.com"
    //         - chain_rpc_url: "http://localhost:8545"
    //           provider_type: Http
    //           weight: 2
    //         - chain_rpc_url: "ws://localhost::8545"
    //           provider_type: Websocket
    //           weight: 3
    //     "#;

    //     let rpc_provider: RpcProvider = serde_yaml::from_str(yaml).unwrap();
    // }
}
