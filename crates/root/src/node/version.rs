use std::cmp::Ordering;

use async_trait::async_trait;
use cursive::{
    align::HAlign, theme::{BorderStyle, Palette}, view::{Nameable, Resizable, Scrollable}, views::{Dialog, ListView, TextView}, Cursive, With
};
use tokio::fs::read_dir;

use crate::{
    core::NsvCore, library::cursive_table::TableViewItem, node::download::NodeDisposeDownload,
};

use super::{NodeLtsTarget, NodeVersionItem, NsvCoreError};

#[async_trait]
pub trait NodeDisposeVersion {
    /// 查找本地 node 版本
    async fn find_local_version(&self, version: &str) -> Result<String, NsvCoreError>;

    /// 查找远程 node 版本
    async fn find_remote_version(&self, version: &str) -> Result<String, NsvCoreError>;

    /// 格式化 用户输入的版本
    async fn formatter_version(&self, version: &str) -> Result<String, NsvCoreError>;

    async fn view_version_detail(&mut self, item: &NodeVersionItem) -> Result<(), NsvCoreError>;
    async fn view_version_list(&mut self) -> Result<(), NsvCoreError>;
}

#[async_trait]
impl NodeDisposeVersion for NsvCore {
    async fn find_local_version(&self, version: &str) -> Result<String, NsvCoreError> {
        let node_dir = self.context.node_dir.as_path();
        let mut local_node_dirs = read_dir(&node_dir).await.unwrap();
        while let Ok(Some(entry)) = local_node_dirs.next_entry().await {
            let file_name = entry.file_name();
            if let Some(name) = file_name.to_str() {
                // 如果是以 输入版本开头的
                if name.starts_with(version) {
                    return Ok(name.to_string());
                }
            }
        }
        return Err(NsvCoreError::NodeVersionLocalNotFound);
    }

    async fn find_remote_version(&self, version: &str) -> Result<String, NsvCoreError> {
        let version_list = self.context.node_version_list.clone();

        let current_version_item = version_list.iter().find(|item| {
            let (_, remote_version) = item.version.split_at(1);
            remote_version.starts_with(version)
        });

        if current_version_item.is_none() {
            return Err(NsvCoreError::NodeVersionRemoteNotFound);
        }

        Ok(current_version_item.unwrap().version.clone())
    }

    async fn formatter_version(&self, version: &str) -> Result<String, NsvCoreError> {
        match version.trim() {
            "lts" => {
                let current_version_item =
                    self.context
                        .node_version_list
                        .iter()
                        .find(|item| match item.lts {
                            // 当 lts是字符串时候就可以了
                            NodeLtsTarget::Str(_) => true,
                            _ => false,
                        });

                if current_version_item.is_none() {
                    return Err(NsvCoreError::NodeVersionRemoteNotFound);
                }

                return Ok(current_version_item.unwrap().version.clone());
            }
            "latest" => {
                // 最新版本就获取 最新的呢个 版本
                let current_version_item = self.context.node_version_list.get(0);

                if current_version_item.is_none() {
                    return Err(NsvCoreError::NodeVersionRemoteNotFound);
                }

                return Ok(current_version_item.unwrap().version.clone());
            }
            _ => {
                let (char, _) = version.split_at(1);
                if char == "v" {
                    return Ok(version.to_string());
                }

                Ok(format!("v{version}"))
            }
        }
    }

    async fn view_version_detail(&mut self, item: &NodeVersionItem) -> Result<(), NsvCoreError> {
        Ok(())
    }

