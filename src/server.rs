use std::{collections::HashMap, future::Future, net::SocketAddr, pin::Pin, sync::Arc};

use tokio::{io::AsyncReadExt, net::TcpListener};

use crate::{
    middleware::Middleware,
    request::Request,
    response::Response,
    util::{HttpError, HttpMethod},
};

pub type FutureResponse<'a> =
    Pin<Box<dyn Future<Output = Result<Response, HttpError>> + Send + 'a>>;
pub type RouteHandler = fn(Request) -> FutureResponse<'static>;

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct Route {
    method: HttpMethod,
    path: String,
}

/// ServerBuild struct . Used to build the server
pub struct ServerBuild {
    routes: Option<HashMap<Route, RouteHandler>>,
    address: Option<SocketAddr>,
    middleware: Option<Arc<Vec<Box<dyn Middleware>>>>,
}

impl ServerBuild {
    pub fn new() -> Self {
        Self {
            routes: Some(HashMap::new()),
            address: None,
            middleware: Some(Arc::new(Vec::new())),
        }
    }
    pub fn bind(mut self, socket: SocketAddr) -> Self {
        self.address = Some(socket);
        self
    }

    pub fn route(mut self, path: &str, method: HttpMethod, handler: RouteHandler) -> Self {
        if let Some(routes) = self.routes.as_mut() {
            routes.insert(
                Route {
                    method,
                    path: String::from(path),
                },
                handler,
            );
        } else {
            let mut routes = HashMap::new();
            routes.insert(
                Route {
                    method,
                    path: String::from(path),
                },
                handler,
            );
            self.routes = Some(routes)
        }
        self
    }
    //TODO:: accept  middleware

    pub fn build(self) -> Result<Server, String> {
        let address = self.address.ok_or("Address not set")?;
        let routes = self.routes.ok_or("Routes Uinitlalize")?;
        let middleware = self.middleware.ok_or("Middleware Error")?;
        Ok(Server {
            routes,
            address,
            middleware,
        })
    }
}

/// Server
pub struct Server {
    routes: HashMap<Route, RouteHandler>,
    address: SocketAddr,
    middleware: Arc<Vec<Box<dyn Middleware>>>,
}
impl Server {
    pub async fn run(&self) -> std::io::Result<()> {
        let addr = self.address;
        let listener = TcpListener::bind(addr).await?;
        println!("Serever listening on {:?}", addr.to_string());
        loop {
            let (mut stream, _) = listener.accept().await?;
            tokio::spawn(async move {
                let mut buffer = [0; 1024];
                let _ = stream.read(&mut buffer).await.unwrap();
            })
        }
        todo!()
    }
}
