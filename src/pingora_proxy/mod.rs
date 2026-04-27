// Pingora-proxy: the logic and APIs to build HTTP proxies

// Apprications -> ports ->  proxy server  -> API -> TheWatcher
//                  load-balancing
// Apprications <- ports <-  proxy server  -> API -> TheWatcher

// @TODO Struct ProxyServiceBuilder(pingora::proxy) will be used to build custom connector and
// custome seetion handler
// https://docs.rs/pingora/latest/pingora/proxy/struct.ProxyServiceBuilder.html
// https://docs.rs/pingora/latest/pingora/proxy/struct.Session.html

use async_trait::async_trait;
use pingora::{ prelude::*, ErrorType, server };
use std::{sync::Arc, thread::Thread, time::Duration};
use tokio::time::{sleep, Duration};

const TCP_PORT: &'static str= "0.0.0.0:6188";
const SNI: &'static str= "one.one.one.one";

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
            HttpPeer::new(upstream, true, SNI.to_string())
        );
        
        Ok(peer)
    }

    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut RequestHeader,
        _ctx: &mut Self::CTX,
        ) -> Result<()> {
        
        upstream_request.insert_header("Host", SNI).unwrap();
        
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
    fn casting_type(server: Server) -> Self{
        self{ server }
    }

    // @TODO
    fn health_checking(server: &Server){
       // pingora::server::Server fn add_service() 
       // pingora Module services -> ServiceReadyNotifierServiceHandle
    }

    async fn run_forever(self) {
        let server= self.server.clone();
        
        self.run(RunArgs::default());
        self.logging_option();

        loop {
            // @TODO Add 
            health_checking(&server);
            
            sleep(Duration::from_secs(300)).await;

            std::process::exit(0);
        }
    }
}

pub fn run_pingora(proxy_server: Server){
    // server
    std::thread::spawn(move || {
        const LB_SETTING: [&'static str; 2]= ["1.1.1.1:443", "1.0.0.1:443"];
        
        let proxy_server= Server::new(None).unwrap();

        // Thread lock
        proxy_server.bootstrap();
    
        let upstreams =
            LoadBalancer::try_from_iter(LB_SETTING[0], LB_SETTING[1]).unwrap();
        
        // Arc<ServerConf>: struct ServerConf too big, so Arc<> used
        let mut lb = pingora::proxy::http_proxy_service(&proxy_server.configuration, 
            LB(Arc::new(upstreams))
        );
        lb.add_tcp(TCP_PORT);
    
        proxy_server.add_service(lb);

        // @TODO build parameters
        Proxy::ProxyHttp::early_request_filter( // -> Pin<Box<dyn Future<Result<>>>>
            proxy_server: &self,
            _sesstion: mut Sesstion,
            _ctx: Self::CTX
        );

        // @TODO build parameters
        Proxy::ProxyHttp::request_body_filter(
            proxy_server: &self,
            _session: mut Session,
            _body: mut Option<Bytes>,
            _end_of_streamL: bool,
            _ctn: Self::CTX,
        );
        
        // @TODO build parameters
        Proxy::ProxyHttp::logging(
            proxy_server: &self,
            _session: mut Session,
            _e: Option<Error>,
            _ctx: Self::CTX,
        );

        Proxy::ProxyHttp::error_while_proxy(
            proxy_server: &self,
            peer: &HttpPeer,
            sesstion: &mut Session,
            e: Box<Error>,
            _cnx: &mut Self::CTX,
            client_reused: bool
        );// -> Box<Error> 

        let custome_proxy_server= CostomServer::casting_type(proxy_server);
        custome_proxy_server.run_forever();
    });
}



// /src/api/mod.rs   
pub fn pingora_api_worker(){
    std::thread::spawn(move || {
        // poart 6188
        
        // filltering

        // logging
        // pingora::services Module listening 
    });
}
// @TODO When server comunicate with API, logging turn on
    struct Log{
        time: Date,
        source: String, // struct IpAdrr ip4 and ip6 both
        des: String,    // struct IpAdrr ip4 and ip6 both
        mac_iq: String, // i guess struct Mac already is
    }