    async fn view_version_list(&mut self) -> Result<(), NsvCoreError> {
        let list = self.download_dist_version().await?;

        use crate::library::cursive_table::*;

        let mut siv = cursive::default();
        let mut table = TableView::<NodeVersionItem, BasicColumn>::new()
            .column(BasicColumn::Version, "version", |c| c.align(HAlign::Center).width(10))
            .column(BasicColumn::Date, "date", |c| {
                c.align(HAlign::Center).width(10)

            })
            .column(BasicColumn::Lts, "lts", |c| c.align(HAlign::Center).width(10))
            .column(BasicColumn::Security, "security", |c| {
                c.align(HAlign::Center).width(12)
            })
            .column(BasicColumn::Installed, "installed", |c| {
                c.align(HAlign::Center).width(12)
            });

        table.set_items(list.to_vec());

        let node_version_list = self.context.node_version_list.clone();

        table.set_on_submit(move |siv: &mut Cursive, _row, index: usize| {
            let item = node_version_list.get(index).unwrap();

            siv.add_layer(
                Dialog::new()
                    .title(format!("{} detail", item.version))
                    .button("Close", |s| {s.pop_layer();})
                    .content(
                        ListView::new()
                            .child("verison", TextView::new(&item.version))
                            .child("date", TextView::new(&item.date))
                            .child("lts", match &item.lts {
                                NodeLtsTarget::Bool(val) => TextView::new(val.to_string()),
                                NodeLtsTarget::Str(val) => TextView::new(val.to_string()),
                            })
                            .child("security", TextView::new(item.security.to_string()))
                            .child("module", TextView::new(item.module.clone().unwrap_or("".to_string())))
                            .child("openssl", TextView::new(item.openssl.clone().unwrap_or("".to_string())))
                            .child("zlib", TextView::new(item.zlib.clone().unwrap_or("".to_string())))
                            .child("uv", TextView::new(item.uv.clone().unwrap_or("".to_string())))
                            .child("v8", TextView::new(item.v8.clone().unwrap_or("".to_string())))
                            .child("npm", TextView::new(item.npm.clone().unwrap_or("".to_string())))
                            .delimiter()
                            .child("installed", TextView::new(item.is_installed.to_string()))
                            .scrollable()
                    ))
        });


        siv.set_theme(cursive::theme::Theme {
            shadow: true,
            borders: BorderStyle::Simple,
            palette: Palette::retro().with(|palette| {
                use cursive::style::BaseColor::*;

                {
                    // First, override some colors from the base palette.
                    use cursive::style::Color::TerminalDefault;
                    use cursive::style::PaletteColor::*;

                    palette[Background] = TerminalDefault;
                    palette[View] = TerminalDefault;
                    palette[Primary] = White.dark();
                    palette[TitlePrimary] = Blue.light();
                    palette[Secondary] = Blue.light();
                    palette[Highlight] = Blue.dark();
                }

                {
                    // Then override some styles.
                    use cursive::style::Effect::*;
                    use cursive::style::PaletteStyle::*;
                    use cursive::style::Style;
                    palette[Highlight] = Style::from(Blue.light()).combine(Bold);
                    palette[EditableTextCursor] =
                        Style::secondary().combine(Reverse).combine(Underline)
                }
            }),
        });

        siv.add_layer(Dialog::around(table.with_name("table").full_height().min_width(68)).title("version list"));

        siv.run();

        Ok(())
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum BasicColumn {
    Version,
    Date,
    Lts,
    Security,
    Installed,
}

impl TableViewItem<BasicColumn> for NodeVersionItem {
    fn to_column(&self, column: BasicColumn) -> String {
        match column {
            BasicColumn::Version => self.version.to_string(),
            BasicColumn::Date => self.date.to_string(),
            BasicColumn::Lts => match &self.lts {

                NodeLtsTarget::Bool(_val) => "".to_string(),
                NodeLtsTarget::Str(val) => val.to_string(),
            }
            BasicColumn::Security => match self.security {
                true => "*".to_string(),
                false => "".to_string(),
            }
            BasicColumn::Installed => match self.is_installed {
                true => "*".to_string(),
                false => "".to_string(),
            }
        }
    }

    fn cmp(&self, other: &Self, column: BasicColumn) -> Ordering
    where
        Self: Sized,
    {
        match column {
            BasicColumn::Version => Ordering::Equal,
            BasicColumn::Date => other.date.cmp(&self.date),
            BasicColumn::Lts => Ordering::Equal,
            BasicColumn::Security => other.security.cmp(&self.security),
            BasicColumn::Installed => other.is_installed.cmp(&self.is_installed),
        }
    }
}
