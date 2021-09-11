/// https://fetch.spec.whatwg.org/#request-class
#[allow(dead_code)]
#[derive(Clone)]
struct Request {
    method: String,  // TODO
    url: String,     // TODO
    headers: String, // TODO
    destination: RequestDestination,
    referrer: String,
    referrer_policy: ReferrerPolicy,
    mode: RequestMode,
    credentials: RequestCredentials,
    cache: RequestCache,
    redirect: RequestRedirect,
    integrity: String,
    is_reload_navigation: bool,
    is_history_navigation: bool,
    abort_signal: AbortSignal, // TODO
}

/// https://fetch.spec.whatwg.org/#requestdestination
#[allow(dead_code)]
#[derive(Clone)]
enum RequestDestination {
    NonCategorized, // corresponds to ""
    Audio,
    AudioWorklet,
    Document,
    Embed,
    Font,
    Frame,
    IFrame,
    Image,
    Manifest,
    Object,
    PaintWorklet,
    Report,
    Script,
    SharedWorker,
    Style,
    Track,
    Video,
    Worker,
    Xslt,
}

/// https://w3c.github.io/webappsec-referrer-policy/#enumdef-referrerpolicy
#[allow(dead_code)]
#[derive(Clone)]
enum ReferrerPolicy {
    NonCategorized, // corresponds to ""
    NoReferrer,
    NoReferrerWhenDowngrade,
    SameOrigin,
    Origin,
    StrictOrigin,
    OriginWhenCrossOrigin,
    StrictOriginWhenCrossOrigin,
    UnsafeUrl,
}

/// https://fetch.spec.whatwg.org/#requestmode
#[allow(dead_code)]
#[derive(Clone)]
enum RequestMode {
    Navigate,
    SameOrigin,
    NoCors,
    Cors,
}

/// https://fetch.spec.whatwg.org/#requestcredentials
#[allow(dead_code)]
#[derive(Clone)]
enum RequestCredentials {
    Omit,
    SameOrigin,
    Include,
}

/// https://fetch.spec.whatwg.org/#requestcache
#[allow(dead_code)]
#[derive(Clone)]
enum RequestCache {
    Default,
    NoStore,
    Reload,
    NoCache,
    ForceCache,
    OnlyIfCached,
}

/// https://fetch.spec.whatwg.org/#requestredirect
#[allow(dead_code)]
#[derive(Clone)]
enum RequestRedirect {
    Follow,
    Error,
    Manual,
}

#[allow(dead_code)]
#[derive(Clone)]
struct AbortSignal {
    aborted: bool,
}

#[allow(dead_code)]
impl AbortSignal {
    pub fn abort() -> AbortSignal {
        AbortSignal { aborted: true }
    }
}
