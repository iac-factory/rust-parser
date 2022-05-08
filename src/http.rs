use std::str::FromStr;

use hyper::http::uri::InvalidUri;
use hyper::Uri;

// --> Standard Library
pub use std::error::Error;

// --> Asynchronous Runtime
pub use tokio::main as Asynchronous;
pub use tokio::fs::File as File;
pub use tokio::io::{AsyncWriteExt as Writer, AsyncReadExt as Reader};

pub use hyper::client::connect::{dns as DNS};
pub use hyper::client::{HttpConnector as HTTP};
pub use hyper::body::{Body as Data, HttpBody as Body};
pub use hyper::{Client as Client, Response, Request};
pub use hyper::http::request::{Builder};
pub use hyper::{Uri as URL};

// --> TLS Compliance
pub use hyper_tls::{HttpsConnector as TLS};
pub use std::borrow::{Borrow, BorrowMut};
pub use std::ops::{Deref};
pub use futures::future::err;
use hyper::client::ResponseFuture;

//  --> Type Definitions
pub type Resolver = DNS::GaiResolver;
pub type Content = hyper::body::Body;
pub type Result<Type, Heap> = std::result::Result<Type, Heap>;
pub type Awaitable<Generic, Allocation> = Result<Generic, Allocation>;
pub type Heap = Box<dyn Error + Send + Sync>;
pub type Future<Generic> = Awaitable<Generic, Heap>;

// --> Utility Type + Implementation

pub struct HTTPs {
    Connection: *const TLS<HTTP>,
    pub(crate) Location: String,
    pub(crate) Client: Client<TLS<HTTP>>,
    pub(crate) Response: Response<hyper::Body>
}

impl HTTPs {
    pub(crate) fn Construct () -> HTTPs {
        return HTTPs {
            Connection: &TLS::new(),
            Location: String::new(),
            Client: HTTPs::Build(),
            Response: Response::new(
                hyper::Body::empty()
            )
        };
    }

    /// --> Private
    fn Build () -> Client<TLS<HTTP<Resolver>>, Data> {
        return Client::builder().build::<(_), Data>(TLS::new());
    }

    pub fn url(uri: &str) -> Result<Uri, InvalidUri> {
        return URL::from_str(&uri);
    }
}
