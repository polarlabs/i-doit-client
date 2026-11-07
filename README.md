Yes. For a Rust crate wrapping i-doit's JSON-RPC API, I’d design it as a typed, async client with a small JSON-RPC core and resource-specific APIs on top.

The important design choice is: don’t expose i-doit’s JSON-RPC wire format directly as your crate’s public API. Hide JSON-RPC details behind ergonomic Rust types.

A good architecture would look roughly like this:

idoit-client
│
├── Client
│   ├── authentication
│   ├── request/response handling
│   ├── error handling
│   └── transport
│
├── api/
│   ├── objects
│   ├── categories
│   ├── cmdb
│   ├── users
│   └── ...
│
├── rpc/
│   ├── request
│   ├── response
│   └── error
│
└── models/
├── Object
├── Category
├── ...

1. The public API

I'd aim for something like:

use idoit::Client;

#[tokio::main]
async fn main() -> Result<(), idoit::Error> {
let client = Client::builder()
.url("https://idoit.example.com/api")
.api_key("...")
.build()?;

    let object = client
        .objects()
        .get(123)
        .await?;

    println!("Object: {}", object.title);

    Ok(())
}


For operations:

let objects = client
.objects()
.list()
.filter("C__OBJTYPE", 5)
.execute()
.await?;


Or, if i-doit's API maps nicely to conventional CRUD:

let object = client.objects().get(123).await?;

let object = client
.objects()
.create(CreateObject {
title: "My Server".into(),
object_type: 5,
})
.await?;

client
.objects()
.update(123, UpdateObject {
title: Some("New Name".into()),
..Default::default()
})
.await?;

client.objects().delete(123).await?;


The exact API can be adjusted once you've catalogued the i-doit methods.

2. Separate JSON-RPC from i-doit

This is probably the most important architectural decision.

Have an internal generic JSON-RPC layer:

#[derive(Debug, Serialize)]
struct RpcRequest<P> {
jsonrpc: &'static str,
method: String,
params: P,
id: u64,
}

#[derive(Debug, Deserialize)]
struct RpcResponse<T> {
jsonrpc: String,
result: Option<T>,
error: Option<RpcError>,
id: u64,
}


Then a lower-level client:

struct RpcClient {
http: reqwest::Client,
endpoint: Url,
api_key: String,
}


with something like:

impl RpcClient {
async fn call<P, R>(
&self,
method: &str,
params: P,
) -> Result<R, Error>
where
P: Serialize,
R: DeserializeOwned,
{
// JSON-RPC implementation
}
}


Your i-doit layer then becomes:

pub struct ObjectApi<'a> {
rpc: &'a RpcClient,
}

impl ObjectApi<'_> {
pub async fn get(&self, id: ObjectId) -> Result<Object, Error> {
self.rpc
.call("cmdb.objects.read", json!({
"id": id
}))
.await
}
}


This gives you a very clean separation:

ObjectApi
↓
RpcClient
↓
reqwest
↓
HTTP


If i-doit changes its transport/authentication mechanism later, you don't have to redesign the entire crate.

3. Don't make serde_json::Value your primary API

It's tempting to write:

pub async fn call(
&self,
method: &str,
params: serde_json::Value,
) -> Result<serde_json::Value, Error>


and expose that everywhere.

I'd definitely provide it, but make it the escape hatch rather than the main interface.

For example:

client.rpc().call("some.idoit.method", params).await?;


can be useful for i-doit methods you haven't modeled yet.

But users should normally get:

client.objects().get(id).await?


rather than:

client.call(
"cmdb.objects.read",
json!({ "id": 123 }),
).await?;


This is especially valuable because i-doit's API has a lot of method-specific parameter and result structures.

4. Use strong types for IDs

Instead of making everything an i64:

pub type ObjectId = i64;
pub type CategoryId = i64;


