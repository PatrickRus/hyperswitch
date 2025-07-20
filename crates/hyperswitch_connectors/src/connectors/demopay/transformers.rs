// use common_enums::enums;
// use common_utils::types::StringMinorUnit;
// use error_stack::ResultExt;
// use hyperswitch_domain_models::{
//     payment_method_data::PaymentMethodData,
//     router_data::{ConnectorAuthType, RouterData},
//     router_flow_types::refunds::{Execute, RSync},
//     router_request_types::ResponseId,
//     router_response_types::{PaymentsResponseData, RefundsResponseData},
//     types::{PaymentsAuthorizeRouterData, PaymentsCaptureRouterData, RefundsRouterData},
// };
// use hyperswitch_interfaces::errors;
// use masking::Secret;
// use serde::{Deserialize, Serialize};

// use crate::{
//     types::{RefundsResponseRouterData, ResponseRouterData},
//     utils::{PaymentsAuthorizeRequestData, WalletData},
// };

// pub struct DemopayRouterData<T> {
//     pub amount: StringMinorUnit,
//     pub router_data: T,
// }

// impl<T> From<(StringMinorUnit, T)> for DemopayRouterData<T> {
//     fn from((amount, item): (StringMinorUnit, T)) -> Self {
//         Self {
//             amount,
//             router_data: item,
//         }
//     }
// }

// #[derive(Default, Debug, Serialize, PartialEq)]
// pub struct DemopayPaymentsRequest {
//     amount: StringMinorUnit,
//     wallet_id: Secret<String>,
//     auto_capture: bool,
// }

// impl TryFrom<&DemopayRouterData<&PaymentsAuthorizeRouterData>> for DemopayPaymentsRequest {
//     type Error = error_stack::Report<errors::ConnectorError>;
//     fn try_from(
//         item: &DemopayRouterData<&PaymentsAuthorizeRouterData>,
//     ) -> Result<Self, Self::Error> {
//         let wallet_data = match item.router_data.request.payment_method_data.clone() {
//             PaymentMethodData::Wallet(wallet_data) => Ok(wallet_data),
//             _ => Err(errors::ConnectorError::NotImplemented(
//                 "Payment method".to_string(),
//             )
//             .into()),
//         }?;

//         let wallet_id_value: serde_json::Value =
//             wallet_data.get_wallet_token_as_json("demopay".to_string())?;

//         let wallet_id = wallet_id_value
//             .get("wallet_id")
//             .and_then(|id| id.as_str())
//             .ok_or(errors::ConnectorError::MissingRequiredField {
//                 field_name: "wallet_id",
//             })?
//             .to_string();

//         Ok(Self {
//             amount: item.amount.clone(),
//             wallet_id: Secret::new(wallet_id),
//             auto_capture: item.router_data.request.is_auto_capture()?,
//         })
//     }
// }

// pub struct DemopayAuthType {
//     pub(super) api_key: Secret<String>,
// }

// impl TryFrom<&ConnectorAuthType> for DemopayAuthType {
//     type Error = error_stack::Report<errors::ConnectorError>;
//     fn try_from(auth_type: &ConnectorAuthType) -> Result<Self, Self::Error> {
//         match auth_type {
//             ConnectorAuthType::HeaderKey { api_key } => Ok(Self {
//                 api_key: api_key.to_owned(),
//             }),
//             _ => Err(errors::ConnectorError::FailedToObtainAuthType.into()),
//         }
//     }
// }

// #[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
// #[serde(rename_all = "snake_case")]
// pub enum DemopayPaymentStatus {
//     Succeeded,
//     Failed,
//     #[default]
//     Processing,
//     Authorized,
// }

// impl From<DemopayPaymentStatus> for common_enums::AttemptStatus {
//     fn from(item: DemopayPaymentStatus) -> Self {
//         match item {
//             DemopayPaymentStatus::Succeeded => Self::Charged,
//             DemopayPaymentStatus::Failed => Self::Failure,
//             DemopayPaymentStatus::Processing => Self::Authorizing,
//             DemopayPaymentStatus::Authorized => Self::Authorized,
//         }
//     }
// }

