use cosmos_sdk_proto::cosmos::authz::v1beta1::{MsgExec,QueryGrantsRequest,QueryGrantsResponse};
use cosmos_sdk_proto::cosmos::bank::v1beta1::MsgSend;
use cosmos_sdk_proto::cosmos::base::v1beta1::Coin;
use cosmos_sdk_proto::cosmos::staking::v1beta1::MsgDelegate;
use cosmos_sdk_proto::ibc::applications::transfer::v1::{
    QueryDenomTraceRequest, QueryDenomTraceResponse,
};
use cosmos_sdk_proto::traits::Message;
use cosmos_sdk_proto::traits::MessageExt;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Api, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,QueryRequest,
};
use cw2::set_contract_version;
use std::io::Cursor;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::msg::QueryMsg::QueryGranter;
use crate::state::{Config, CONFIG};
// Get the protobuf file we care about
// include!("protos/mod.rs");

// Version info for migration (boilerplate stuff)
const CONTRACT_NAME: &str = "crates.io:authz-demo";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Taken from the cw-plus crate's cw1-whitelist
fn map_validate(api: &dyn Api, admins: &[String]) -> StdResult<Vec<Addr>> {
    admins.iter().map(|addr| api.addr_validate(addr)).collect()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Validate that they sent us good addresses
    let config = Config {
        granter: info.sender,
        allowed: map_validate(deps.api, &msg.allowed)?,
    };

    // This sets the version, imported from cw2, just a normal thing to do
    // Boilerplate, don't worry about it
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("contract", CONTRACT_NAME)
        .add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let contract_address = env.contract.address.to_string();
    match msg {
        ExecuteMsg::TransferAuthFunds {
            to_address,
            granter_address,
            denom,
            amount,
        } => execute_transfer(
            deps,
            info,
            to_address,
            granter_address,
            contract_address,
            denom,
            amount,
        ),
    }
}


pub fn execute_transfer(
    deps: DepsMut,
    _info: MessageInfo,
    to_address: String,
    granter_address: String,
    contract_address: String,
    denom: String,
    amount: String,
) -> Result<Response, ContractError> {
    deps.api.addr_validate(&to_address)?;
    deps.api.addr_validate(&granter_address)?;
    deps.api.addr_validate(&contract_address)?;

    let mut send = MsgSend {
        to_address: to_address.to_owned(),
        from_address: granter_address,
        amount: vec![Coin {
            denom: denom.to_string(),
            amount: amount,
        }],
    };

    let exec = MsgExec {
        msgs: vec![send.to_any().unwrap()],
        grantee: contract_address,
    }
    .encode_to_vec();

    let msg = CosmosMsg::Stargate {
        type_url: "/cosmos.authz.v1beta1.MsgExec".to_string(),
        value: Binary::from(exec),
    };
    Ok(Response::new()
        .add_attribute("contract", "authz_demo")
        .add_attribute("method", "execute_transfer")
        .add_message(msg))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, queryMsg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::QueryGranter { granter } => {
            println!("The granter is {}", granter);
            let data = QueryGrantsRequest {
                granter: granter.to_string(),
                grantee: contract_address.to_string(),
                msg_type_url: "".to_string(),
                pagination: None,
            }.encode_to_vec();;
            let query = QueryRequest::Stargate {
                path: "/cosmos.authz.v1beta1.Query/Grants".to_string(),
                data: Binary::from(data),
            };

            let bin: Binary = deps.querier.query(&query)?;
            let response = QueryGrantsRequest::decode(&mut Cursor::new(bin.to_vec()))
                .map_err(ContractError::Decode)?;
            match response.denom_trace {
                None => Ok(to_binary("not_found")?),
                Some(trace) => Ok(to_binary(&trace.base_denom)?),
            }
        },
    }
}
