use crate::types::Types;
use tokio::io::AsyncReadExt;

pub(crate) enum Command {
    None,
    Ping,
    Set { key: Types, value: Types },
}

pub(crate) async fn parse_user_command<R: AsyncReadExt + Unpin>(
    mut reader: R,
) -> anyhow::Result<Command> {
    let mut buf = [0u8; 1];
    let mut n = reader.read(&mut buf).await?;
    if n == 0 {
        return Ok(Command::None);
    }

    Ok(Command::None)
}

async fn parse_line_until_cr_lf<R: AsyncReadExt + Unpin>(
    mut reader: R,
) -> anyhow::Result<Option<Vec<u8>>> {
    let mut result = Vec::with_capacity(1024 * 1024);
    let mut buf = [0u8; 1];
    let mut seen_any = false;

    loop {
        let n = reader.read(&mut buf).await?;
        if n == 0 {
            if !seen_any {
                return Ok(None);
            } else {
                return Err(anyhow::anyhow!("Unexpected EOF"));
            }
        }

        seen_any = true;
        result.push(buf[0]);

        let len = result.len();
        if len >= 2 && result[len - 2] == b'\r' && result[len - 1] == b'\n' {
            result.truncate(len - 2);
            return Ok(Some(result));
        }
    }
}
