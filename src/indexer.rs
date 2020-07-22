use anyhow::Result;

pub(crate) trait IndexerTrait {
    fn update_index() -> Result<()>;
}
