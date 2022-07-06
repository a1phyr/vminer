use ibc::{IceResult, ResultExt};
use std::{io::Read, path::PathBuf};

pub struct SymbolLoader {
    root: PathBuf,
    url_base: String,
}

impl SymbolLoader {
    #[cfg(target_os = "windows")]
    pub fn with_system_root() -> IceResult<Self> {
        Self::with_root(r"C:\ProgramData\Dbg\sym".into())
    }

    pub fn with_root(root: PathBuf) -> IceResult<Self> {
        Self::with_root_and_url(root, "https://msdl.microsoft.com/download/symbols".into())
    }

    pub fn with_root_and_url(root: PathBuf, url_base: String) -> IceResult<Self> {
        std::fs::create_dir_all(&root)?;
        Ok(Self { root, url_base })
    }

    #[cfg(feature = "download_pdb")]
    fn download_pdb(
        &self,
        path: &std::path::Path,
        name: &str,
        id: &str,
    ) -> IceResult<ibc::ModuleSymbols> {
        // Download PDB
        let url = format!("{}/{name}/{id}/{name}", self.url_base);
        log::info!("Downloading {name}...");
        let mut pdb = Vec::new();
        ureq::get(&url)
            .call()
            .context("failed to dowload PDB")?
            .into_reader()
            .read_to_end(&mut pdb)
            .context("failed to dowload PDB")?;

        // Save it to the filesystem
        let res = (|| {
            std::fs::create_dir_all(path.parent().unwrap())?;
            std::fs::write(&path, &pdb)
        })();
        if let Err(err) = res {
            log::error!("Failed to write PDB at {}: {err}", path.display());
        }

        ibc::ModuleSymbols::from_bytes(&pdb)
    }
}

impl super::super::SymbolLoader for SymbolLoader {
    fn load(&self, name: &str, id: &str) -> IceResult<Option<ibc::ModuleSymbols>> {
        let components = [&*self.root, name.as_ref(), id.as_ref(), name.as_ref()];
        let path: PathBuf = components.iter().collect();

        if path.exists() {
            ibc::ModuleSymbols::from_file(path).map(Some)
        } else {
            #[cfg(feature = "download_pdb")]
            return self.download_pdb(&path, name, id).map(Some);

            #[cfg(not(feature = "download_pdb"))]
            Ok(None)
        }
    }
}
