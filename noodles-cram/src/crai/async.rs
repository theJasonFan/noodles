//! Async CRAM index.

pub mod io;

use std::path::Path;

use tokio::fs::File;

use self::io::{Reader, Writer};
use super::{Index, Record};

/// Reads the entire contents of a CRAM index.
///
/// This is a convenience function and is equivalent to opening the file at the given path and
/// reading the index.
///
/// # Examples
///
/// ```no_run
/// # use std::io;
/// #
/// # #[tokio::main]
/// # async fn main() -> io::Result<()> {
/// use noodles_cram::crai;
/// let index = crai::r#async::read("sample.cram.crai").await?;
/// # Ok(())
/// # }
/// ```
pub async fn read<P>(src: P) -> tokio::io::Result<Index>
where
    P: AsRef<Path>,
{
    let mut reader = File::open(src).await.map(Reader::new)?;
    reader.read_index().await
}

/// Writes a CRAM index to a file.
///
/// This is a convenience function and is equivalent to creating a file at the given path and
/// writing the index.
///
/// # Examples
///
/// ```no_run
/// # use std::io;
/// #
/// # #[tokio::main]
/// # async fn main() -> io::Result<()> {
/// use noodles_cram::crai;
/// let index = crai::Index::default();
/// crai::r#async::write("sample.cram.crai", &index).await?;
/// # Ok(())
/// # }
/// ```
pub async fn write<P>(dst: P, index: &[Record]) -> tokio::io::Result<()>
where
    P: AsRef<Path>,
{
    let mut writer = File::create(dst).await.map(Writer::new)?;
    writer.write_index(index).await?;
    writer.shutdown().await?;
    Ok(())
}
