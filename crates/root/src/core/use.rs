use async_trait::async_trait;

use super::{node::NodeVersion, NsvCore};

#[derive(PartialEq, Debug)]
pub enum UseVersionError {
    NotValidVersion,
    NotFound,
    Empty,
}

#[derive(Clone, Debug)]
pub enum UseVersionTarget {
    Lts,
    Latest,
    Assign,
    None,
}

#[async_trait]
pub trait UseVersion {
    async fn use_version(&mut self, version: String) -> Result<(), UseVersionError>;
}

#[async_trait]
impl UseVersion for NsvCore {
    async fn use_version(&mut self, _version: String) -> Result<(), UseVersionError> {
        let version = filter_version(_version);

        // 验证 版本是否有效
        auth_version(&version)?;

        match version.as_str() {
            "lts" => {
                self.context.target = UseVersionTarget::Lts;
                let version = self.get_lts_version().await;
                if let Some(item) = version {
                    let version = item.version;
                    println!("111 {}", version)
                }
            }
            "latest" => self.context.target = UseVersionTarget::Latest,
            _ => self.context.target = UseVersionTarget::Assign,
        }

        Ok(())
    }
}

fn auth_version(version: &str) -> Result<(), UseVersionError> {
    // 如果传入版本为空
    if version == "" {
        return Err(UseVersionError::Empty);
    }

    // 如果是 lts latest origin 版本的 就通过
    if version == "lts" || version == "latest" {
        return Ok(());
    }

    let version_list = version.split(".").collect::<Vec<&str>>();

    for v in version_list {
        if let Err(_) = v.parse::<i32>() {
            return Err(UseVersionError::NotValidVersion);
        }
    }

    return Ok(());
}

fn filter_version(version: String) -> String {
    let first_char = &version[0..1];
    if first_char == "v" {
        return version[1..].to_string();
    };
    return version;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_version() {
        assert!(matches!(auth_version(""), Err(UseVersionError::Empty),));
        assert!(matches!(
            auth_version("a.1.2"),
            Err(UseVersionError::NotValidVersion)
        ));
        assert!(matches!(auth_version("1.1.2"), Ok(())));
    }

    #[test]
    fn test_filter_version() {
        assert_eq!(filter_version("v1.2.3".to_string()), "1.2.3");
        assert_eq!(filter_version("1.2.3".to_string()), "1.2.3");
        assert_eq!(filter_version("lts".to_string()), "lts");
    }
}
