# Rust API client for runtime

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: version not set
- Package version: 0.1.0
- Generator version: 7.10.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `runtime` and add the following to `Cargo.toml` under `[dependencies]`:

```
runtime = { path = "./runtime" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*ConnectorServiceApi* | [**connector_service_big_query_list_datasets**](docs/ConnectorServiceApi.md#connector_service_big_query_list_datasets) | **GET** /v1/bigquery/datasets | BigQueryListDatasets list all datasets in a bigquery project
*ConnectorServiceApi* | [**connector_service_big_query_list_tables**](docs/ConnectorServiceApi.md#connector_service_big_query_list_tables) | **GET** /v1/bigquery/tables | BigQueryListTables list all tables in a bigquery project:dataset
*ConnectorServiceApi* | [**connector_service_gcs_get_credentials_info**](docs/ConnectorServiceApi.md#connector_service_gcs_get_credentials_info) | **GET** /v1/gcs/credentials_info | GCSGetCredentialsInfo returns metadata for the given bucket.
*ConnectorServiceApi* | [**connector_service_gcs_list_buckets**](docs/ConnectorServiceApi.md#connector_service_gcs_list_buckets) | **GET** /v1/gcs/buckets | GCSListBuckets lists buckets accessible with the configured credentials.
*ConnectorServiceApi* | [**connector_service_gcs_list_objects**](docs/ConnectorServiceApi.md#connector_service_gcs_list_objects) | **GET** /v1/gcs/bucket/{bucket}/objects | GCSListObjects lists objects for the given bucket.
*ConnectorServiceApi* | [**connector_service_olap_get_table**](docs/ConnectorServiceApi.md#connector_service_olap_get_table) | **GET** /v1/connectors/olap/table | OLAPGetTable returns metadata about a table or view in an OLAP
*ConnectorServiceApi* | [**connector_service_olap_list_tables**](docs/ConnectorServiceApi.md#connector_service_olap_list_tables) | **GET** /v1/olap/tables | OLAPListTables list all tables across all databases on motherduck
*ConnectorServiceApi* | [**connector_service_s3_get_bucket_metadata**](docs/ConnectorServiceApi.md#connector_service_s3_get_bucket_metadata) | **GET** /v1/s3/bucket/{bucket}/metadata | S3GetBucketMetadata returns metadata for the given bucket.
*ConnectorServiceApi* | [**connector_service_s3_get_credentials_info**](docs/ConnectorServiceApi.md#connector_service_s3_get_credentials_info) | **GET** /v1/s3/credentials_info | S3GetCredentialsInfo returns metadata for the given bucket.
*ConnectorServiceApi* | [**connector_service_s3_list_buckets**](docs/ConnectorServiceApi.md#connector_service_s3_list_buckets) | **GET** /v1/s3/buckets | S3ListBuckets lists buckets accessible with the configured credentials.
*ConnectorServiceApi* | [**connector_service_s3_list_objects**](docs/ConnectorServiceApi.md#connector_service_s3_list_objects) | **GET** /v1/s3/bucket/{bucket}/objects | S3ListBuckets lists objects for the given bucket.
*QueryServiceApi* | [**query_service_column_cardinality**](docs/QueryServiceApi.md#query_service_column_cardinality) | **GET** /v1/instances/{instanceId}/queries/column-cardinality/tables/{tableName} | Get cardinality for a column
*QueryServiceApi* | [**query_service_column_descriptive_statistics**](docs/QueryServiceApi.md#query_service_column_descriptive_statistics) | **GET** /v1/instances/{instanceId}/queries/descriptive-statistics/tables/{tableName} | Get basic stats for a numeric column like min, max, mean, stddev, etc
*QueryServiceApi* | [**query_service_column_null_count**](docs/QueryServiceApi.md#query_service_column_null_count) | **GET** /v1/instances/{instanceId}/queries/null-count/tables/{tableName} | Get the number of nulls in a column
*QueryServiceApi* | [**query_service_column_numeric_histogram**](docs/QueryServiceApi.md#query_service_column_numeric_histogram) | **GET** /v1/instances/{instanceId}/queries/numeric-histogram/tables/{tableName} | Get the histogram for values in a column
*QueryServiceApi* | [**query_service_column_rollup_interval**](docs/QueryServiceApi.md#query_service_column_rollup_interval) | **POST** /v1/instances/{instanceId}/queries/rollup-interval/tables/{tableName} | ColumnRollupInterval returns the minimum time granularity (as well as the time range) for a specified timestamp column
*QueryServiceApi* | [**query_service_column_rug_histogram**](docs/QueryServiceApi.md#query_service_column_rug_histogram) | **GET** /v1/instances/{instanceId}/queries/rug-histogram/tables/{tableName} | Get outliers for a numeric column
*QueryServiceApi* | [**query_service_column_time_grain**](docs/QueryServiceApi.md#query_service_column_time_grain) | **GET** /v1/instances/{instanceId}/queries/smallest-time-grain/tables/{tableName} | Estimates the smallest time grain present in the column
*QueryServiceApi* | [**query_service_column_time_range**](docs/QueryServiceApi.md#query_service_column_time_range) | **GET** /v1/instances/{instanceId}/queries/time-range-summary/tables/{tableName} | Get the time range summaries (min, max) for a column
*QueryServiceApi* | [**query_service_column_time_series**](docs/QueryServiceApi.md#query_service_column_time_series) | **POST** /v1/instances/{instanceId}/queries/timeseries/tables/{tableName} | Generate time series for the given measures (aggregation expressions) along with the sparkline timeseries
*QueryServiceApi* | [**query_service_column_top_k**](docs/QueryServiceApi.md#query_service_column_top_k) | **POST** /v1/instances/{instanceId}/queries/topk/tables/{tableName} | Get TopK elements from a table for a column given an agg function agg function and k are optional, defaults are count(*) and 50 respectively
*QueryServiceApi* | [**query_service_export**](docs/QueryServiceApi.md#query_service_export) | **POST** /v1/instances/{instanceId}/queries/export | Export builds a URL to download the results of a query as a file.
*QueryServiceApi* | [**query_service_export_report**](docs/QueryServiceApi.md#query_service_export_report) | **POST** /v1/instances/{instanceId}/reports/{report}/export | ExportReport builds a URL to download the results of a query as a file.
*QueryServiceApi* | [**query_service_metrics_view_aggregation**](docs/QueryServiceApi.md#query_service_metrics_view_aggregation) | **POST** /v1/instances/{instanceId}/queries/metrics-views/{metricsView}/aggregation | MetricsViewAggregation is a generic API for running group-by/pivot queries against a metrics view.
*QueryServiceApi* | [**query_service_metrics_view_comparison**](docs/QueryServiceApi.md#query_service_metrics_view_comparison) | **POST** /v1/instances/{instanceId}/queries/metrics-views/{metricsViewName}/compare-toplist | MetricsViewComparison returns a toplist containing comparison data of another toplist (same dimension/measure but a different time range). Returns a toplist without comparison data if comparison time range is omitted.
*QueryServiceApi* | [**query_service_metrics_view_rows**](docs/QueryServiceApi.md#query_service_metrics_view_rows) | **POST** /v1/instances/{instanceId}/queries/metrics-views/{metricsViewName}/rows | MetricsViewRows returns the underlying model rows matching a metrics view time range and filter(s).
*QueryServiceApi* | [**query_service_metrics_view_schema**](docs/QueryServiceApi.md#query_service_metrics_view_schema) | **GET** /v1/instances/{instanceId}/queries/metrics-views/{metricsViewName}/schema | MetricsViewSchema Get the data types of measures and dimensions
*QueryServiceApi* | [**query_service_metrics_view_search**](docs/QueryServiceApi.md#query_service_metrics_view_search) | **POST** /v1/instances/{instanceId}/queries/metrics-views/{metricsViewName}/search | MetricsViewSearch Get the data types of measures and dimensions
*QueryServiceApi* | [**query_service_metrics_view_time_range**](docs/QueryServiceApi.md#query_service_metrics_view_time_range) | **POST** /v1/instances/{instanceId}/queries/metrics-views/{metricsViewName}/time-range-summary | MetricsViewTimeRange Get the time range summaries (min, max) for time column in a metrics view
*QueryServiceApi* | [**query_service_metrics_view_time_series**](docs/QueryServiceApi.md#query_service_metrics_view_time_series) | **POST** /v1/instances/{instanceId}/queries/metrics-views/{metricsViewName}/timeseries | MetricsViewTimeSeries returns time series for the measures in the metrics view. It's a convenience API for querying a metrics view.
*QueryServiceApi* | [**query_service_metrics_view_toplist**](docs/QueryServiceApi.md#query_service_metrics_view_toplist) | **POST** /v1/instances/{instanceId}/queries/metrics-views/{metricsViewName}/toplist | Deprecated - use MetricsViewComparison instead. MetricsViewToplist returns the top dimension values of a metrics view sorted by one or more measures. It's a convenience API for querying a metrics view.
*QueryServiceApi* | [**query_service_metrics_view_totals**](docs/QueryServiceApi.md#query_service_metrics_view_totals) | **POST** /v1/instances/{instanceId}/queries/metrics-views/{metricsViewName}/totals | MetricsViewTotals returns totals over a time period for the measures in a metrics view. It's a convenience API for querying a metrics view.
*QueryServiceApi* | [**query_service_query**](docs/QueryServiceApi.md#query_service_query) | **POST** /v1/instances/{instanceId}/query | Query runs a SQL query against the instance's OLAP datastore.
*QueryServiceApi* | [**query_service_query_batch**](docs/QueryServiceApi.md#query_service_query_batch) | **POST** /v1/instances/{instanceId}/query/batch | Batch request with different queries
*QueryServiceApi* | [**query_service_resolve_component**](docs/QueryServiceApi.md#query_service_resolve_component) | **POST** /v1/instances/{instanceId}/queries/components/{component}/resolve | ResolveComponent resolves the data and renderer for a Component resource.
*QueryServiceApi* | [**query_service_table_cardinality**](docs/QueryServiceApi.md#query_service_table_cardinality) | **GET** /v1/instances/{instanceId}/queries/table-cardinality/tables/{tableName} | TableCardinality returns row count
*QueryServiceApi* | [**query_service_table_columns**](docs/QueryServiceApi.md#query_service_table_columns) | **POST** /v1/instances/{instanceId}/queries/columns-profile/tables/{tableName} | TableColumns returns column profiles
*QueryServiceApi* | [**query_service_table_rows**](docs/QueryServiceApi.md#query_service_table_rows) | **GET** /v1/instances/{instanceId}/queries/rows/tables/{tableName} | TableRows returns table rows
*RuntimeServiceApi* | [**runtime_service_analyze_connectors**](docs/RuntimeServiceApi.md#runtime_service_analyze_connectors) | **GET** /v1/instances/{instanceId}/connectors/analyze | AnalyzeConnectors scans all the project files and returns information about all referenced connectors.
*RuntimeServiceApi* | [**runtime_service_analyze_variables**](docs/RuntimeServiceApi.md#runtime_service_analyze_variables) | **GET** /v1/instances/{instanceId}/variables/analyze | AnalyzeVariables scans `Source`, `Model` and `Connector` resources in the catalog for use of an environment variable
*RuntimeServiceApi* | [**runtime_service_create_directory**](docs/RuntimeServiceApi.md#runtime_service_create_directory) | **POST** /v1/instances/{instanceId}/files/dir | CreateDirectory create a directory for the given path
*RuntimeServiceApi* | [**runtime_service_create_instance**](docs/RuntimeServiceApi.md#runtime_service_create_instance) | **POST** /v1/instances | CreateInstance creates a new instance
*RuntimeServiceApi* | [**runtime_service_create_trigger**](docs/RuntimeServiceApi.md#runtime_service_create_trigger) | **POST** /v1/instances/{instanceId}/trigger | CreateTrigger submits a refresh trigger, which will asynchronously refresh the specified resources. Triggers are ephemeral resources that will be cleaned up by the controller.
*RuntimeServiceApi* | [**runtime_service_delete_file**](docs/RuntimeServiceApi.md#runtime_service_delete_file) | **DELETE** /v1/instances/{instanceId}/files/entry | DeleteFile deletes a file from a repo
*RuntimeServiceApi* | [**runtime_service_delete_instance**](docs/RuntimeServiceApi.md#runtime_service_delete_instance) | **POST** /v1/instances/{instanceId} | DeleteInstance deletes an instance
*RuntimeServiceApi* | [**runtime_service_edit_instance**](docs/RuntimeServiceApi.md#runtime_service_edit_instance) | **PATCH** /v1/instances/{instanceId} | EditInstance edits an existing instance
*RuntimeServiceApi* | [**runtime_service_generate_metrics_view_file**](docs/RuntimeServiceApi.md#runtime_service_generate_metrics_view_file) | **POST** /v1/instances/{instanceId}/files/generate-metrics-view | GenerateMetricsViewFile generates a metrics view YAML file from a table in an OLAP database
*RuntimeServiceApi* | [**runtime_service_generate_renderer**](docs/RuntimeServiceApi.md#runtime_service_generate_renderer) | **POST** /v1/instances/{instanceId}/generate/renderer | GenerateRenderer generates a component renderer and renderer properties from a resolver and resolver properties
*RuntimeServiceApi* | [**runtime_service_generate_resolver**](docs/RuntimeServiceApi.md#runtime_service_generate_resolver) | **POST** /v1/instances/{instanceId}/generate/resolver | GenerateResolver generates resolver and resolver properties from a table or a metrics view
*RuntimeServiceApi* | [**runtime_service_get_explore**](docs/RuntimeServiceApi.md#runtime_service_get_explore) | **GET** /v1/instances/{instanceId}/resources/explore | GetExplore is a convenience RPC that combines looking up an Explore resource and its underlying MetricsView into one network call.
*RuntimeServiceApi* | [**runtime_service_get_file**](docs/RuntimeServiceApi.md#runtime_service_get_file) | **GET** /v1/instances/{instanceId}/files/entry | GetFile returns the contents of a specific file in a repo.
*RuntimeServiceApi* | [**runtime_service_get_instance**](docs/RuntimeServiceApi.md#runtime_service_get_instance) | **GET** /v1/instances/{instanceId} | GetInstance returns information about a specific instance
*RuntimeServiceApi* | [**runtime_service_get_logs**](docs/RuntimeServiceApi.md#runtime_service_get_logs) | **GET** /v1/instances/{instanceId}/logs | GetLogs returns recent logs from a controller
*RuntimeServiceApi* | [**runtime_service_get_model_partitions**](docs/RuntimeServiceApi.md#runtime_service_get_model_partitions) | **GET** /v1/instances/{instanceId}/models/{model}/partitions | GetModelPartitions returns the partitions of a model
*RuntimeServiceApi* | [**runtime_service_get_resource**](docs/RuntimeServiceApi.md#runtime_service_get_resource) | **GET** /v1/instances/{instanceId}/resource | GetResource looks up a specific catalog resource
*RuntimeServiceApi* | [**runtime_service_health**](docs/RuntimeServiceApi.md#runtime_service_health) | **GET** /v1/health | Health runs a health check on the runtime.
*RuntimeServiceApi* | [**runtime_service_instance_health**](docs/RuntimeServiceApi.md#runtime_service_instance_health) | **GET** /v1/health/instances/{instanceId} | InstanceHealth runs a health check on a specific instance.
*RuntimeServiceApi* | [**runtime_service_issue_dev_jwt**](docs/RuntimeServiceApi.md#runtime_service_issue_dev_jwt) | **POST** /v1/dev-jwt | IssueDevJWT issues a JWT for mimicking a user in local development.
*RuntimeServiceApi* | [**runtime_service_list_connector_drivers**](docs/RuntimeServiceApi.md#runtime_service_list_connector_drivers) | **GET** /v1/connectors/meta | ListConnectorDrivers returns a description of all the connector drivers registed in the runtime, including their configuration specs and the capabilities they support.
*RuntimeServiceApi* | [**runtime_service_list_examples**](docs/RuntimeServiceApi.md#runtime_service_list_examples) | **GET** /v1/examples | ListExamples lists all the examples embedded into binary
*RuntimeServiceApi* | [**runtime_service_list_files**](docs/RuntimeServiceApi.md#runtime_service_list_files) | **GET** /v1/instances/{instanceId}/files | ListFiles lists all the files matching a glob in a repo. The files are sorted by their full path.
*RuntimeServiceApi* | [**runtime_service_list_instances**](docs/RuntimeServiceApi.md#runtime_service_list_instances) | **GET** /v1/instances | ListInstances lists all the instances currently managed by the runtime
*RuntimeServiceApi* | [**runtime_service_list_notifier_connectors**](docs/RuntimeServiceApi.md#runtime_service_list_notifier_connectors) | **GET** /v1/instances/{instanceId}/connectors/notifiers | ListNotifierConnectors returns the names of all configured connectors that can be used as notifiers. This API is much faster than AnalyzeConnectors and can be called without admin-level permissions.
*RuntimeServiceApi* | [**runtime_service_list_resources**](docs/RuntimeServiceApi.md#runtime_service_list_resources) | **GET** /v1/instances/{instanceId}/resources | ListResources lists the resources stored in the catalog
*RuntimeServiceApi* | [**runtime_service_ping**](docs/RuntimeServiceApi.md#runtime_service_ping) | **GET** /v1/ping | Ping returns information about the runtime
*RuntimeServiceApi* | [**runtime_service_put_file**](docs/RuntimeServiceApi.md#runtime_service_put_file) | **POST** /v1/instances/{instanceId}/files/entry | PutFile creates or updates a file in a repo
*RuntimeServiceApi* | [**runtime_service_rename_file**](docs/RuntimeServiceApi.md#runtime_service_rename_file) | **POST** /v1/instances/{instanceId}/files/rename | RenameFile renames a file in a repo
*RuntimeServiceApi* | [**runtime_service_unpack_empty**](docs/RuntimeServiceApi.md#runtime_service_unpack_empty) | **POST** /v1/instances/{instanceId}/files/unpack-empty | UnpackEmpty unpacks an empty project
*RuntimeServiceApi* | [**runtime_service_unpack_example**](docs/RuntimeServiceApi.md#runtime_service_unpack_example) | **POST** /v1/instances/{instanceId}/files/unpack-example | UnpackExample unpacks an example project
*RuntimeServiceApi* | [**runtime_service_watch_files**](docs/RuntimeServiceApi.md#runtime_service_watch_files) | **GET** /v1/instances/{instanceId}/files/watch | WatchFiles streams repo file update events. It is not supported on all backends.
*RuntimeServiceApi* | [**runtime_service_watch_logs**](docs/RuntimeServiceApi.md#runtime_service_watch_logs) | **GET** /v1/instances/{instanceId}/logs/watch | WatchLogs streams new logs emitted from a controller
*RuntimeServiceApi* | [**runtime_service_watch_resources**](docs/RuntimeServiceApi.md#runtime_service_watch_resources) | **GET** /v1/instances/{instanceId}/resources/-/watch | WatchResources streams updates to catalog resources (including creation and deletion events)


## Documentation For Models

 - [BucketExtractPolicyStrategy](docs/BucketExtractPolicyStrategy.md)
 - [ColumnTimeSeriesRequestBasicMeasure](docs/ColumnTimeSeriesRequestBasicMeasure.md)
 - [ConnectorDriverProperty](docs/ConnectorDriverProperty.md)
 - [ConnectorDriverPropertyType](docs/ConnectorDriverPropertyType.md)
 - [DeprecatedUseMetricsViewComparisonRequestWithoutAComparisonTimeRange](docs/DeprecatedUseMetricsViewComparisonRequestWithoutAComparisonTimeRange.md)
 - [MetricsViewFilterCond](docs/MetricsViewFilterCond.md)
 - [MetricsViewSearchResponseSearchResult](docs/MetricsViewSearchResponseSearchResult.md)
 - [MetricsViewSpecAvailableComparisonOffset](docs/MetricsViewSpecAvailableComparisonOffset.md)
 - [MetricsViewSpecAvailableTimeRange](docs/MetricsViewSpecAvailableTimeRange.md)
 - [MetricsViewSpecComparisonMode](docs/MetricsViewSpecComparisonMode.md)
 - [MetricsViewSpecDimensionSelector](docs/MetricsViewSpecDimensionSelector.md)
 - [MetricsViewSpecDimensionV2](docs/MetricsViewSpecDimensionV2.md)
 - [MetricsViewSpecMeasureType](docs/MetricsViewSpecMeasureType.md)
 - [MetricsViewSpecMeasureV2](docs/MetricsViewSpecMeasureV2.md)
 - [MetricsViewSpecMeasureWindow](docs/MetricsViewSpecMeasureWindow.md)
 - [NumericHistogramBinsBin](docs/NumericHistogramBinsBin.md)
 - [NumericOutliersOutlier](docs/NumericOutliersOutlier.md)
 - [ProtobufAny](docs/ProtobufAny.md)
 - [ProtobufNullValue](docs/ProtobufNullValue.md)
 - [QueryServiceColumnRollupIntervalRequest](docs/QueryServiceColumnRollupIntervalRequest.md)
 - [QueryServiceColumnTimeSeriesRequest](docs/QueryServiceColumnTimeSeriesRequest.md)
 - [QueryServiceColumnTopKRequest](docs/QueryServiceColumnTopKRequest.md)
 - [QueryServiceExportReportRequest](docs/QueryServiceExportReportRequest.md)
 - [QueryServiceExportRequest](docs/QueryServiceExportRequest.md)
 - [QueryServiceMetricsViewAggregationRequest](docs/QueryServiceMetricsViewAggregationRequest.md)
 - [QueryServiceMetricsViewRowsRequest](docs/QueryServiceMetricsViewRowsRequest.md)
 - [QueryServiceMetricsViewSearchRequest](docs/QueryServiceMetricsViewSearchRequest.md)
 - [QueryServiceMetricsViewTimeRangeRequest](docs/QueryServiceMetricsViewTimeRangeRequest.md)
 - [QueryServiceMetricsViewTimeSeriesRequest](docs/QueryServiceMetricsViewTimeSeriesRequest.md)
 - [QueryServiceMetricsViewTotalsRequest](docs/QueryServiceMetricsViewTotalsRequest.md)
 - [QueryServiceQueryBatchRequest](docs/QueryServiceQueryBatchRequest.md)
 - [QueryServiceQueryRequest](docs/QueryServiceQueryRequest.md)
 - [QueryServiceResolveComponentRequest](docs/QueryServiceResolveComponentRequest.md)
 - [RequestMessageForQueryServiceMetricsViewComparison](docs/RequestMessageForQueryServiceMetricsViewComparison.md)
 - [RequestMessageForRuntimeServiceCreateDirectory](docs/RequestMessageForRuntimeServiceCreateDirectory.md)
 - [RequestMessageForRuntimeServiceGenerateMetricsViewFile](docs/RequestMessageForRuntimeServiceGenerateMetricsViewFile.md)
 - [RequestMessageForRuntimeServicePutFile](docs/RequestMessageForRuntimeServicePutFile.md)
 - [RequestMessageForRuntimeServiceRenameFile](docs/RequestMessageForRuntimeServiceRenameFile.md)
 - [RequestMessageForRuntimeServiceUnpackEmpty](docs/RequestMessageForRuntimeServiceUnpackEmpty.md)
 - [RequestMessageForRuntimeServiceUnpackExample](docs/RequestMessageForRuntimeServiceUnpackExample.md)
 - [RpcStatus](docs/RpcStatus.md)
 - [RuntimeServiceCreateTriggerRequest](docs/RuntimeServiceCreateTriggerRequest.md)
 - [RuntimeServiceEditInstanceRequest](docs/RuntimeServiceEditInstanceRequest.md)
 - [RuntimeServiceGenerateRendererRequest](docs/RuntimeServiceGenerateRendererRequest.md)
 - [RuntimeServiceGenerateResolverRequest](docs/RuntimeServiceGenerateResolverRequest.md)
 - [Runtimev1Operation](docs/Runtimev1Operation.md)
 - [Runtimev1Type](docs/Runtimev1Type.md)
 - [StreamResultOfV1QueryBatchResponse](docs/StreamResultOfV1QueryBatchResponse.md)
 - [StreamResultOfV1WatchFilesResponse](docs/StreamResultOfV1WatchFilesResponse.md)
 - [StreamResultOfV1WatchLogsResponse](docs/StreamResultOfV1WatchLogsResponse.md)
 - [StreamResultOfV1WatchResourcesResponse](docs/StreamResultOfV1WatchResourcesResponse.md)
 - [StructTypeField](docs/StructTypeField.md)
 - [TimeRangeSummaryInterval](docs/TimeRangeSummaryInterval.md)
 - [TopKEntry](docs/TopKEntry.md)
 - [TypeCode](docs/TypeCode.md)
 - [V1Alert](docs/V1Alert.md)
 - [V1AlertExecution](docs/V1AlertExecution.md)
 - [V1AlertSpec](docs/V1AlertSpec.md)
 - [V1AlertState](docs/V1AlertState.md)
 - [V1AnalyzeConnectorsResponse](docs/V1AnalyzeConnectorsResponse.md)
 - [V1AnalyzeVariablesResponse](docs/V1AnalyzeVariablesResponse.md)
 - [V1AnalyzedConnector](docs/V1AnalyzedConnector.md)
 - [V1AnalyzedVariable](docs/V1AnalyzedVariable.md)
 - [V1Api](docs/V1Api.md)
 - [V1ApiSpec](docs/V1ApiSpec.md)
 - [V1AssertionResult](docs/V1AssertionResult.md)
 - [V1AssertionStatus](docs/V1AssertionStatus.md)
 - [V1BigQueryListDatasetsResponse](docs/V1BigQueryListDatasetsResponse.md)
 - [V1BigQueryListTablesResponse](docs/V1BigQueryListTablesResponse.md)
 - [V1BucketExtractPolicy](docs/V1BucketExtractPolicy.md)
 - [V1BucketPlanner](docs/V1BucketPlanner.md)
 - [V1BucketPlannerSpec](docs/V1BucketPlannerSpec.md)
 - [V1BucketPlannerState](docs/V1BucketPlannerState.md)
 - [V1BuiltinMeasure](docs/V1BuiltinMeasure.md)
 - [V1Canvas](docs/V1Canvas.md)
 - [V1CanvasItem](docs/V1CanvasItem.md)
 - [V1CanvasSpec](docs/V1CanvasSpec.md)
 - [V1CanvasState](docs/V1CanvasState.md)
 - [V1CategoricalSummary](docs/V1CategoricalSummary.md)
 - [V1CharLocation](docs/V1CharLocation.md)
 - [V1Color](docs/V1Color.md)
 - [V1ColumnCardinalityRequest](docs/V1ColumnCardinalityRequest.md)
 - [V1ColumnCardinalityResponse](docs/V1ColumnCardinalityResponse.md)
 - [V1ColumnDescriptiveStatisticsRequest](docs/V1ColumnDescriptiveStatisticsRequest.md)
 - [V1ColumnDescriptiveStatisticsResponse](docs/V1ColumnDescriptiveStatisticsResponse.md)
 - [V1ColumnNullCountRequest](docs/V1ColumnNullCountRequest.md)
 - [V1ColumnNullCountResponse](docs/V1ColumnNullCountResponse.md)
 - [V1ColumnNumericHistogramRequest](docs/V1ColumnNumericHistogramRequest.md)
 - [V1ColumnNumericHistogramResponse](docs/V1ColumnNumericHistogramResponse.md)
 - [V1ColumnRollupIntervalRequest](docs/V1ColumnRollupIntervalRequest.md)
 - [V1ColumnRollupIntervalResponse](docs/V1ColumnRollupIntervalResponse.md)
 - [V1ColumnRugHistogramRequest](docs/V1ColumnRugHistogramRequest.md)
 - [V1ColumnRugHistogramResponse](docs/V1ColumnRugHistogramResponse.md)
 - [V1ColumnTimeGrainRequest](docs/V1ColumnTimeGrainRequest.md)
 - [V1ColumnTimeGrainResponse](docs/V1ColumnTimeGrainResponse.md)
 - [V1ColumnTimeRangeRequest](docs/V1ColumnTimeRangeRequest.md)
 - [V1ColumnTimeRangeResponse](docs/V1ColumnTimeRangeResponse.md)
 - [V1ColumnTimeSeriesRequest](docs/V1ColumnTimeSeriesRequest.md)
 - [V1ColumnTimeSeriesResponse](docs/V1ColumnTimeSeriesResponse.md)
 - [V1ColumnTopKRequest](docs/V1ColumnTopKRequest.md)
 - [V1ColumnTopKResponse](docs/V1ColumnTopKResponse.md)
 - [V1Component](docs/V1Component.md)
 - [V1ComponentSpec](docs/V1ComponentSpec.md)
 - [V1ComponentState](docs/V1ComponentState.md)
 - [V1ComponentVariable](docs/V1ComponentVariable.md)
 - [V1Condition](docs/V1Condition.md)
 - [V1Connector](docs/V1Connector.md)
 - [V1ConnectorDriver](docs/V1ConnectorDriver.md)
 - [V1ConnectorSpec](docs/V1ConnectorSpec.md)
 - [V1ConnectorState](docs/V1ConnectorState.md)
 - [V1ConnectorV2](docs/V1ConnectorV2.md)
 - [V1CreateInstanceRequest](docs/V1CreateInstanceRequest.md)
 - [V1CreateInstanceResponse](docs/V1CreateInstanceResponse.md)
 - [V1DirEntry](docs/V1DirEntry.md)
 - [V1EditInstanceResponse](docs/V1EditInstanceResponse.md)
 - [V1Example](docs/V1Example.md)
 - [V1Explore](docs/V1Explore.md)
 - [V1ExploreComparisonMode](docs/V1ExploreComparisonMode.md)
 - [V1ExploreComparisonTimeRange](docs/V1ExploreComparisonTimeRange.md)
 - [V1ExplorePreset](docs/V1ExplorePreset.md)
 - [V1ExploreSortType](docs/V1ExploreSortType.md)
 - [V1ExploreSpec](docs/V1ExploreSpec.md)
 - [V1ExploreState](docs/V1ExploreState.md)
 - [V1ExploreTimeRange](docs/V1ExploreTimeRange.md)
 - [V1ExploreWebView](docs/V1ExploreWebView.md)
 - [V1ExportFormat](docs/V1ExportFormat.md)
 - [V1ExportReportResponse](docs/V1ExportReportResponse.md)
 - [V1ExportResponse](docs/V1ExportResponse.md)
 - [V1Expression](docs/V1Expression.md)
 - [V1FieldSelector](docs/V1FieldSelector.md)
 - [V1FileEvent](docs/V1FileEvent.md)
 - [V1GcsGetCredentialsInfoResponse](docs/V1GcsGetCredentialsInfoResponse.md)
 - [V1GcsListBucketsResponse](docs/V1GcsListBucketsResponse.md)
 - [V1GcsListObjectsResponse](docs/V1GcsListObjectsResponse.md)
 - [V1GcsObject](docs/V1GcsObject.md)
 - [V1GenerateMetricsViewFileResponse](docs/V1GenerateMetricsViewFileResponse.md)
 - [V1GenerateRendererResponse](docs/V1GenerateRendererResponse.md)
 - [V1GenerateResolverResponse](docs/V1GenerateResolverResponse.md)
 - [V1GetExploreResponse](docs/V1GetExploreResponse.md)
 - [V1GetFileResponse](docs/V1GetFileResponse.md)
 - [V1GetInstanceResponse](docs/V1GetInstanceResponse.md)
 - [V1GetLogsResponse](docs/V1GetLogsResponse.md)
 - [V1GetModelPartitionsResponse](docs/V1GetModelPartitionsResponse.md)
 - [V1GetResourceResponse](docs/V1GetResourceResponse.md)
 - [V1HealthResponse](docs/V1HealthResponse.md)
 - [V1HistogramMethod](docs/V1HistogramMethod.md)
 - [V1Instance](docs/V1Instance.md)
 - [V1InstanceHealth](docs/V1InstanceHealth.md)
 - [V1InstanceHealthResponse](docs/V1InstanceHealthResponse.md)
 - [V1IssueDevJwtRequest](docs/V1IssueDevJwtRequest.md)
 - [V1IssueDevJwtResponse](docs/V1IssueDevJwtResponse.md)
 - [V1ListConnectorDriversResponse](docs/V1ListConnectorDriversResponse.md)
 - [V1ListExamplesResponse](docs/V1ListExamplesResponse.md)
 - [V1ListFilesResponse](docs/V1ListFilesResponse.md)
 - [V1ListInstancesResponse](docs/V1ListInstancesResponse.md)
 - [V1ListNotifierConnectorsResponse](docs/V1ListNotifierConnectorsResponse.md)
 - [V1ListResourcesResponse](docs/V1ListResourcesResponse.md)
 - [V1Log](docs/V1Log.md)
 - [V1LogLevel](docs/V1LogLevel.md)
 - [V1MapType](docs/V1MapType.md)
 - [V1MetricsViewAggregationDimension](docs/V1MetricsViewAggregationDimension.md)
 - [V1MetricsViewAggregationMeasure](docs/V1MetricsViewAggregationMeasure.md)
 - [V1MetricsViewAggregationMeasureComputeComparisonDelta](docs/V1MetricsViewAggregationMeasureComputeComparisonDelta.md)
 - [V1MetricsViewAggregationMeasureComputeComparisonRatio](docs/V1MetricsViewAggregationMeasureComputeComparisonRatio.md)
 - [V1MetricsViewAggregationMeasureComputeComparisonValue](docs/V1MetricsViewAggregationMeasureComputeComparisonValue.md)
 - [V1MetricsViewAggregationMeasureComputeCountDistinct](docs/V1MetricsViewAggregationMeasureComputeCountDistinct.md)
 - [V1MetricsViewAggregationMeasureComputePercentOfTotal](docs/V1MetricsViewAggregationMeasureComputePercentOfTotal.md)
 - [V1MetricsViewAggregationMeasureComputeUri](docs/V1MetricsViewAggregationMeasureComputeUri.md)
 - [V1MetricsViewAggregationRequest](docs/V1MetricsViewAggregationRequest.md)
 - [V1MetricsViewAggregationResponse](docs/V1MetricsViewAggregationResponse.md)
 - [V1MetricsViewAggregationSort](docs/V1MetricsViewAggregationSort.md)
 - [V1MetricsViewColumn](docs/V1MetricsViewColumn.md)
 - [V1MetricsViewComparisonMeasureAlias](docs/V1MetricsViewComparisonMeasureAlias.md)
 - [V1MetricsViewComparisonMeasureType](docs/V1MetricsViewComparisonMeasureType.md)
 - [V1MetricsViewComparisonRequest](docs/V1MetricsViewComparisonRequest.md)
 - [V1MetricsViewComparisonResponse](docs/V1MetricsViewComparisonResponse.md)
 - [V1MetricsViewComparisonRow](docs/V1MetricsViewComparisonRow.md)
 - [V1MetricsViewComparisonSort](docs/V1MetricsViewComparisonSort.md)
 - [V1MetricsViewComparisonSortType](docs/V1MetricsViewComparisonSortType.md)
 - [V1MetricsViewComparisonValue](docs/V1MetricsViewComparisonValue.md)
 - [V1MetricsViewFilter](docs/V1MetricsViewFilter.md)
 - [V1MetricsViewRowsRequest](docs/V1MetricsViewRowsRequest.md)
 - [V1MetricsViewRowsResponse](docs/V1MetricsViewRowsResponse.md)
 - [V1MetricsViewSchemaResponse](docs/V1MetricsViewSchemaResponse.md)
 - [V1MetricsViewSearchResponse](docs/V1MetricsViewSearchResponse.md)
 - [V1MetricsViewSort](docs/V1MetricsViewSort.md)
 - [V1MetricsViewSpec](docs/V1MetricsViewSpec.md)
 - [V1MetricsViewState](docs/V1MetricsViewState.md)
 - [V1MetricsViewTimeRangeResponse](docs/V1MetricsViewTimeRangeResponse.md)
 - [V1MetricsViewTimeSeriesRequest](docs/V1MetricsViewTimeSeriesRequest.md)
 - [V1MetricsViewTimeSeriesResponse](docs/V1MetricsViewTimeSeriesResponse.md)
 - [V1MetricsViewToplistRequest](docs/V1MetricsViewToplistRequest.md)
 - [V1MetricsViewToplistResponse](docs/V1MetricsViewToplistResponse.md)
 - [V1MetricsViewTotalsRequest](docs/V1MetricsViewTotalsRequest.md)
 - [V1MetricsViewTotalsResponse](docs/V1MetricsViewTotalsResponse.md)
 - [V1MetricsViewV2](docs/V1MetricsViewV2.md)
 - [V1Migration](docs/V1Migration.md)
 - [V1MigrationSpec](docs/V1MigrationSpec.md)
 - [V1MigrationState](docs/V1MigrationState.md)
 - [V1ModelPartition](docs/V1ModelPartition.md)
 - [V1ModelSpec](docs/V1ModelSpec.md)
 - [V1ModelState](docs/V1ModelState.md)
 - [V1ModelV2](docs/V1ModelV2.md)
 - [V1Notifier](docs/V1Notifier.md)
 - [V1NumericHistogramBins](docs/V1NumericHistogramBins.md)
 - [V1NumericOutliers](docs/V1NumericOutliers.md)
 - [V1NumericStatistics](docs/V1NumericStatistics.md)
 - [V1NumericSummary](docs/V1NumericSummary.md)
 - [V1OlapGetTableResponse](docs/V1OlapGetTableResponse.md)
 - [V1OlapListTablesResponse](docs/V1OlapListTablesResponse.md)
 - [V1ParseError](docs/V1ParseError.md)
 - [V1PingResponse](docs/V1PingResponse.md)
 - [V1ProfileColumn](docs/V1ProfileColumn.md)
 - [V1ProjectParser](docs/V1ProjectParser.md)
 - [V1ProjectParserState](docs/V1ProjectParserState.md)
 - [V1PullTrigger](docs/V1PullTrigger.md)
 - [V1PutFileResponse](docs/V1PutFileResponse.md)
 - [V1Query](docs/V1Query.md)
 - [V1QueryBatchResponse](docs/V1QueryBatchResponse.md)
 - [V1QueryResponse](docs/V1QueryResponse.md)
 - [V1QueryResult](docs/V1QueryResult.md)
 - [V1ReconcileStatus](docs/V1ReconcileStatus.md)
 - [V1RefreshModelTrigger](docs/V1RefreshModelTrigger.md)
 - [V1RefreshTrigger](docs/V1RefreshTrigger.md)
 - [V1RefreshTriggerSpec](docs/V1RefreshTriggerSpec.md)
 - [V1Report](docs/V1Report.md)
 - [V1ReportExecution](docs/V1ReportExecution.md)
 - [V1ReportSpec](docs/V1ReportSpec.md)
 - [V1ReportState](docs/V1ReportState.md)
 - [V1ResolveComponentResponse](docs/V1ResolveComponentResponse.md)
 - [V1Resource](docs/V1Resource.md)
 - [V1ResourceEvent](docs/V1ResourceEvent.md)
 - [V1ResourceMeta](docs/V1ResourceMeta.md)
 - [V1ResourceName](docs/V1ResourceName.md)
 - [V1S3GetBucketMetadataResponse](docs/V1S3GetBucketMetadataResponse.md)
 - [V1S3GetCredentialsInfoResponse](docs/V1S3GetCredentialsInfoResponse.md)
 - [V1S3ListBucketsResponse](docs/V1S3ListBucketsResponse.md)
 - [V1S3ListObjectsResponse](docs/V1S3ListObjectsResponse.md)
 - [V1S3Object](docs/V1S3Object.md)
 - [V1Schedule](docs/V1Schedule.md)
 - [V1SecurityRule](docs/V1SecurityRule.md)
 - [V1SecurityRuleAccess](docs/V1SecurityRuleAccess.md)
 - [V1SecurityRuleFieldAccess](docs/V1SecurityRuleFieldAccess.md)
 - [V1SecurityRuleRowFilter](docs/V1SecurityRuleRowFilter.md)
 - [V1SourceSpec](docs/V1SourceSpec.md)
 - [V1SourceState](docs/V1SourceState.md)
 - [V1SourceV2](docs/V1SourceV2.md)
 - [V1StringListValue](docs/V1StringListValue.md)
 - [V1StructType](docs/V1StructType.md)
 - [V1Subquery](docs/V1Subquery.md)
 - [V1TableCardinalityRequest](docs/V1TableCardinalityRequest.md)
 - [V1TableCardinalityResponse](docs/V1TableCardinalityResponse.md)
 - [V1TableColumnsRequest](docs/V1TableColumnsRequest.md)
 - [V1TableColumnsResponse](docs/V1TableColumnsResponse.md)
 - [V1TableInfo](docs/V1TableInfo.md)
 - [V1TableRowsRequest](docs/V1TableRowsRequest.md)
 - [V1TableRowsResponse](docs/V1TableRowsResponse.md)
 - [V1Theme](docs/V1Theme.md)
 - [V1ThemeSpec](docs/V1ThemeSpec.md)
 - [V1TimeGrain](docs/V1TimeGrain.md)
 - [V1TimeRange](docs/V1TimeRange.md)
 - [V1TimeRangeSummary](docs/V1TimeRangeSummary.md)
 - [V1TimeSeriesResponse](docs/V1TimeSeriesResponse.md)
 - [V1TimeSeriesTimeRange](docs/V1TimeSeriesTimeRange.md)
 - [V1TimeSeriesValue](docs/V1TimeSeriesValue.md)
 - [V1TopK](docs/V1TopK.md)
 - [V1WatchFilesResponse](docs/V1WatchFilesResponse.md)
 - [V1WatchLogsResponse](docs/V1WatchLogsResponse.md)
 - [V1WatchResourcesResponse](docs/V1WatchResourcesResponse.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author


