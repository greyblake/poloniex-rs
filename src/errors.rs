error_chain! {
    foreign_links {
        Reqwest(::reqwest::Error);
        ParseFloat(::std::num::ParseFloatError);
        ParseJson(::serde_json::Error);
        Io(::std::io::Error);
    }
    errors {
        KeyMissing { description("key is missing") }
        SecretMissing { description("secret is missing") }
    }
}
