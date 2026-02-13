pub mod rock_paper_scissors;
pub mod server;

use log::error;

use std::io::Error;

use crate::server::ServerBuilder;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = env_logger::try_init();

    match ServerBuilder::new().port(6767).build() {
        Ok(server) => {
            server.run().await?;
        }
        Err(e) => {
            error!("{:?}", e);
        }
    }

    Ok(())
}