// #[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct DemopayPaymentsResponse {
//     status: DemopayPaymentStatus,
//     id: String,
// }

// impl<F, T> TryFrom<ResponseRouterData<F, DemopayPaymentsResponse, T, PaymentsResponseData>>
//     for RouterData<F, T, PaymentsResponseData>
// {
//     type Error = error_stack::Report<errors::ConnectorError>;
//     fn try_from(
//         item: ResponseRouterData<F, DemopayPaymentsResponse, T, PaymentsResponseData>,
//     ) -> Result<Self, Self::Error> {
//         Ok(Self {
//             status: common_enums::AttemptStatus::from(item.response.status),
//             response: Ok(PaymentsResponseData::TransactionResponse {
//                 resource_id: ResponseId::ConnectorTransactionId(item.response.id),
//                 redirection_data: Box::new(None),
//                 mandate_reference: Box::new(None),
//                 connector_metadata: None,
//                 network_txn_id: None,
//                 connector_response_reference_id: None,
//                 incremental_authorization_allowed: None,
//                 charges: None,
//             }),
//             ..item.data
//         })
//     }
// }

// #[derive(Default, Debug, Serialize)]
// pub struct DemopayCaptureRequest {
//     pub transaction_id: String,
// }

// impl TryFrom<&PaymentsCaptureRouterData> for DemopayCaptureRequest {
//     type Error = error_stack::Report<errors::ConnectorError>;
//     fn try_from(item: &PaymentsCaptureRouterData) -> Result<Self, Self::Error> {
//         Ok(Self {
//             transaction_id: item.request.connector_transaction_id.clone(),
//         })
//     }
// }

// #[derive(Default, Debug, Serialize)]
// pub struct DemopayRefundRequest {
//     pub amount: StringMinorUnit,
//     pub transaction_id: String,
// }

// impl<F> TryFrom<&DemopayRouterData<&RefundsRouterData<F>>> for DemopayRefundRequest {
//     type Error = error_stack::Report<errors::ConnectorError>;
//     fn try_from(item: &DemopayRouterData<&RefundsRouterData<F>>) -> Result<Self, Self::Error> {
//         Ok(Self {
//             amount: item.amount.to_owned(),
//             transaction_id: item.router_data.request.connector_transaction_id.clone(),
//         })
//     }
// }

// #[allow(dead_code)]
// #[derive(Debug, Serialize, Default, Deserialize, Clone)]
// #[serde(rename_all = "snake_case")]
// pub enum RefundStatus {
//     Succeeded,
//     Failed,
//     #[default]
//     Processing,
// }

// impl From<RefundStatus> for enums::RefundStatus {
//     fn from(item: RefundStatus) -> Self {
//         match item {
//             RefundStatus::Succeeded => Self::Success,
//             RefundStatus::Failed => Self::Failure,
//             RefundStatus::Processing => Self::Pending,
//         }
//     }
// }

// #[derive(Default, Debug, Clone, Serialize, Deserialize)]
// pub struct RefundResponse {
//     id: String,
//     status: RefundStatus,
// }

// impl TryFrom<RefundsResponseRouterData<Execute, RefundResponse>> for RefundsRouterData<Execute> {
//     type Error = error_stack::Report<errors::ConnectorError>;
//     fn try_from(
//         item: RefundsResponseRouterData<Execute, RefundResponse>,
//     ) -> Result<Self, Self::Error> {
//         Ok(Self {
//             response: Ok(RefundsResponseData {
//                 connector_refund_id: item.response.id.to_string(),
//                 refund_status: enums::RefundStatus::from(item.response.status),
//             }),
//             ..item.data
//         })
//     }
// }

