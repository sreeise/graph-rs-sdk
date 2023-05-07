use std::collections::HashSet;
use std::hash::Hash;
use url::{Host, Url};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HostValidator {
    Valid,
    Invalid,
}

pub trait ValidateHosts<RHS = Self> {
    fn validate_hosts(&self, valid_hosts: &[Url]) -> HostValidator;
}

impl ValidateHosts for Url {
    fn validate_hosts(&self, valid_hosts: &[Url]) -> HostValidator {
        if valid_hosts.is_empty() {
            return HostValidator::Invalid;
        }

        let size_before = valid_hosts.len();
        let hosts: Vec<Host<&str>> = valid_hosts.iter().flat_map(|url| url.host()).collect();
        assert_eq!(size_before, hosts.len());

        if let Some(host) = self.host() {
            if hosts.contains(&host) {
                return HostValidator::Valid;
            }
        }

        for value in valid_hosts.iter() {
            if !value.scheme().eq("https") {
                return HostValidator::Invalid;
            }
        }

        HostValidator::Invalid
    }
}

impl ValidateHosts for String {
    fn validate_hosts(&self, valid_hosts: &[Url]) -> HostValidator {
        if let Ok(url) = Url::parse(self) {
            return url.validate_hosts(valid_hosts);
        }

        HostValidator::Invalid
    }
}

impl ValidateHosts for &str {
    fn validate_hosts(&self, valid_hosts: &[Url]) -> HostValidator {
        if let Ok(url) = Url::parse(self) {
            return url.validate_hosts(valid_hosts);
        }

        HostValidator::Invalid
    }
}

#[derive(Clone, Debug)]
pub struct AllowedHostValidator {
    allowed_hosts: HashSet<Url>,
}

impl AllowedHostValidator {
    pub fn new(allowed_hosts: HashSet<Url>) -> AllowedHostValidator {
        for url in allowed_hosts.iter() {
            if !url.scheme().eq("https") {
                panic!("Requires https scheme");
            }
        }

        AllowedHostValidator { allowed_hosts }
    }

    pub fn validate_str(&self, url_str: &str) -> HostValidator {
        if let Ok(url) = Url::parse(url_str) {
            return self.validate_hosts(&[url]);
        }

        HostValidator::Invalid
    }

    pub fn validate_url(&self, url: &Url) -> HostValidator {
        self.validate_hosts(&[url.clone()])
    }
}

impl From<&[Url]> for AllowedHostValidator {
    fn from(value: &[Url]) -> Self {
        let hash_set = HashSet::from_iter(value.iter().cloned());
        AllowedHostValidator::new(hash_set)
    }
}

impl ValidateHosts for AllowedHostValidator {
    fn validate_hosts(&self, valid_hosts: &[Url]) -> HostValidator {
        if valid_hosts.is_empty() {
            return HostValidator::Invalid;
        }

        let urls: Vec<Url> = self.allowed_hosts.iter().cloned().collect();
        for url in valid_hosts.iter() {
            if url
                .validate_hosts(urls.as_slice())
                .eq(&HostValidator::Invalid)
            {
                return HostValidator::Invalid;
            }
        }

        HostValidator::Valid
    }
}

impl Default for AllowedHostValidator {
    fn default() -> Self {
        let urls: HashSet<Url> = vec![
            "https://graph.microsoft.com",
            "https://graph.microsoft.us",
            "https://dod-graph.microsoft.us",
            "https://graph.microsoft.de",
            "https://microsoftgraph.chinacloudapi.cn",
            "https://canary.graph.microsoft.com",
        ]
        .iter()
        .flat_map(|url_str| Url::parse(url_str))
        .collect();
        assert_eq!(6, urls.len());

        AllowedHostValidator::new(urls)
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
            assert_eq!(HostValidator::Valid, url.validate_hosts(&host_urls));
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
            assert_eq!(
                HostValidator::Invalid,
                url.validate_hosts(valid_hosts.as_slice())
            );
        }
    }

    #[test]
    fn test_allowed_host_validator() {
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

        let allowed_host_validator = AllowedHostValidator::from(host_urls.as_slice());

        for url in host_urls.iter() {
            assert_eq!(
                HostValidator::Valid,
                allowed_host_validator.validate_url(url)
            );
        }
    }
}
