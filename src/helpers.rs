use anyhow::bail;
use surf::Exception;

// this is required due to https://github.com/http-rs/surf/issues/86
pub(crate) fn surf2anyhow<T>(r: Result<T, Exception>) -> anyhow::Result<T> {
    match r {
        Ok(v) => Ok(v),
        Err(e) => bail!(e),
    }
}
