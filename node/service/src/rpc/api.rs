use client::Client;
use jsonrpc_core::{IoHandler, Result as JsonRpcResult};
use primitives::types::{SignedTransaction, TransactionBody, ViewCall, ViewCallResult};
use rpc::types::SendMoneyRequest;
use rpc::types::ViewAccountRequest;
use std::sync::Arc;
use rpc::types::DeployContractRequest;
use rpc::types::CallMethodRequest;

build_rpc_trait! {
    pub trait TransactionApi {
        /// Receive new transaction.
        #[rpc(name = "send_money")]
        fn rpc_send_money(&self, SendMoneyRequest) -> JsonRpcResult<()>;
        /// Call view function.
        #[rpc(name = "view_account")]
        fn rpc_view_account(&self, ViewAccountRequest) -> JsonRpcResult<ViewCallResult>;
        /// Deploy smart contract.
        #[rpc(name = "deploy_contract")]
        fn rpc_deploy_contract(&self, DeployContractRequest) -> JsonRpcResult<()>;
        /// Call method on smart contract.
        #[rpc(name = "call_method")]
        fn rpc_call_method(&self, CallMethodRequest) -> JsonRpcResult<()>;
    }
}

pub struct RpcImpl {
    pub client: Arc<Client>,
}

fn _generate_fake_signed_transaction(body: TransactionBody) -> SignedTransaction {
    SignedTransaction::new(123, body)
}

impl TransactionApi for RpcImpl {
    fn rpc_send_money(&self, r: SendMoneyRequest) -> JsonRpcResult<()> {
        let body = TransactionBody {
            nonce: r.nonce,
            sender: r.sender_account_id,
            receiver: r.receiver_account_id,
            amount: r.amount,
            method_name: String::new(),
            args: Vec::new()
        };
        let transaction = _generate_fake_signed_transaction(body);
        Ok(self.client.receive_transaction(transaction))
    }

    fn rpc_view_account(&self, r: ViewAccountRequest) -> JsonRpcResult<ViewCallResult> {
        let call = ViewCall {
            account: r.account_id,
            method_name: String::new(),
            args: Vec::new(),
        };
        Ok(self.client.view_call(&call))
    }

    fn rpc_deploy_contract(&self, r: DeployContractRequest) -> JsonRpcResult<()> {
        let body = TransactionBody {
            nonce: r.nonce,
            sender: r.sender_account_id,
            receiver: r.contract_account_id,
            amount: 0,
            method_name: "deploy".into(),
            args: vec![r.wasm_byte_array],
        };
        let transaction = _generate_fake_signed_transaction(body);
        Ok(self.client.receive_transaction(transaction))
    }

    fn rpc_call_method(&self, r: CallMethodRequest) -> JsonRpcResult<()> {
        let body = TransactionBody {
            nonce: r.nonce,
            sender: r.sender_account_id,
            receiver: r.contract_account_id,
            amount: 0,
            method_name: r.method_name,
            args: r.args,
        };
        let transaction = _generate_fake_signed_transaction(body);
        Ok(self.client.receive_transaction(transaction))
    }
}

pub fn get_handler(rpc_impl: RpcImpl) -> IoHandler {
    let mut io = IoHandler::new();
    io.extend_with(rpc_impl.to_delegate());
    io
}

#[cfg(test)]
mod tests {
    extern crate jsonrpc_test;

    use client::test_utils::generate_test_client;
    use primitives::hash::hash;
    use self::jsonrpc_test::Rpc;
    use super::*;

    #[test]
    fn test_call() {
        let client = Arc::new(generate_test_client());
        let rpc_impl = RpcImpl { client };
        let handler = get_handler(rpc_impl);
        let rpc = Rpc::from(handler);
        let t = SendMoneyRequest {
            nonce: 0,
            sender_account_id: hash(b"alice"),
            receiver_account_id: hash(b"bob"),
            amount: 1,
        };
        assert_eq!(rpc.request("send_money", &[t]), "null");
    }
}
