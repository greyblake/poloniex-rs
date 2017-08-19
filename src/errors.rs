error_chain! {
    links {
        //Reqwest(reqwest::Error, reqwest::ErrorKind);
        //Req(reqwest::Error, reqwest::Error::Kind) #[cfg(unix)];
    }

    foreign_links {
        Reqwest(::reqwest::Error);
        ParseFloat(::std::num::ParseFloatError);
    }
}
