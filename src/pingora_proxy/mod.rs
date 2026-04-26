// Pingora-proxy: the logic and APIs to build HTTP proxies

// Apprications -> ports ->  proxy server  -> API -> TheWatcher
//                  load-balancing
// Apprications <- ports <-  proxy server  -> API -> TheWatcher

use async_trait::async_trait;
use pingora::{ prelude::*, ErrorType, server };
use std::{sync::Arc, thread::Thread, time::Duration};
use tokio::time::{sleep, Duration};

const TCP_PORT: &'static str= "0.0.0.0:6188";

pub struct LB(Arc<LoadBalancer<RoundRobin>>);

#[async_trait]
impl ProxyHttp for LB {
    async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        let upstream = self.0
            .select(b"", 256) // hash doesn't matter for round robin
            .unwrap();

        println!("upstream peer is: {upstream:?}");

        // Set SNI to one.one.one.one
        let peer = Box::new(
            HttpPeer::new(upstream, true, "one.one.one.one".to_string())
        );
        
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

// @TODO idk Model meaning
fn heath_checking(){
    // pingora::lb::Backend
}
struct CostomServer{
    server: server,
}
impl CostomServer for Box<impl Server>{
    fn casting(server: Server) -> Self{
        self{ server }
    }

    async fn run_forever(self) {
        let server= self.server.clone();
        
        self.run(RunArgs::default());
        
        loop {
            health_checking(server);
            
            sleep(Duration::from_secs(300)).await;

            std::process::exit(0);
        }
    }
}

pub fn run_pingora(proxy_server: Server){
    std::thread::spawn(move || {
        let proxy_server= Server::new(None).unwrap();

        // Thread lock
        proxy_server.bootstrap();
    
        const LB_SETTING: [&'static str; 2]= ["1.1.1.1:443", "1.0.0.1:443"];
        let upstreams =
            LoadBalancer::try_from_iter(LB_SETTING[0], LB_SETTING[1]).unwrap();
        
        // Arc<ServerConf>: struct ServerConf is big, so Arc<> used
        let mut lb = pingora::proxy::http_proxy_service(&proxy_server.configuration, 
            LB(Arc::new(upstreams))
        );
        lb.add_tcp(TCP_PORT);
    
        proxy_server.add_service(lb);

        let custome_proxy_server= CostomServer::casting(proxy_server);
        custome_proxy_server.run_forever();
    });
}
