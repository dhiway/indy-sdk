extern crate libc;

use std::sync::mpsc::channel;
use std::ffi::CString;
use std::ptr::null;

use indy::api::authz::*;
use indy::api::ErrorCode;

use utils::callback::CallbackUtils;
use utils::timeout::TimeoutUtils;
use utils::ledger::LedgerUtils;

pub struct AuthzUtils {}

impl AuthzUtils {
    pub fn create_and_store_policy_address(wallet_handle: i32) -> Result<String, ErrorCode> {
        let (create_and_store_policy, create_and_store_policy_receiver) = channel();
        let create_and_store_my_policy = Box::new(move |err, address| {
            create_and_store_policy.send((err, address)).unwrap();
        });
        let (create_and_store_policy_command_handle, create_and_store_policy_callback) = CallbackUtils::closure_to_create_and_store_policy_cb(create_and_store_my_policy);

        /*let my_did_json = seed.map_or("{}".to_string(), |seed| format!("{{\"seed\":\"{}\" }}", seed));

        let my_did_json = CString::new(my_did_json).unwrap();*/

        let err =
            indy_create_and_store_new_policy(create_and_store_policy_command_handle,
                                         wallet_handle,
                                         create_and_store_policy_callback);

        if err != ErrorCode::Success {
            return Err(err);
        }
        let (err, address) = create_and_store_policy_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
        if err != ErrorCode::Success {
            return Err(err);
        }
        Ok(address)
    }

    pub fn get_policy_from_wallet(wallet_handle: i32, policy_address: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();
        let cb = Box::new(move |err, policy| {
            sender.send((err, policy)).unwrap();
        });
        let (command_handle, callback) = CallbackUtils::closure_to_get_policy_cb(cb);

        let policy_address = CString::new(policy_address).unwrap();

        let err = indy_get_policy(command_handle,
                                            wallet_handle,
                                  policy_address.as_ptr(),
                                            callback);

        if err != ErrorCode::Success {
            return Err(err);
        }
        let (err, policy) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
        if err != ErrorCode::Success {
            return Err(err);
        }
        Ok(policy)
    }

    pub fn add_agent_to_policy_in_wallet(wallet_handle: i32, policy_address: &str,
                                         key_json: Option<&str>, master_secret_name: Option<&str>) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();
        let cb = Box::new(move |err, agent_verkey| {
            sender.send((err, agent_verkey)).unwrap();
        });
        let (command_handle, callback) = CallbackUtils::closure_to_add_agent_to_policy_in_wallet_cb(cb);

        let policy_address = CString::new(policy_address).unwrap();
        /*let key_json = CString::new(key_json).unwrap();
        let master_secret_name = CString::new(master_secret_name).unwrap();*/

        let key_json_str = key_json.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("{}").unwrap());
        let master_secret_name_str = master_secret_name.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err = indy_add_new_agent_to_policy(command_handle,
                                  wallet_handle,
                                  policy_address.as_ptr(),
                                               /*key_json.as_ptr(),
                                               master_secret_name.as_ptr(),*/
                                               if key_json.is_some() { key_json_str.as_ptr() } else { null() },
                                               if master_secret_name.is_some() { master_secret_name_str.as_ptr() } else { null() },
                                  callback);

        if err != ErrorCode::Success {
            return Err(err);
        }
        let (err, agent_verkey) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
        if err != ErrorCode::Success {
            return Err(err);
        }
        Ok(agent_verkey)
    }
}