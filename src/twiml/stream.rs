use super::{format_xml_string, Action, Method};

pub enum Track {
    Inbound,
    Outbound,
    Both,
}

pub struct StartStream {
    pub url: String,
    pub name: Option<String>,
    pub track: Track,
    pub status_callback: Option<String>,
    pub method: Method,
}

impl Action for StartStream {
    fn as_twiml(&self) -> String {
        let mut attrs = Vec::new();
        attrs.push(("url", self.url.as_str()));
        if let Some(name) = &self.name {
            attrs.push(("name", name));
        }

        let track = match self.track {
            Track::Inbound => "inbound_track",
            Track::Outbound => "outbound_track",
            Track::Both => "both_tracks",
        };

        attrs.push(("track", track));
        if let Some(callback) = &self.status_callback {
            attrs.push(("statusCallback", callback));

            let method = match self.method {
                Method::Get => "GET",
                Method::Post => "POST",
            };

            attrs.push(("statusCallbackMethod", method));
        }

        let stream = format_xml_string("Stream", &attrs, "");
        format_xml_string("Connect", &[], &stream)
    }
}

impl Default for StartStream {
    fn default() -> Self {
        Self {
            url: "".to_string(),
            name: None,
            track: Track::Inbound,
            status_callback: None,
            method: Method::Post,
        }
    }
}

pub struct ConnectStream {
    pub url: String,
    pub name: Option<String>,
    pub status_callback: Option<String>,
    pub method: Method,
}

impl Action for ConnectStream {
    fn as_twiml(&self) -> String {
        let mut attrs = Vec::new();
        attrs.push(("url", self.url.as_str()));
        if let Some(name) = &self.name {
            attrs.push(("name", name));
        }

        if let Some(callback) = &self.status_callback {
            attrs.push(("statusCallback", callback));

            let method = match self.method {
                Method::Get => "GET",
                Method::Post => "POST",
            };

            attrs.push(("statusCallbackMethod", method));
        }

        let stream = format_xml_string("Stream", &attrs, "");
        format_xml_string("Connect", &[], &stream)
    }
}

impl Default for ConnectStream {
    fn default() -> Self {
        Self {
            url: "".to_string(),
            name: None,
            status_callback: None,
            method: Method::Post,
        }
    }
}
