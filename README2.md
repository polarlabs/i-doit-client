If you're building a Rust client for i-doit's JSON-RPC API, I'd aim for an API that hides JSON-RPC mechanics completely while still exposing i-doit's domain concepts naturally.

I'd roughly target this:

let client = IdoitClient::builder()
.endpoint("https://idoit-demo.keepbit.net/src/jsonrpc.php")
.api_key(api_key)
.build()?;

let session = client.login(username, password).await?;

let objects = session
.objects()
.list()
.await?;

for object in objects {
println!("{}: {}", object.id, object.title);
}


But there are some design decisions I'd make carefully.

1. Separate configuration, authentication, and API calls

I'd use the builder for local configuration, not login:

let client = IdoitClient::builder()
.endpoint("https://idoit-demo.keepbit.net/src/jsonrpc.php")
.api_key(api_key)
.timeout(Duration::from_secs(30))
.build()?;

let client = client.login(username, password).await?;


The types could be:

IdoitClient<Unauthenticated>


after build(), and:

IdoitClient<Authenticated>


after login().

Then:

impl IdoitClient<Authenticated> {
pub async fn objects(&self) -> Result<Objects<'_>, Error> {
// ...
}
}


This gives you a useful guarantee without making every runtime authentication condition part of the type system.

2. Hide JSON-RPC from the user

I would not make the normal API look like this:

client
.call("cmdb.objects.read", json!({
"id": 22
}))
.await?;


That's basically exposing the wire protocol.

Instead:

let object = client.objects().get(22).await?;


Internally, you can have:

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


Your public API then becomes domain-oriented.

3. I'd use endpoint/domain objects

For example:

let object = client.objects().get(22).await?;

let objects = client
.objects()
.list()
.await?;

let categories = client
.categories()
.list()
.await?;

let object = client
.objects()
.get(22)
.await?;


This gives you a nice structure:

IdoitClient
│
├── objects()
│   ├── get()
│   ├── list()
│   ├── create()
│   ├── update()
│   └── delete()
│
├── categories()
│   └── ...
│
├── objects_types()
│   └── ...
│
└── ...


You don't necessarily need all of these to be actual nested structs. They're an ergonomic API boundary.

4. Don't expose Url everywhere

Since you've already chosen url::Url, I'd keep it internally:

pub struct IdoitClient<State> {
endpoint: Url,
// ...
}


Then let the builder accept strings ergonomically:

pub fn endpoint(mut self, endpoint: impl AsRef<str>) -> Result<Self, Error> {
self.endpoint = Some(Url::parse(endpoint.as_ref())?);
Ok(self)
}


So users get:

let client = IdoitClient::builder()
.endpoint("https://idoit.example.com/src/jsonrpc.php")?
.build()?;


rather than having to write:

.endpoint(Url::parse("...")?)


That's exactly the kind of abstraction I'd put in a client library.

5. I'd probably use typestate only for authentication

Something like:

pub struct IdoitClient<State> {
endpoint: Url,
http: reqwest::Client,
state: State,
}

pub struct Unauthenticated;

pub struct Authenticated {
session_id: String,
}


Then:

impl IdoitClient<Unauthenticated> {
pub async fn login(
self,
username: &str,
password: &str,
) -> Result<IdoitClient<Authenticated>, Error> {
// authenticate
}
}


And authenticated operations:

impl IdoitClient<Authenticated> {
pub fn objects(&self) -> Objects<'_> {
Objects { client: self }
}
}


This is a good use of typestate because it gives you a meaningful API distinction:

client.objects()


isn't even available until authentication has happened.

I wouldn't create types like:

AuthenticatedWithValidToken
AuthenticatedWithExpiredToken
Refreshing
AuthenticationFailed


Those are runtime conditions, not useful API states.

6. Keep a low-level escape hatch

This is important for a library around an API as large/irregular as i-doit.

You will inevitably encounter an API endpoint that you haven't wrapped yet.

I'd therefore provide something like:

client
.rpc("some.idoit.method", params)
.await?;


while making the typed API the preferred path.

So users get both:

// High-level API
let object = client.objects().get(22).await?;


