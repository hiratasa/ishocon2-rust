use std::env;

use newrelic::{App, Transaction};

const APP_NAME: &str = "ishocon2-rust-hiratasa";

pub struct NewRelicAppData {
    app: Option<App>,
}

impl NewRelicAppData {
    fn new() -> NewRelicAppData {
        NewRelicAppData { app: None }
    }

    fn new_with_key(key: &str) -> NewRelicAppData {
        let app = App::new(APP_NAME, key).expect("Could not create app.");
        NewRelicAppData { app: Some(app) }
    }

    pub fn transaction(&self, name: &str) -> Option<Transaction> {
        self.app.as_ref().map(|app| {
            app.web_transaction(name)
                .expect("Could not start transaction")
        })
    }
}

pub fn create_app() -> NewRelicAppData {
    let license_key = match env::var("NEW_RELIC_LICENSE_KEY") {
        Ok(key) => key,
        Err(e) => {
            eprintln!("{}", e);
            return NewRelicAppData::new();
        }
    };
    NewRelicAppData::new_with_key(&license_key)
}

pub mod actix_web {
    use actix_service::*;
    use actix_web::dev::*;
    use actix_web::Error;
    use core::future::Future;
    use futures::FutureExt;

    use super::NewRelicAppData;

    // Use with App::wrap_fn
    pub fn wrap_log_transaction<S, Res>(
        req: ServiceRequest,
        srv: &mut S,
    ) -> impl Future<Output = Result<Res, Error>>
    where
        S: Service<Request = ServiceRequest, Response = Res, Error = Error>,
    {
        let newrelic: &NewRelicAppData = &req.app_data().unwrap();

        let transaction = newrelic.transaction(&(req.method().to_string() + req.path()));

        srv.call(req).map(|res| {
            // take ownership
            let _transaction = transaction;
            res
        })
    }
}
