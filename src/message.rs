// use crate::data::content;
use iced::pane_grid;

use crate::data::content;
use crate::data::ipfs_client::IpfsClient;
use crate::page;

use async_std::sync::Arc;
use ipfs_embed::core::{Cid, Error, Result};

#[derive(Clone, Debug)]
pub enum Message {
    FileDroppedOnWindow(std::path::PathBuf),
    PageChanged(page::PageType),
    ContentPublished(content::ContentItem),
    TestButtonPressed,
    Close(pane_grid::Pane),
    SplitPane,
    IpfsReady(Result<IpfsClient, Arc<Error>>),
    ContentAddedToIpfs(Result<Cid, Arc<Error>>),
    ContentPageInputChanged(String),
    ContentPageLoadContent,
    ContentPageImageLoaded(Result<Vec<u8>, Arc<Error>>),
    SitePageContentChanged(String),
    SitePagePublishButtonClicked,
}
