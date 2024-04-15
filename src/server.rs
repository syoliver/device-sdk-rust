#![allow(unused)]

use openapi;
use std::sync::{Arc, Weak};
use crate::service::Service;
use axum_extra::extract::{CookieJar};
use http::Method;
use axum;
use axum::extract::Host;
use tokio;
use async_trait::async_trait;
use std::ops::Drop;
use log;
use tokio::sync::Notify;

#[derive(Clone)]
pub struct RestServer {
    service: Weak<Service>
}

impl RestServer {
    pub fn new(service: Arc<Service>) -> Arc<Self> {
        Arc::new(Self {
            service: Arc::downgrade(&service)
        })
    }

    pub fn run(self: Arc<Self>, address: String, notify_stop: Arc<Notify>) {

        let spawn_notify_stop = notify_stop.clone();
        tokio::spawn(async move {
            // Run the server with graceful shutdown
            let listener = tokio::net::TcpListener::bind(address).await.unwrap();
            let router = openapi::server::new(self);

            axum::serve(listener, router)
                .with_graceful_shutdown(async move { spawn_notify_stop.notified().await; })
                .await
                .unwrap();
        });
    }
}

unsafe impl Send for RestServer {}
unsafe impl Sync for RestServer {}

impl Drop for RestServer {
    fn drop(&mut self) {
        // TODO: kill the running task
    }
}
#[async_trait]
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
        Err("not implemented".to_string())
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
            Err("not implemented".to_string())
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
            Err("not implemented".to_string())
        }


        /// DiscoveryPost - POST /api/v3/discovery
        async fn discovery_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        ) -> Result<openapi::DiscoveryPostResponse, String> {
            Err("not implemented".to_string())
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
            Err("not implemented".to_string())
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
            Err("not implemented".to_string())
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
            Err("not implemented".to_string())
        }
    
}