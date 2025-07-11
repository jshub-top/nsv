use std::{env::consts::EXE_SUFFIX, path::PathBuf};

use anyhow::Result;
use util::fs::ensure_dir;

use crate::core::core::Main;

pub trait MainDownloadExt {
    async fn download_nsv_binary(&self) -> Result<()>;
    fn nsv_binary_name(&self) -> String;
}

impl MainDownloadExt for Main {
    async fn download_nsv_binary(&self) -> Result<()> {

        let nsv_binary_path = PathBuf::from(&self.config.nsv_home).join("nsv");
        ensure_dir(nsv_binary_path.parent().unwrap()).await?;

        let nsv_binary_url = format!("{}/repos/{}/nsv/releases/tags/nsv-v{}", self.config.origin, self.context.owner, self.context.version);
        let nsv_binary_name = self.nsv_binary_name();

        println!("nsv_binary_url: {}", nsv_binary_url);
        println!("nsv_binary_name: {}", nsv_binary_name);

        let response = reqwest::get(nsv_binary_url).await?;
        let body = response.text().await?;
        println!("body: {}", body);

        Ok(())
    }

    fn nsv_binary_name(&self) -> String {
        format!("nsv-{}-{}{}", self.context.arch, self.context.os, EXE_SUFFIX)
    }
}
