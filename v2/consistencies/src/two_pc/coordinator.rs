#![allow(dead_code)]
#![allow(unused_imports)]

use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use std::sync::atomic::{self, AtomicBool, Ordering};
use std::thread::JoinHandle;
use tonic::{transport::Server, Request, Response, Status};
// use participant::Participant;
use crate::two_pc::participant::Participant;
use rpc;
use std::thread;
use std::time::Duration;
use tokio::sync::futures;
use tonic::transport::Channel;
use super::futures_core::executor; 
use tokio::task;
use tokio::runtime::Builder;
use rpc::two_phase_commit::two_phase_commit_service_client::TwoPhaseCommitServiceClient;
use rpc::two_phase_commit::{PreparePhaseReq, PreparePhaseResp, CommitPhaseReq, CommitPhaseResp};
use rpc::two_phase_commit::two_phase_commit_service_server::{TwoPhaseCommitService, TwoPhaseCommitServiceServer};
use rpc::RpcClient;
use std::boxed::Box;
use std::pin::Pin;
use std::any::Any;

pub type CHANNEL<T> = Arc<(Sender<T>, Receiver<T>)>;

// type ImplRpcClient = impl RpcClient;

#[derive(Debug, Default)]
pub struct Coordinator {
    pub name: String,
    pub participants: HashMap<String, Option<String>>,
}

impl Coordinator {
    pub fn new(name: String) -> Self {
        Coordinator {
            name: name,
            // participants: vec![],
            participants: HashMap::new(),
        }
    }

    fn query_to_commit(&mut self, msg: String){
        let participants = self.participants.borrow_mut();
        for (name, p) in participants {
            tracing::debug!("query_to_commit: {:?} => {:?}", name, p);
            // p.vote(msg.to_string());
        }
    }

    pub fn pre_commit(&mut self, msg: &str){
        tracing::debug!("pre_commit : {:?}", msg);
        self.query_to_commit(msg.to_string());
    }

    pub fn start_txn(&mut self, msg: &str) {
        // start transaction
        // first phase: prepare
        // second phase: commit
        // send transaction
    }

    // abort transaction
    pub fn abort_txn(){

    }

    pub fn register_participant(self: &mut Self, name: &str, host: &str){
        if !self.participants.contains_key(name) {
            self.participants.insert(name.to_string(), Some(host.to_string()));
        } else {
            println!("Participant {:} has been existed!!!", name)
        }
        tracing::debug!("register participant {:?}", self.participants)
    }
}


