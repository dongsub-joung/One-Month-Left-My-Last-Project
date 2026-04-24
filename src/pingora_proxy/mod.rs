// Pingora-proxy: the logic and APIs to build HTTP proxies

// Apprications -> ports ->  proxy server  -> API -> TheWatcher
//                  load-balancing
// Apprications <- ports <-  proxy server  -> API -> TheWatcher

use async_trait::async_trait;
use pingora::prelude::*;
use std::sync::Arc;

pub struct LB(Arc<LoadBalancer<RoundRobin>>);
#[async_trait]
impl ProxyHttp for LB {
    async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        let upstream = self.0
            .select(b"", 256) // hash doesn't matter for round robin
            .unwrap();

        println!("upstream peer is: {upstream:?}");

        // Set SNI to one.one.one.one
        let peer = Box::new(HttpPeer::new(upstream, true, "one.one.one.one".to_string()));
        
        Ok(peer)
    }

    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut RequestHeader,
        _ctx: &mut Self::CTX,
        ) -> Result<()> {
        
        upstream_request.insert_header("Host", "one.one.one.one").unwrap();
        
        Ok(())
    }
}

fn create_pingora_server(){
    let mut my_server = Server::new(None).unwrap();
    my_server.bootstrap();
}

pub fn run_pingora(server: Server){
    server.run_forever();
}