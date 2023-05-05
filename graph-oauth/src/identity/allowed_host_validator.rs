use url::{Host, Url};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HostValidator {
    Valid,
    Invalid,
}

pub trait AllowedHostValidator<RHS = Self> {
    fn validate(&self, valid_hosts: &[Url]) -> HostValidator;
}

impl AllowedHostValidator for Url {
    fn validate(&self, valid_hosts: &[Url]) -> HostValidator {
        let size_before = valid_hosts.len();
        let hosts: Vec<Host<&str>> = valid_hosts.iter().flat_map(|url| url.host()).collect();
        assert_eq!(size_before, hosts.len());

        if let Some(host) = self.host() {
            if hosts.contains(&host) {
                return HostValidator::Valid;
            }
        }

        HostValidator::Invalid
    }
}

impl AllowedHostValidator for String {
    fn validate(&self, valid_hosts: &[Url]) -> HostValidator {
        if let Ok(url) = Url::parse(self) {
            return url.validate(valid_hosts);
        }

        HostValidator::Invalid
    }
}

impl AllowedHostValidator for &str {
    fn validate(&self, valid_hosts: &[Url]) -> HostValidator {
        if let Ok(url) = Url::parse(self) {
            return url.validate(valid_hosts);
        }

        HostValidator::Invalid
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_hosts() {
        let valid_hosts: Vec<String> = vec![
            "graph.microsoft.com",
            "graph.microsoft.us",
            "dod-graph.microsoft.us",
            "graph.microsoft.de",
            "microsoftgraph.chinacloudapi.cn",
            "canary.graph.microsoft.com",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        let host_urls: Vec<Url> = valid_hosts
            .iter()
            .map(|s| format!("https://{s}"))
            .flat_map(|s| Url::parse(&s))
            .collect();

        assert_eq!(6, host_urls.len());

        for url in host_urls.iter() {
            assert_eq!(HostValidator::Valid, url.validate(&host_urls));
        }
    }

    #[test]
    fn test_invalid_hosts() {
        let invalid_hosts = [
            "graph.on.microsoft.com",
            "microsoft.com",
            "windows.net",
            "example.org",
        ];

        let valid_hosts: Vec<Url> = vec![
            "graph.microsoft.com",
            "graph.microsoft.us",
            "dod-graph.microsoft.us",
            "graph.microsoft.de",
            "microsoftgraph.chinacloudapi.cn",
            "canary.graph.microsoft.com",
        ]
        .iter()
        .map(|s| Url::parse(&format!("https://{s}")).unwrap())
        .collect();
        assert_eq!(6, valid_hosts.len());

        let host_urls: Vec<Url> = invalid_hosts
            .iter()
            .map(|s| format!("https://{s}"))
            .flat_map(|s| Url::parse(&s))
            .collect();

        assert_eq!(4, host_urls.len());

        for url in host_urls.iter() {
            assert_eq!(HostValidator::Invalid, url.validate(valid_hosts.as_slice()));
        }
    }
}
