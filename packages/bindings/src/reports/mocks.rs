//! Contains some useful mocks of the Desmos x/reports module's types made to be used in any test.

use crate::reports::models::{Reason, Report, ReportTarget};
use crate::reports::models_query::{
    QueryReasonResponse, QueryReasonsResponse, QueryReportResponse, QueryReportsResponse,
};
use crate::reports::query::ReportsQuery;
use cosmwasm_std::{to_binary, Addr, Binary, ContractResult, Uint64};

/// Struct that contains some utility methods to mock data of the Desmos
/// x/reports module.
pub struct MockReportsQueries {}

impl MockReportsQueries {
    /// Functions that generate a mocked list of reports present in a subspace.
    pub fn get_mocked_reports(subspace_id: &Uint64) -> Vec<Report> {
        vec![
            Report {
                subspace_id: *subspace_id,
                id: Uint64::new(0),
                reasons_ids: vec![0, 2],
                message: None,
                reporter: Addr::unchecked("desmos1rfv0f7mx7w9d3jv3h803u38vqym9ygg344asm3"),
                target: ReportTarget::User {
                    user: Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
                }
                .into(),
                creation_date: "".to_string(),
            },
            Report {
                subspace_id: *subspace_id,
                id: Uint64::new(0),
                reasons_ids: vec![],
                message: Some("Report text".to_string()),
                reporter: Addr::unchecked("desmos1rfv0f7mx7w9d3jv3h803u38vqym9ygg344asm3"),
                target: ReportTarget::Post {
                    post_id: Uint64::new(42),
                }
                .into(),
                creation_date: "".to_string(),
            },
        ]
    }

    /// Functions that generate a mocked report present in a subspace.
    pub fn get_mocked_report(subspace_id: &Uint64) -> Report {
        Report {
            subspace_id: *subspace_id,
            id: Uint64::new(0),
            reasons_ids: vec![0, 2],
            message: None,
            reporter: Addr::unchecked("desmos1rfv0f7mx7w9d3jv3h803u38vqym9ygg344asm3"),
            target: ReportTarget::User {
                user: Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
            }
            .into(),
            creation_date: "".to_string(),
        }
    }

    /// Functions that generate a mocked list of report reasons present in a subspace.
    pub fn get_mocked_reasons(subspace_id: &Uint64) -> Vec<Reason> {
        vec![
            Reason {
                subspace_id: *subspace_id,
                id: 1,
                title: "Mock reason 1".to_string(),
                description: None,
            },
            Reason {
                subspace_id: *subspace_id,
                id: 2,
                title: "Mock reason 2".to_string(),
                description: Some("Reason description".to_string()),
            },
        ]
    }

    /// Functions that generate a mocked report reason present in a subspace.
    pub fn get_mocked_reason(subspace_id: &Uint64) -> Reason {
        Reason {
            subspace_id: *subspace_id,
            id: 1,
            title: "Mock reason 1".to_string(),
            description: None,
        }
    }
}

/// Functions that mocks the reports query responses.
pub fn mock_reports_query_response(query: &ReportsQuery) -> ContractResult<Binary> {
    let response = match query {
        ReportsQuery::Reports { subspace_id, .. } => to_binary(&QueryReportsResponse {
            reports: MockReportsQueries::get_mocked_reports(subspace_id),
            pagination: None,
        }),
        ReportsQuery::Report { subspace_id, .. } => to_binary(&QueryReportResponse {
            report: MockReportsQueries::get_mocked_report(subspace_id),
        }),
        ReportsQuery::Reasons { subspace_id, .. } => to_binary(&QueryReasonsResponse {
            reasons: MockReportsQueries::get_mocked_reasons(subspace_id),
            pagination: None,
        }),
        ReportsQuery::Reason { subspace_id, .. } => to_binary(&QueryReasonResponse {
            reason: MockReportsQueries::get_mocked_reason(subspace_id),
        }),
    };
    response.into()
}