and:

// Escape hatch
let response: MyResponse = client
.rpc("some.idoit.method", params)
.await?;


That's a very valuable property for an SDK.

7. Model JSON-RPC errors separately from HTTP errors

I'd have something along these lines:

#[derive(Debug, thiserror::Error)]
pub enum Error {
#[error("HTTP request failed")]
Http(#[from] reqwest::Error),

    #[error("invalid URL")]
    InvalidUrl(#[from] url::ParseError),

    #[error("JSON serialization failed")]
    Serialization(#[from] serde_json::Error),

    #[error("i-doit returned an error: {0}")]
    Rpc(RpcError),

    #[error("authentication failed")]
    Authentication(AuthenticationError),
}


That's important because:

HTTP failure


and:

HTTP 200 + JSON-RPC error


are completely different things.

Your user should be able to distinguish them.

8. Make requests typed

Internally, JSON-RPC might look like:

#[derive(Serialize)]
struct JsonRpcRequest<P> {
jsonrpc: &'static str,
method: String,
params: P,
id: u64,
}


But the user should never need to construct that.

For example:

#[derive(Serialize)]
struct ObjectReadParams {
id: u64,
}


Then:

pub async fn get(&self, id: u64) -> Result<Object, Error> {
self.client
.call(
"cmdb.objects.read",
ObjectReadParams { id },
)
.await
}


The public API is therefore:

client.objects().get(22).await?;


while the implementation knows:

cmdb.objects.read

9. Consider a request builder for complicated endpoints

Some i-doit operations have many parameters.

Rather than:

client.objects().list(
Some(10),
Some(100),
Some("server"),
None,
...
)


I'd use another Builder:

let objects = client
.objects()
.list()
.limit(100)
.offset(10)
.object_type("C__OBJTYPE__SERVER")
.await?;


Or:

let objects = client
.objects()
.list(
ObjectListParams::builder()
.limit(100)
.offset(10)
.object_type("C__OBJTYPE__SERVER")
.build()?
)
.await?;


I'd prefer the first style if the parameters aren't reused.

10. I'd distinguish IDs from arbitrary integers

This is optional, but I think it's worthwhile in a domain-heavy client.

Instead of:

fn get(&self, id: u64)


you could eventually have:

pub struct ObjectId(u64);


Then:

client.objects().get(ObjectId::new(22)).await?;


This becomes especially valuable if i-doit has several kinds of IDs.

You don't want to accidentally do:

client.objects().get(category_id)


and have everything compile because they're all u64.

You don't necessarily need to introduce these types immediately, though. I'd start with u64 and introduce domain IDs where confusion actually arises.

The API I'd aim for

Putting it all together:

let client = IdoitClient::builder()
.endpoint("https://idoit-demo.keepbit.net/src/jsonrpc.php")?
.api_key(api_key)
.timeout(Duration::from_secs(30))
.build()?;

let client = client
.login("admin", password)
.await?;

let object = client
.objects()
.get(22)
.await?;

println!("{}: {}", object.id, object.title);


And more complex operations:

let objects = client
.objects()
.list()
.limit(100)
.offset(0)
.await?;


With the escape hatch:

let result: MyType = client
.rpc("some.idoit.method", params)
.await?;


Internally:

                  ┌─────────────────────┐
                  │    IdoitClient       │
                  │                     │
                  │ reqwest::Client     │
                  │ Url                 │
                  │ authentication      │
                  └──────────┬──────────┘
                             │
              ┌──────────────┼──────────────┐
              │              │              │
              ▼              ▼              ▼
          Objects        Categories       RPC
              │              │              │
              ▼              ▼              ▼
        typed params    typed params    raw params
              │              │              │
              └──────────────┼──────────────┘
                             ▼
                       JSON-RPC layer
                             │
                             ▼
                          i-doit


My main design principle would be: make the library feel like a Rust API for i-doit, not like a Rust wrapper around JSON-RPC.

And given what you've been asking about—Builder + Url + typestate—I think you're on a good track. I'd use Builder for constructing/configuring the client, typestate for the explicit authentication boundary, and ordinary runtime state for token/session validity.