I'd consider newtypes:

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ObjectId(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CategoryId(pub i64);


Then:

pub async fn get(&self, id: ObjectId) -> Result<Object, Error>


rather than:

pub async fn get(&self, id: i64) -> Result<Object, Error>


This prevents accidentally passing a category ID where an object ID is expected.

5. Model the API around domains

I wouldn't create one enormous Client implementation containing hundreds of methods.

Instead:

pub struct Client {
rpc: RpcClient,
}

impl Client {
pub fn objects(&self) -> ObjectApi<'_> {
ObjectApi { rpc: &self.rpc }
}

    pub fn categories(&self) -> CategoryApi<'_> {
        CategoryApi { rpc: &self.rpc }
    }

    pub fn users(&self) -> UserApi<'_> {
        UserApi { rpc: &self.rpc }
    }
}


Then:

client.objects()
client.categories()
client.users()
client.organizations()
client.relations()
client.reports()
...


This makes IDE autocomplete much more useful.

6. Make request types explicit

For anything non-trivial, don't expose a giant HashMap.

For example:

#[derive(Debug, Serialize)]
pub struct CreateObject {
pub title: String,
pub object_type: ObjectTypeId,
pub purpose: Option<String>,
}


And:

#[derive(Debug, Serialize, Default)]
pub struct UpdateObject {
#[serde(skip_serializing_if = "Option::is_none")]
pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
}


Then:

client.objects()
.create(CreateObject {
title: "web01".into(),
object_type: ObjectTypeId(5),
purpose: Some("Web server".into()),
})
.await?;


This gives compile-time documentation and makes the crate pleasant to use.

7. But i-doit has dynamic data — embrace that

This is where I'd avoid over-typing the entire API.

Some i-doit concepts are inherently dynamic, particularly category-specific data.

I'd use a hybrid model.

For example:

pub struct Object {
pub id: ObjectId,
pub title: String,
pub object_type: ObjectType,
// ...
}


but category data could be:

pub struct CategoryData {
pub category: String,
pub data: serde_json::Value,
}


Then provide typed helpers where worthwhile:

let network = object.category::<NetworkInterface>().await?;


Potentially:

let network: NetworkInterface =
client.categories()
.get(object.id, NetworkInterface::NAME)
.await?;


This lets you gradually add strongly typed representations without making the whole crate unusably complicated.

8. Consider a generic category abstraction

If i-doit has many categories with similar API semantics, you can exploit Rust's traits.

Something along these lines:

pub trait Category {
const NAME: &'static str;

    type Data: Serialize + DeserializeOwned;
}


Then:

pub struct NetworkInterface;

impl Category for NetworkInterface {
const NAME: &'static str = "C__NETWORK_PORT";
type Data = NetworkInterfaceData;
}


Your client could expose:

pub async fn get_category<C>(
&self,
object: ObjectId,
) -> Result<C::Data, Error>
where
C: Category,
{
// ...
}


Usage:

let interfaces: NetworkInterfaceData =
client.get_category::<NetworkInterface>(object_id).await?;


That's a very Rust-y API and could become one of the crate's nicest features.

9. Design errors carefully

Don't just return:

Result<T, reqwest::Error>


because there are several failure layers.

I'd have something like:

#[derive(Debug, thiserror::Error)]
pub enum Error {
#[error("HTTP request failed")]
Http(#[from] reqwest::Error),

    #[error("failed to serialize request")]
    Serialization(#[from] serde_json::Error),

    #[error("i-doit API error: {0}")]
    Api(#[from] ApiError),

    #[error("unexpected API response: {0}")]
    InvalidResponse(String),
}


And represent JSON-RPC errors separately:

#[derive(Debug, Deserialize)]
pub struct ApiError {
pub code: i64,
pub message: String,
pub data: Option<serde_json::Value>,
}


Ideally users can inspect:

match client.objects().get(id).await {
Ok(object) => println!("{object:?}"),

    Err(Error::Api(err)) => {
        println!("i-doit error {}: {}", err.code, err.message);
    }

    Err(err) => {
        println!("transport error: {err}");
    }
}

10. Authentication should be builder-based

I'd avoid making users construct the client struct manually.

Something like:

let client = Client::builder()
.url("https://idoit.example.com/api")
.api_key("...")
.build()?;


Potentially:

let client = Client::builder()
.url(url)
.api_key(api_key)
.timeout(Duration::from_secs(30))
.build()?;


And possibly:

Client::builder()
.url(url)
.credentials(username, password)
.build()?;


depending on what authentication mechanisms i-doit supports.

11. Make the underlying HTTP client configurable

For a library, don't force one global reqwest::Client.

I'd support:

Client::builder()
.http_client(reqwest_client)
.build()?;


This allows applications to configure:

proxies
TLS
connection pools
timeouts
tracing
custom headers
middleware

without your crate having to duplicate those features.

12. Async by default

I'd make the primary API async:

pub async fn get(&self, id: ObjectId) -> Result<Object, Error>


using:

tokio
reqwest
serde
serde_json
thiserror


Potentially make the runtime choice mostly invisible by relying on reqwest's async API.

If you want the crate to work in both Tokio and other async environments, don't unnecessarily depend on Tokio-specific functionality outside what reqwest requires.

13. Expose the raw RPC escape hatch

This is important for a third-party API wrapper.

Something like:

impl Client {
pub fn rpc(&self) -> &RpcClient {
&self.rpc
}
}


Then:

let result: MyResponse = client
.rpc()
.call("some.future.idoit.method", params)
.await?;


This means your crate doesn't have to implement 100% of i-doit before it's useful.

It also provides a migration path when i-doit introduces new API methods.

14. I'd distinguish API version from crate version

If i-doit has API versions, don't encode it into every method:

client.objects_v1().get(...)


unless multiple versions genuinely need to coexist.

Prefer:

Client::builder()
.api_version(ApiVersion::V2)
...


or whatever matches the actual i-doit versioning scheme.

Internally:

Client
│
├── RpcClient
│
└── API implementation
├── objects
├── categories
└── ...


That gives you room to support multiple server versions.

15. A possible crate layout

I'd start with:

src/
├── lib.rs
├── client.rs
├── error.rs
│
├── rpc/
│   ├── mod.rs
│   ├── request.rs
│   ├── response.rs
│   └── error.rs
│
├── api/
│   ├── mod.rs
│   ├── objects.rs
│   ├── categories.rs
│   ├── users.rs
│   └── ...
│
└── model/
├── mod.rs
├── object.rs
├── category.rs
├── user.rs
└── ...


And the public surface in lib.rs should be deliberately small:

pub use client::{Client, ClientBuilder};
pub use error::Error;

pub use model::{
Object,
ObjectId,
};


Don't necessarily expose every internal RPC type.

16. The design I'd personally choose

I'd target this user experience:

let client = idoit::Client::builder()
.url("https://idoit.example/api")
.api_key("secret")
.build()?;

let server = client.objects().get(ObjectId(42)).await?;

println!("{}", server.title);

let interfaces = client
.categories()
.get::<NetworkInterfaces>(server.id)
.await?;

for interface in interfaces {
println!("{}: {}", interface.name, interface.ip);
}


while still allowing:

let result: serde_json::Value = client
.rpc()
.call(
"some.idoit.method",
serde_json::json!({
"foo": "bar"
}),
)
.await?;


So the core philosophy would be:

Typed and ergonomic for the common path, dynamic and low-level when necessary.

That will make the crate much more useful than a thin reqwest + serde_json wrapper, while avoiding the trap of trying to encode every oddity of i-doit's API into Rust's type system.

If you're starting this project now, I'd also recommend designing the first 3–5 i-doit methods together before committing to the public API. The shape of those methods will reveal whether you want a resource-oriented API (client.objects().get()) or a more direct RPC-oriented API (client.objects().read()).