// impl TryFrom<RefundsResponseRouterData<RSync, RefundResponse>> for RefundsRouterData<RSync> {
//     type Error = error_stack::Report<errors::ConnectorError>;
//     fn try_from(
//         item: RefundsResponseRouterData<RSync, RefundResponse>,
//     ) -> Result<Self, Self::Error> {
//         Ok(Self {
//             response: Ok(RefundsResponseData {
//                 connector_refund_id: item.response.id.to_string(),
//                 refund_status: enums::RefundStatus::from(item.response.status),
//             }),
//             ..item.data
//         })
//     }
// }

// #[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
// pub struct DemopayErrorResponse {
//     pub status_code: u16,
//     pub code: String,
//     pub message: String,
//     pub reason: Option<String>,
// }
use common_enums::enums;
use common_utils::types::StringMinorUnit;
use hyperswitch_domain_models::{
    payment_method_data::PaymentMethodData,
    router_data::{ConnectorAuthType, RouterData},
    router_flow_types::refunds::{Execute, RSync},
    router_request_types::ResponseId,
    router_response_types::{PaymentsResponseData, RefundsResponseData},
    types::{PaymentsAuthorizeRouterData, PaymentsCaptureRouterData, RefundsRouterData},
};
use hyperswitch_interfaces::errors;
use masking::Secret;
use serde::{Deserialize, Serialize};

use crate::{
    types::{RefundsResponseRouterData, ResponseRouterData},
    utils::{PaymentsAuthorizeRequestData, WalletData},
};

pub struct DemopayRouterData<T> {
    pub amount: StringMinorUnit,
    pub router_data: T,
}

impl<T> From<(StringMinorUnit, T)> for DemopayRouterData<T> {
    fn from((amount, item): (StringMinorUnit, T)) -> Self {
        Self {
            amount,
            router_data: item,
        }
    }
}

#[derive(Default, Debug, Serialize, PartialEq)]
pub struct DemopayPaymentsRequest {
    amount: StringMinorUnit,
    wallet_id: Secret<String>,
    auto_capture: bool,
}

impl TryFrom<&DemopayRouterData<&PaymentsAuthorizeRouterData>> for DemopayPaymentsRequest {
    type Error = error_stack::Report<errors::ConnectorError>;

    fn try_from(
        item: &DemopayRouterData<&PaymentsAuthorizeRouterData>,
    ) -> Result<Self, Self::Error> {
        let wallet_data = match &item.router_data.request.payment_method_data {
            PaymentMethodData::Wallet(wallet_data) => wallet_data.clone(),
            _ => {
                return Err(error_stack::report!(
                    errors::ConnectorError::NotImplemented("Payment method".to_string())
                ));
            }
        };

        let wallet_id_value: serde_json::Value =
            wallet_data.get_wallet_token_as_json("demopay".to_string())?;

        let wallet_id = wallet_id_value
            .get("wallet_id")
            .and_then(|id| id.as_str())
            .ok_or(error_stack::report!(
                errors::ConnectorError::MissingRequiredField {
                    field_name: "wallet_id",
                }
            ))?
            .to_string();

        Ok(Self {
            amount: item.amount.clone(),
            wallet_id: Secret::new(wallet_id),
            auto_capture: item.router_data.request.is_auto_capture()?,
        })
    }
}

pub struct DemopayAuthType {
    pub(super) api_key: Secret<String>,
}

impl TryFrom<&ConnectorAuthType> for DemopayAuthType {
    type Error = error_stack::Report<errors::ConnectorError>;

