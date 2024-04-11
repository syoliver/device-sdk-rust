use openapi;
use std::sync::Arc;
use crate::service::Service;
use axum_extra::extract::{CookieJar};
use http::Method;
use axum;
use axum::extract::Host;
use tokio;

pub struct RestServer {
    service: Arc<Service>
}


async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

impl RestServer {
    pub fn new(service: Arc<Service>) -> Self {
        Self {
            service: service
        }
    }

    pub async fn run(self: Arc<Self>, address: &str) {
        // Run the server with graceful shutdown
        let listener = tokio::net::TcpListener::bind(address).await.unwrap();
        let router = openapi::server::new(self);
        axum::serve(listener, router)
            .with_graceful_shutdown(shutdown_signal())
            .await
            .unwrap();
    }
}

impl openapi::Api for RestServer {
    /// Returns the current configuration of the service..
    ///
    /// ConfigGet - GET /api/v3/config
    async fn config_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        ) -> Result<openapi::ConfigGetResponse, String> {
        Err("not implemented")
    }


        /// DeviceNameNameCommandGet - GET /api/v3/device/name/{name}/{command}
        async fn device_name_name_command_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
            header_params: openapi::models::DeviceNameNameCommandGetHeaderParams,
            path_params: openapi::models::DeviceNameNameCommandGetPathParams,
            query_params: openapi::models::DeviceNameNameCommandGetQueryParams,
        ) -> Result<openapi::DeviceNameNameCommandGetResponse, String> {
            Err("not implemented")
        }


        /// DeviceNameNameCommandPut - PUT /api/v3/device/name/{name}/{command}
        async fn device_name_name_command_put(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
            header_params: openapi::models::DeviceNameNameCommandPutHeaderParams,
            path_params: openapi::models::DeviceNameNameCommandPutPathParams,
                body: openapi::models::SettingRequest,
        ) -> Result<openapi::DeviceNameNameCommandPutResponse, String> {
            Err("not implemented")
        }


        /// DiscoveryPost - POST /api/v3/discovery
        async fn discovery_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        ) -> Result<openapi::DiscoveryPostResponse, String> {
            Err("not implemented")
        }


        /// A simple 'ping' endpoint that can be used as a service healthcheck.
        ///
        /// PingGet - GET /api/v3/ping
        async fn ping_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        ) -> Result<openapi::PingGetResponse, String> {
            Err("not implemented")
        }


        /// Stores a secret to the secure Secret Store.
        ///
        /// SecretPost - POST /api/v3/secret
        async fn secret_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        header_params: openapi::models::SecretPostHeaderParams,
        body: openapi::models::SecretRequest,
        ) -> Result<openapi::SecretPostResponse, String> {
            Err("not implemented")
        }


        /// A simple 'version' endpoint that will return the current version of the service.
        ///
        /// VersionGet - GET /api/v3/version
        async fn version_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        ) -> Result<openapi::VersionGetResponse, String> {
            Err("not implemented")
        }
    
}