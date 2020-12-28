// #![allow(unused_imports)] // TODO: Dependencies cleanup
use ipfs_embed::core::{BitswapStorage, BitswapStore, BitswapSync, Error, Result, Storage};
use ipfs_embed::db::StorageService;
use ipfs_embed::net::{NetworkConfig, NetworkService};
use ipfs_embed::DefaultIpfs;
use ipfs_embed::Ipfs;
use libipld::block::Block;
use libipld::cbor::DagCborCodec;
use libipld::multihash::Code;
use libipld::store::{DefaultParams, Store};
use libipld::{alias, Cid, DagCbor};

use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

use async_std::fs::read;
use async_std::sync::{Arc, Mutex};
use directories_next::ProjectDirs;

use iced_futures::futures;
use iced_futures::futures::channel::mpsc;
use iced_futures::futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::data::content::{ContentItem, ContentItemBlock, ImageContent, PageContent};
use crate::data::tasks::Task;

type IpfsEmbed = Ipfs<DefaultParams, StorageService<DefaultParams>, NetworkService<DefaultParams>>;
type MaybeIpfs = Option<Arc<Mutex<IpfsEmbed>>>;
type ThreadsafeIpfsClient = Arc<Mutex<IpfsClient>>;
type ThreadsafeIpfsResult<T> = Result<T, Arc<Error>>;

#[derive(Clone)]
pub struct IpfsClient {
    ipfs: MaybeIpfs,
    // task_sender: UnboundedSender<Task>,
    // task_receiver: UnboundedReceiver<Task>,
}

impl IpfsClient {
    // task_sender: UnboundedSender<Task>, task_receiver: UnboundedReceiver<Task>
    pub async fn new() -> Result<IpfsClient, Arc<Error>> {
        let path = match ProjectDirs::from("net", "FuzzrNet", "Fuzzr") {
            Some(project_dirs) => project_dirs.data_local_dir().to_owned(),
            None => PathBuf::from("/tmp/fuzzr"),
        };
        let cache_size: usize = 10;

        let ipfs = Some(Arc::new(Mutex::new(
            DefaultIpfs::default(Some(path.join("blocks")), cache_size)
                .await
                .unwrap(),
        )));

        Ok(IpfsClient { ipfs }) // task_sender, task_receiver
    }

    pub async fn add(&self, block: &ContentItemBlock) -> Result<Cid, Arc<Error>> {
        let ipld_block = libipld::Block::encode(DagCborCodec, Code::Blake3_256, block)?;

        if let Some(ipfs) = self.ipfs.clone() {
            let ipfs = ipfs.lock().await;
            ipfs.insert(&ipld_block).await?;
            let cid = *ipld_block.cid();
            Ok(cid)
        } else {
            Err(Arc::new(Error::msg(
                "Embedded IPFS not yet initialized when content was added",
            )))
        }
    }

    pub async fn get(&self, cid: &Cid) -> Result<Block<DefaultParams>, Arc<Error>> {
        if let Some(ipfs) = self.ipfs.clone() {
            let ipfs = ipfs.lock().await;
            let content = ipfs.get(cid).await?;
            Ok(content)
        } else {
            Err(Arc::new(Error::msg(
                "Embedded IPFS not yet initialized when content was retrieved",
            )))
        }
    }

    pub async fn add_file_from_path(&self, path: PathBuf) -> Result<Cid, Arc<Error>> {
        let buffer = async_std::fs::read(path).await.unwrap();

        let block = ContentItemBlock {
            content: ContentItem::Image(ImageContent { buffer }), // TODO: validate via magic number
        };

        let cid = self.add(&block).await?;

        Ok(cid)
    }

    pub async fn get_bytes_from_cid_string(
        &self,
        cid_string: String,
    ) -> Result<Vec<u8>, Arc<Error>> {
        let cid = Cid::from_str(&cid_string).unwrap();
        let data = self.get(&cid).await?;

        Ok(data.to_bytes())
    }
}

impl fmt::Debug for IpfsClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<TODO IpfsClient debug formatting>")
    }
}

// pub async fn init_ipfs(ipfs_client: Arc<Mutex<IpfsClient>>) -> ThreadsafeIpfsResult<bool> {
//     let mut ipfs = ipfs_client.lock().await;
//     Ok(ipfs.init().await?)
// }

// TODO: Graveyard of potentially useful shit
// let index = sled::open(path.join("index"))?;

// let sweep_interval = Duration::from_millis(10000);
// let net_config = NetworkConfig::new();
// let storage = Arc::new(StorageService::open(&sled_config, cache_size, sweep_interval).unwrap());
// let bitswap_storage = BitswapStorage::new(storage.clone());
// let network = Arc::new(NetworkService::new(net_config, bitswap_storage).unwrap());
// let ipfs = Ipfs::new(storage, network);

// let cid = ipfs.insert(&identity).await?;