    fn try_from(auth_type: &ConnectorAuthType) -> Result<Self, Self::Error> {
        match auth_type {
            ConnectorAuthType::HeaderKey { api_key } => Ok(Self {
                api_key: api_key.to_owned(),
            }),
            _ => Err(error_stack::report!(
                errors::ConnectorError::FailedToObtainAuthType
            )),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DemopayPaymentStatus {
    Succeeded,
    Failed,
    #[default]
    Processing,
    Authorized,
}

impl From<DemopayPaymentStatus> for common_enums::AttemptStatus {
    fn from(item: DemopayPaymentStatus) -> Self {
        match item {
            DemopayPaymentStatus::Succeeded => Self::Charged,
            DemopayPaymentStatus::Failed => Self::Failure,
            DemopayPaymentStatus::Processing => Self::Authorizing,
            DemopayPaymentStatus::Authorized => Self::Authorized,
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DemopayPaymentsResponse {
    status: DemopayPaymentStatus,
    id: String,
}

impl<F, T> TryFrom<ResponseRouterData<F, DemopayPaymentsResponse, T, PaymentsResponseData>>
    for RouterData<F, T, PaymentsResponseData>
{
    type Error = error_stack::Report<errors::ConnectorError>;

    fn try_from(
        item: ResponseRouterData<F, DemopayPaymentsResponse, T, PaymentsResponseData>,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            status: common_enums::AttemptStatus::from(item.response.status),
            response: Ok(PaymentsResponseData::TransactionResponse {
                resource_id: ResponseId::ConnectorTransactionId(item.response.id),
                redirection_data: Box::new(None),
                mandate_reference: Box::new(None),
                connector_metadata: None,
                network_txn_id: None,
                connector_response_reference_id: None,
                incremental_authorization_allowed: None,
                charges: None,
            }),
            ..item.data
        })
    }
}

#[derive(Default, Debug, Serialize)]
pub struct DemopayCaptureRequest {
    pub transaction_id: String,
}

impl TryFrom<&PaymentsCaptureRouterData> for DemopayCaptureRequest {
    type Error = error_stack::Report<errors::ConnectorError>;

    fn try_from(item: &PaymentsCaptureRouterData) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction_id: item.request.connector_transaction_id.clone(),
        })
    }
}

#[derive(Default, Debug, Serialize)]
pub struct DemopayRefundRequest {
    pub amount: StringMinorUnit,
    pub transaction_id: String,
}

impl<F> TryFrom<&DemopayRouterData<&RefundsRouterData<F>>> for DemopayRefundRequest {
    type Error = error_stack::Report<errors::ConnectorError>;

    fn try_from(item: &DemopayRouterData<&RefundsRouterData<F>>) -> Result<Self, Self::Error> {
        Ok(Self {
            amount: item.amount.to_owned(),
            transaction_id: item.router_data.request.connector_transaction_id.clone(),
        })
    }
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Default, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RefundStatus {
    Succeeded,
    Failed,
    #[default]
    Processing,
}

impl From<RefundStatus> for enums::RefundStatus {
    fn from(item: RefundStatus) -> Self {
        match item {
            RefundStatus::Succeeded => Self::Success,
            RefundStatus::Failed => Self::Failure,
            RefundStatus::Processing => Self::Pending,
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RefundResponse {
    id: String,
    status: RefundStatus,
}

impl TryFrom<RefundsResponseRouterData<Execute, RefundResponse>>
    for RefundsRouterData<Execute>
{
    type Error = error_stack::Report<errors::ConnectorError>;

    fn try_from(
        item: RefundsResponseRouterData<Execute, RefundResponse>,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            response: Ok(RefundsResponseData {
                connector_refund_id: item.response.id.to_string(),
                refund_status: enums::RefundStatus::from(item.response.status),
            }),
            ..item.data
        })
    }
}

impl TryFrom<RefundsResponseRouterData<RSync, RefundResponse>> for RefundsRouterData<RSync> {
    type Error = error_stack::Report<errors::ConnectorError>;

    fn try_from(
        item: RefundsResponseRouterData<RSync, RefundResponse>,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            response: Ok(RefundsResponseData {
                connector_refund_id: item.response.id.to_string(),
                refund_status: enums::RefundStatus::from(item.response.status),
            }),
            ..item.data
        })
    }
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct DemopayErrorResponse {
    pub status_code: u16,
    pub code: String,
    pub message: String,
    pub reason: Option<String>,
}
