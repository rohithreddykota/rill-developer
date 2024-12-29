/*
 * rill/runtime/v1/api.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: version not set
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1Query {
    #[serde(rename = "metricsViewAggregationRequest", skip_serializing_if = "Option::is_none")]
    pub metrics_view_aggregation_request: Option<Box<models::V1MetricsViewAggregationRequest>>,
    #[serde(rename = "metricsViewToplistRequest", skip_serializing_if = "Option::is_none")]
    pub metrics_view_toplist_request: Option<Box<models::V1MetricsViewToplistRequest>>,
    #[serde(rename = "metricsViewComparisonRequest", skip_serializing_if = "Option::is_none")]
    pub metrics_view_comparison_request: Option<Box<models::V1MetricsViewComparisonRequest>>,
    #[serde(rename = "metricsViewTimeSeriesRequest", skip_serializing_if = "Option::is_none")]
    pub metrics_view_time_series_request: Option<Box<models::V1MetricsViewTimeSeriesRequest>>,
    #[serde(rename = "metricsViewTotalsRequest", skip_serializing_if = "Option::is_none")]
    pub metrics_view_totals_request: Option<Box<models::V1MetricsViewTotalsRequest>>,
    #[serde(rename = "metricsViewRowsRequest", skip_serializing_if = "Option::is_none")]
    pub metrics_view_rows_request: Option<Box<models::V1MetricsViewRowsRequest>>,
    #[serde(rename = "columnRollupIntervalRequest", skip_serializing_if = "Option::is_none")]
    pub column_rollup_interval_request: Option<Box<models::V1ColumnRollupIntervalRequest>>,
    #[serde(rename = "columnTopKRequest", skip_serializing_if = "Option::is_none")]
    pub column_top_k_request: Option<Box<models::V1ColumnTopKRequest>>,
    #[serde(rename = "columnNullCountRequest", skip_serializing_if = "Option::is_none")]
    pub column_null_count_request: Option<Box<models::V1ColumnNullCountRequest>>,
    #[serde(rename = "columnDescriptiveStatisticsRequest", skip_serializing_if = "Option::is_none")]
    pub column_descriptive_statistics_request: Option<Box<models::V1ColumnDescriptiveStatisticsRequest>>,
    #[serde(rename = "columnTimeGrainRequest", skip_serializing_if = "Option::is_none")]
    pub column_time_grain_request: Option<Box<models::V1ColumnTimeGrainRequest>>,
    #[serde(rename = "columnNumericHistogramRequest", skip_serializing_if = "Option::is_none")]
    pub column_numeric_histogram_request: Option<Box<models::V1ColumnNumericHistogramRequest>>,
    #[serde(rename = "columnRugHistogramRequest", skip_serializing_if = "Option::is_none")]
    pub column_rug_histogram_request: Option<Box<models::V1ColumnRugHistogramRequest>>,
    #[serde(rename = "columnTimeRangeRequest", skip_serializing_if = "Option::is_none")]
    pub column_time_range_request: Option<Box<models::V1ColumnTimeRangeRequest>>,
    #[serde(rename = "columnCardinalityRequest", skip_serializing_if = "Option::is_none")]
    pub column_cardinality_request: Option<Box<models::V1ColumnCardinalityRequest>>,
    #[serde(rename = "columnTimeSeriesRequest", skip_serializing_if = "Option::is_none")]
    pub column_time_series_request: Option<Box<models::V1ColumnTimeSeriesRequest>>,
    #[serde(rename = "tableCardinalityRequest", skip_serializing_if = "Option::is_none")]
    pub table_cardinality_request: Option<Box<models::V1TableCardinalityRequest>>,
    #[serde(rename = "tableColumnsRequest", skip_serializing_if = "Option::is_none")]
    pub table_columns_request: Option<Box<models::V1TableColumnsRequest>>,
    #[serde(rename = "tableRowsRequest", skip_serializing_if = "Option::is_none")]
    pub table_rows_request: Option<Box<models::V1TableRowsRequest>>,
}

impl V1Query {
    pub fn new() -> V1Query {
        V1Query {
            metrics_view_aggregation_request: None,
            metrics_view_toplist_request: None,
            metrics_view_comparison_request: None,
            metrics_view_time_series_request: None,
            metrics_view_totals_request: None,
            metrics_view_rows_request: None,
            column_rollup_interval_request: None,
            column_top_k_request: None,
            column_null_count_request: None,
            column_descriptive_statistics_request: None,
            column_time_grain_request: None,
            column_numeric_histogram_request: None,
            column_rug_histogram_request: None,
            column_time_range_request: None,
            column_cardinality_request: None,
            column_time_series_request: None,
            table_cardinality_request: None,
            table_columns_request: None,
            table_rows_request: None,
        }
    }
}
