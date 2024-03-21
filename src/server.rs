use std::{collections::HashMap, future::Future, net::SocketAddr, pin::Pin, sync::Arc};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

use crate::{
    middleware::Middleware,
    request::Request,
    response::Response,
    util::{HttpError, HttpMethod, HttpStatusCode},
};

pub type FutureResponse<'a> =
    Pin<Box<dyn Future<Output = Result<Response, HttpError>> + Send + 'a>>;
pub type RouteHandler = fn(Request) -> FutureResponse<'static>;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
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

    ///add route
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

    /// build  a server
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

impl Default for ServerBuild {
    fn default() -> Self {
        Self::new()
    }
}

/// Server
#[derive(Clone)]
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
            let routes = self.routes.clone();

            tokio::spawn(async move {
                let mut buffer = [0; 1024];
                //TODO::Handling tcp sticky packets
                let _ = stream.read(&mut buffer).await.unwrap();
                let request = Request::parse(&buffer).unwrap();
                let future_response = handle_route(request, &routes);
                if let Ok(response) = future_response.await {
                    let response_string = format!(
                        "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}",
                        response.status_code,
                        response.status_text,
                        response
                            .headers
                            .iter()
                            .map(|(k, v)| format!("{k}: {v}"))
                            .collect::<Vec<_>>()
                            .join("\r\n"),
                        response.body.unwrap_or_default()
                    );
                    stream.write_all(response_string.as_bytes()).await.unwrap();
                    stream.flush().await.unwrap();
                }
            });
        }
    }
}
fn handle_route<'a>(
    request: Request,
    routes: &'a HashMap<Route, RouteHandler>,
    // _middleware: &'a Vec<Box<dyn Middleware>>,
) -> FutureResponse<'a> {
    if let Some(handler) = routes.get(&Route {
        method: request.method.clone(),
        path: request.uri.clone(),
    }) {
        //TODO::: exucte middleware
        handler(request)
    } else {
        Box::pin(async move {
            Err(HttpError::InternalServerError(
                HttpStatusCode::InternalServerError,
                "Internal Server Error",
            ))
        })
    }
}
