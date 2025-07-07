/***
 * Copyright (c) 2025 Rafael Fernández López <ereslibre@curried.software>
 *
 * Permission is hereby granted, free of charge, to any person
 * obtaining a copy of this software and associated documentation
 * files (the "Software"), to deal in the Software without
 * restriction, including without limitation the rights to use, copy,
 * modify, merge, publish, distribute, sublicense, and/or sell copies
 * of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be
 * included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 ***/

use crate::{
    client::{Body, Client, Environment, HttpMethod},
    errors::ApiResult,
    merchant::client::MerchantAuthentication,
};

pub mod unversioned {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, strum::Display, Serialize)]
    #[serde(tag = "type", rename_all = "snake_case")]
    #[strum(serialize_all = "snake_case")]
    pub enum ReportRunRequest {
        SettlementReport(SettlementReport),
        CustomReport(CustomReport),
        PayoutStatementReport(PayoutStatementReport),
        IcppFeeBreakdownReport,
    }

    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct SettlementReport {
        pub filter: SettlementReportFilter,
        pub format: String,
        pub r#type: String,
        pub options: Option<SettlementReportOptions>,
    }

    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct SettlementReportFilter {
        pub from: Option<String>,
        pub to: String,
        pub entity_types: Option<String>,
        pub entity_states: Option<String>,
        pub currency: Option<String>,
        pub location_id: Option<String>,
    }

    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct SettlementReportOptions {
        pub timezone: Option<String>,
        pub columns: Option<String>,
    }

    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct CustomReport {
        pub filter: CustomReportFilter,
        pub format: String,
        pub r#type: String,
        pub options: Option<CustomReportOptions>,
    }

    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct CustomReportFilter {
        pub from: Option<String>,
        pub to: String,
        pub entity_types: Option<String>,
        pub entity_states: Option<String>,
        pub currency: Option<String>,
        pub location_id: Option<String>,
    }

    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct CustomReportOptions {
        pub timezone: Option<String>,
        pub columns: Option<String>,
    }

    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct PayoutStatementReport {
        pub filter: PayoutStatementFilter,
        pub format: String,
        pub r#type: String,
        pub options: Option<PayoutStatementReportOptions>,
    }

    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct PayoutStatementFilter {
        pub payout_id: String,
    }

    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct PayoutStatementReportOptions {
        pub timezone: Option<String>,
        pub columns: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct ReportRun {
        pub report_run_id: String,
        pub status: ReportRunStatus,
        pub file_url: Option<String>,
    }

    #[derive(Debug, Deserialize, strum::Display, Serialize)]
    #[serde(tag = "type", rename_all = "snake_case")]
    #[strum(serialize_all = "snake_case")]
    pub enum ReportRunStatus {
        #[serde(alias = "PROCESSING")]
        Processing,
        #[serde(alias = "COMPLETED")]
        Completed,
        #[serde(alias = "FAILED")]
        Failed,
        #[serde(alias = "EXPIRED")]
        Expired,
    }
}

pub async fn create<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    order: &unversioned::ReportRunRequest,
) -> ApiResult<unversioned::ReportRun> {
    client
        .request(
            HttpMethod::Post {
                body: Some(Body::Json(&order)),
            },
            &client.environment.unversioned_uri("/orders"),
        )
        .await
}

pub async fn retrieve<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    report_run_id: &str,
) -> ApiResult<unversioned::ReportRun> {
    client
        .request(
            HttpMethod::Get::<()>,
            &client
                .environment
                .unversioned_uri(&format!("/report-runs/{report_run_id}")),
        )
        .await
}

pub async fn download<E: Environment>(
    client: &Client<E, MerchantAuthentication>,
    report_run_id: &str,
) -> ApiResult<Vec<u8>> {
    client
        .request(
            HttpMethod::Get::<()>,
            &client
                .environment
                .unversioned_uri(&format!("/report-runs/{report_run_id}/file")),
        )
        .await
}
