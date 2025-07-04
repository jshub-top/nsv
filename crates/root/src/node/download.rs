use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

use async_trait::async_trait;
use serde_json::Value;
use tokio::{
    fs::{read, remove_dir_all, remove_file, rename, write}
};

use crate::{
    core::NsvCore, node::NodeLtsTarget, util::{
        dir::ensure_dir,
        download::{unzip_file, write_file},
        http::get,
    }
};

use super::{NodeVersionItem, NsvCoreError};

#[async_trait]
pub trait NodeDisposeDownload {
    // 下载node文件
    async fn download_node(&self, version: &str) -> Result<(), NsvCoreError>;

    // 下载 node 版本的 index.json 文件
    async fn download_dist_version(&mut self) -> Result<Arc<Vec<NodeVersionItem>>, NsvCoreError>;

    // 获取下载 node 的文件名
    fn get_download_file_name(&self, version: &str) -> String;

    // 解压node文件
    async fn unzip_node_file(&self, version: &str) -> Result<(), NsvCoreError>;

    // 解压node文件
    fn transform_version_json(&self,  version_json_bytes: &[u8]) -> Result<Vec<NodeVersionItem>, NsvCoreError>;
}

#[async_trait]
impl NodeDisposeDownload for NsvCore {
    async fn unzip_node_file(&self, version: &str) -> Result<(), NsvCoreError> {
        let file_name = self.get_download_file_name(version);
        let file_path = self.context.node_file.join(&file_name);
        let output_dir = self.context.temp.clone();

        // 清空这个文件夹
        remove_dir_all(&output_dir).await.unwrap();
        ensure_dir(&output_dir).await.unwrap();

        let node_dir_file_name = file_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .replace(&format!(".{}", self.context.rar_extension), "");

        let mut unzip_node_path = output_dir;

        //先解压到临时文件夹
        unzip_file(&file_path, &unzip_node_path).await.unwrap();

        unzip_node_path.push(node_dir_file_name);

        let node_dir = self.context.node_dir.join(version);
        rename(&unzip_node_path, &node_dir).await.unwrap();
        Ok(())
    }

    fn get_download_file_name(&self, version: &str) -> String {
        format!(
            "node-{}-{}-{}.{}",
            version, self.context.os, self.context.arch, self.context.rar_extension
        )
    }
    async fn download_node(&self, version: &str) -> Result<(), NsvCoreError> {
        let file_name = self.get_download_file_name(version);
        let url = format!(
            "{}/{}/{}",
            self.config.get::<String>("origin"),
            version,
            file_name
        );

        // 先下载到 临时文件夹
        let target = self.context.temp.join(&file_name);
        let res = get(&url).await.unwrap();
        write_file(res, &target).await.unwrap();
        // 复制到 放node文件的文件夹
        rename(target, self.context.node_file.join(file_name))
            .await
            .unwrap();
        Ok(())
    }

    async fn download_dist_version(&mut self) -> Result<Arc<Vec<NodeVersionItem>>, NsvCoreError> {
        if self.context.node_version_list.len() != 0 {
            return Ok(self.context.node_version_list.clone());
        }

        let dist_version_path = self.context.nsv_home.join("version.json");

        let mut version_json_bytes = None;

        // 如果 本地存在 就用本地的
        if dist_version_path.exists() {
            let file_meta = dist_version_path.metadata().unwrap();

            let file_create_time = file_meta.created().unwrap();
            let current_time = SystemTime::now();
            let time_difference = current_time
                .duration_since(file_create_time)
                .unwrap_or_default();

            let is_recent = time_difference
                <= Duration::from_secs(self.config.get("index_json_file_effect_time"));

            if is_recent {
                let bug = read(&dist_version_path).await.unwrap();
                version_json_bytes = Some(bug);
            }
        }

        if version_json_bytes.is_none() {
            let url = format!("{}/index.json", self.config.get::<String>("origin"));
            let resp = reqwest::get(url).await.unwrap();
            let resp_byt = resp.bytes().await.unwrap();
            version_json_bytes = Some(resp_byt.to_vec());

            // 缓存到本地
            let _ = remove_file(&dist_version_path).await;
            write(&dist_version_path, resp_byt).await.unwrap();
        }

        if version_json_bytes.is_none() {
            return Err(NsvCoreError::Str("download_dist_version error"));
        }

        let version_json_bytes = version_json_bytes.unwrap();

        self.context.node_version_list = Arc::new(self.transform_version_json(&version_json_bytes)?);

        Ok(self.context.node_version_list.clone())
    }

    fn transform_version_json(&self, version_json_bytes: &[u8]) -> Result<Vec<NodeVersionItem>, NsvCoreError> {
        let resp_json: Value = serde_json::from_slice(&version_json_bytes).unwrap();
        let version_list = resp_json.as_array().unwrap();
        let installed_version = self.context.local_version.clone();
        let version_list = version_list.iter().map(|item| {

            // 版本
            let version = item["version"].as_str().unwrap().to_string();

            // 发布日期
            let date = item["date"].as_str().unwrap().to_string();

            // lts
            let lts = match item["lts"].as_bool() {
                Some(true) => NodeLtsTarget::Bool(true),
                Some(false) => NodeLtsTarget::Bool(false),
                None =>  NodeLtsTarget::Str(item["lts"].as_str().unwrap().to_string()),
            };

            // 安全版本
            let security = item["security"].as_bool().unwrap();

            // 是否安装
            let is_installed = installed_version.contains(&version);


            return NodeVersionItem {
                version,
                date,
                lts,
                security,
                is_installed,
                module: item["module"].as_str().map(|s| s.to_string()),
                openssl: item["openssl"].as_str().map(|s| s.to_string()),
                zlib: item["zlib"].as_str().map(|s| s.to_string()),
                uv: item["uv"].as_str().map(|s| s.to_string()),
                v8: item["v8"].as_str().map(|s| s.to_string()),
                npm: item["npm"].as_str().map(|s| s.to_string()),
            };
        }).collect();
        Ok(version_list)
    }
}
