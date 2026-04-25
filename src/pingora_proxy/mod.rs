// Pingora-proxy: the logic and APIs to build HTTP proxies

// Apprications -> ports ->  proxy server  -> API -> TheWatcher
//                  load-balancing
// Apprications <- ports <-  proxy server  -> API -> TheWatcher

use async_trait::async_trait;
use pingora::{ prelude::*, ErrorType };
use std::{sync::Arc, thread::Thread, time::Duration};

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

impl ProxyHttp for LB {
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

// IDK I will give a question to Google's Gemini
struct costom_server for pingora_core::server::Server{}
impl custom_server_run for Box<impl Server>{
    fn run_forever(self) -> ! {
        self.run(RunArgs::default());

        loop {
            heath_checking();

            let duration= Duration::new(300, Default::default())
            sleep(duration);
        }

        std::process::exit(0)
    }
}

pub fn run_pingora(proxy_server: Server){
    std::thread::spawn(move || {
        let proxy_server= Server::new(None).unwrap();

        // Thread lock
        proxy_server.bootstrap();
    
        let upstreams =
            LoadBalancer::try_from_iter(["1.1.1.1:443", "1.0.0.1:443"]).unwrap();
        
        // Arc<ServerConf>: struct ServerConf is big, so Arc<> used
        // IDK "Module proxy"'s meaning
        let mut lb = pingora::proxy::http_proxy_service(&proxy_server.configuration, 
            LB(Arc::new(upstreams))
        );
        lb.add_tcp("0.0.0.0:6188");
    
        proxy_server.add_service(lb);

        heath_checking();
        
        // proxy_server.run_forever();
        custom_server_run::run_forever(proxy_server);
    });
}