use crate::parse_arg::CliArgs;
use std::{net::IpAddr, slice::Iter};

#[derive(Debug, Hash, Clone)]
pub struct HostInfo {
    elapsed: std::time::Duration,
    ips: Vec<IpAddr>,
}

impl HostInfo {
    pub async fn try_from(address: &str) -> Result<Self, ()> {
        let now = std::time::Instant::now();
        (tokio::time::timeout(
            std::time::Duration::from_millis(5000),
            tokio::net::lookup_host(format!("{address}:80")),
        )
        .await)
            .map_or(Err(()), |lookup| {
                lookup.map_or(Err(()), |mut addr| {
                    let mut all_ips = vec![];
                    for socket_addr in addr.by_ref() {
                        all_ips.push(socket_addr.ip());
                    }
                    if all_ips.is_empty() {
                        Err(())
                    } else {
                        all_ips.sort();
                        Ok(Self {
                            elapsed: now.elapsed(),
                            ips: all_ips,
                        })
                    }
                })
            })
    }

    pub fn get_ip(&self, cli_args: &CliArgs) -> Option<&IpAddr> {
        self.ips.iter().find(|x| {
            if cli_args.ip6 {
                x.is_ipv6()
            } else {
                x.is_ipv4()
            }
        })
    }

    pub fn iter_ip(&self) -> Iter<IpAddr> {
        self.ips.iter()
    }

    pub fn ip_len(&self) -> usize {
        self.ips.len()
    }

    #[cfg(test)]
    pub const fn test_get(ips: Vec<IpAddr>) -> Self {
        Self {
            elapsed: std::time::Duration::from_millis(0),
            ips,
        }
    }
}
