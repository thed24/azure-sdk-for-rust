#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The properties that are associated with an aggregate function."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AggregateFunctionProperties {
    #[serde(flatten)]
    pub function_properties: FunctionProperties,
}
impl AggregateFunctionProperties {
    pub fn new(function_properties: FunctionProperties) -> Self {
        Self { function_properties }
    }
}
#[doc = "Authentication Mode. Valid modes are `ConnectionString`, `Msi` and 'UserToken'."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AuthenticationMode")]
pub enum AuthenticationMode {
    Msi,
    UserToken,
    ConnectionString,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AuthenticationMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AuthenticationMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AuthenticationMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Msi => serializer.serialize_unit_variant("AuthenticationMode", 0u32, "Msi"),
            Self::UserToken => serializer.serialize_unit_variant("AuthenticationMode", 1u32, "UserToken"),
            Self::ConnectionString => serializer.serialize_unit_variant("AuthenticationMode", 2u32, "ConnectionString"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for AuthenticationMode {
    fn default() -> Self {
        Self::ConnectionString
    }
}
#[doc = "Describes how data from an input is serialized or how data is serialized when written to an output in Avro format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvroSerialization {
    #[serde(flatten)]
    pub serialization: Serialization,
    #[doc = "The properties that are associated with the Avro serialization type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AvroSerializationProperties>,
}
impl AvroSerialization {
    pub fn new(serialization: Serialization) -> Self {
        Self {
            serialization,
            properties: None,
        }
    }
}
#[doc = "The properties that are associated with the Avro serialization type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvroSerializationProperties {}
impl AvroSerializationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an Azure Data Lake Store output data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureDataLakeStoreOutputDataSource {
    #[serde(flatten)]
    pub output_data_source: OutputDataSource,
    #[doc = "The properties that are associated with an Azure Data Lake Store."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureDataLakeStoreOutputDataSourceProperties>,
}
impl AzureDataLakeStoreOutputDataSource {
    pub fn new(output_data_source: OutputDataSource) -> Self {
        Self {
            output_data_source,
            properties: None,
        }
    }
}
#[doc = "The properties that are associated with an Azure Data Lake Store."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDataLakeStoreOutputDataSourceProperties {
    #[serde(flatten)]
    pub o_auth_based_data_source_properties: OAuthBasedDataSourceProperties,
    #[doc = "The name of the Azure Data Lake Store account. Required on PUT (CreateOrReplace) requests."]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[doc = "The tenant id of the user used to obtain the refresh token. Required on PUT (CreateOrReplace) requests."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The location of the file to which the output should be written to. Required on PUT (CreateOrReplace) requests."]
    #[serde(rename = "filePathPrefix", default, skip_serializing_if = "Option::is_none")]
    pub file_path_prefix: Option<String>,
    #[doc = "The date format. Wherever {date} appears in filePathPrefix, the value of this property is used as the date format instead."]
    #[serde(rename = "dateFormat", default, skip_serializing_if = "Option::is_none")]
    pub date_format: Option<String>,
    #[doc = "The time format. Wherever {time} appears in filePathPrefix, the value of this property is used as the time format instead."]
    #[serde(rename = "timeFormat", default, skip_serializing_if = "Option::is_none")]
    pub time_format: Option<String>,
    #[doc = "Authentication Mode. Valid modes are `ConnectionString`, `Msi` and 'UserToken'."]
    #[serde(rename = "authenticationMode", default, skip_serializing_if = "Option::is_none")]
    pub authentication_mode: Option<AuthenticationMode>,
}
impl AzureDataLakeStoreOutputDataSourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the metadata of AzureFunctionOutputDataSource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFunctionOutputDataSource {
    #[serde(flatten)]
    pub output_data_source: OutputDataSource,
    #[doc = "The properties that are associated with a DocumentDB output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureFunctionOutputDataSourceProperties>,
}
impl AzureFunctionOutputDataSource {
    pub fn new(output_data_source: OutputDataSource) -> Self {
        Self {
            output_data_source,
            properties: None,
        }
    }
}
#[doc = "The properties that are associated with a DocumentDB output."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureFunctionOutputDataSourceProperties {
    #[doc = "The name of your Azure Functions app."]
    #[serde(rename = "functionAppName", default, skip_serializing_if = "Option::is_none")]
    pub function_app_name: Option<String>,
    #[doc = "The name of the function in your Azure Functions app."]
    #[serde(rename = "functionName", default, skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    #[doc = "If you want to use an Azure Function from another subscription, you can do so by providing the key to access your function."]
    #[serde(rename = "apiKey", default, skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[doc = "A property that lets you set the maximum size for each output batch that's sent to your Azure function. The input unit is in bytes. By default, this value is 262,144 bytes (256 KB)."]
    #[serde(rename = "maxBatchSize", default, skip_serializing_if = "Option::is_none")]
    pub max_batch_size: Option<f64>,
    #[doc = "A property that lets you specify the maximum number of events in each batch that's sent to Azure Functions. The default value is 100."]
    #[serde(rename = "maxBatchCount", default, skip_serializing_if = "Option::is_none")]
    pub max_batch_count: Option<f64>,
}
impl AzureFunctionOutputDataSourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The binding to an Azure Machine Learning web service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureMachineLearningServiceFunctionBinding {
    #[serde(flatten)]
    pub function_binding: FunctionBinding,
    #[doc = "The binding properties associated with an Azure Machine learning web service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureMachineLearningServiceFunctionBindingProperties>,
}
impl AzureMachineLearningServiceFunctionBinding {
    pub fn new(function_binding: FunctionBinding) -> Self {
        Self {
            function_binding,
            properties: None,
        }
    }
}
#[doc = "The binding properties associated with an Azure Machine learning web service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureMachineLearningServiceFunctionBindingProperties {
    #[doc = "The Request-Response execute endpoint of the Azure Machine Learning web service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[doc = "The API key used to authenticate with Request-Response endpoint."]
    #[serde(rename = "apiKey", default, skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[doc = "The inputs for the Azure Machine Learning web service endpoint."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub inputs: Vec<AzureMachineLearningServiceInputColumn>,
    #[doc = "A list of outputs from the Azure Machine Learning web service endpoint execution."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub outputs: Vec<AzureMachineLearningServiceOutputColumn>,
    #[doc = "Number between 1 and 10000 describing maximum number of rows for every Azure ML RRS execute request. Default is 1000."]
    #[serde(rename = "batchSize", default, skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
    #[doc = "The number of parallel requests that will be sent per partition of your job to the machine learning service. Default is 1."]
    #[serde(rename = "numberOfParallelRequests", default, skip_serializing_if = "Option::is_none")]
    pub number_of_parallel_requests: Option<i32>,
}
impl AzureMachineLearningServiceFunctionBindingProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The binding retrieval properties associated with an Azure Machine learning web service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureMachineLearningServiceFunctionBindingRetrievalProperties {
    #[doc = "The Request-Response execute endpoint of the Azure Machine Learning web service."]
    #[serde(rename = "executeEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub execute_endpoint: Option<String>,
    #[doc = "The function type."]
    #[serde(rename = "udfType", default, skip_serializing_if = "Option::is_none")]
    pub udf_type: Option<UdfType>,
}
impl AzureMachineLearningServiceFunctionBindingRetrievalProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters needed to retrieve the default function definition for an Azure Machine Learning web service function."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureMachineLearningServiceFunctionRetrieveDefaultDefinitionParameters {
    #[serde(flatten)]
    pub function_retrieve_default_definition_parameters: FunctionRetrieveDefaultDefinitionParameters,
    #[doc = "The binding retrieval properties associated with an Azure Machine learning web service."]
    #[serde(rename = "bindingRetrievalProperties", default, skip_serializing_if = "Option::is_none")]
    pub binding_retrieval_properties: Option<AzureMachineLearningServiceFunctionBindingRetrievalProperties>,
}
impl AzureMachineLearningServiceFunctionRetrieveDefaultDefinitionParameters {
    pub fn new(function_retrieve_default_definition_parameters: FunctionRetrieveDefaultDefinitionParameters) -> Self {
        Self {
            function_retrieve_default_definition_parameters,
            binding_retrieval_properties: None,
        }
    }
}
#[doc = "Describes an input column for the Azure Machine Learning web service endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureMachineLearningServiceInputColumn {
    #[doc = "The name of the input column."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The (Azure Machine Learning supported) data type of the input column."]
    #[serde(rename = "dataType", default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[doc = "The zero based index of the function parameter this input maps to."]
    #[serde(rename = "mapTo", default, skip_serializing_if = "Option::is_none")]
    pub map_to: Option<i32>,
}
impl AzureMachineLearningServiceInputColumn {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The inputs for the Azure Machine Learning web service endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureMachineLearningServiceInputs {
    #[doc = "The name of the input. This is the name provided while authoring the endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A list of input columns for the Azure Machine Learning web service endpoint."]
    #[serde(
        rename = "columnNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub column_names: Vec<AzureMachineLearningServiceInputColumn>,
}
impl AzureMachineLearningServiceInputs {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an output column for the Azure Machine Learning web service endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureMachineLearningServiceOutputColumn {
    #[doc = "The name of the output column."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The (Azure Machine Learning supported) data type of the output column."]
    #[serde(rename = "dataType", default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[doc = "The zero based index of the function parameter this input maps to."]
    #[serde(rename = "mapTo", default, skip_serializing_if = "Option::is_none")]
    pub map_to: Option<i32>,
}
impl AzureMachineLearningServiceOutputColumn {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The binding to an Azure Machine Learning Studio."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureMachineLearningStudioFunctionBinding {
    #[serde(flatten)]
    pub function_binding: FunctionBinding,
    #[doc = "The binding properties associated with an Azure Machine learning Studio."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureMachineLearningStudioFunctionBindingProperties>,
}
impl AzureMachineLearningStudioFunctionBinding {
    pub fn new(function_binding: FunctionBinding) -> Self {
        Self {
            function_binding,
            properties: None,
        }
    }
}
#[doc = "The binding properties associated with an Azure Machine learning Studio."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureMachineLearningStudioFunctionBindingProperties {
    #[doc = "The Request-Response execute endpoint of the Azure Machine Learning Studio. Find out more here: https://docs.microsoft.com/en-us/azure/machine-learning/machine-learning-consume-web-services#request-response-service-rrs"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[doc = "The API key used to authenticate with Request-Response endpoint."]
    #[serde(rename = "apiKey", default, skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[doc = "The inputs for the Azure Machine Learning Studio endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<AzureMachineLearningStudioInputs>,
    #[doc = "A list of outputs from the Azure Machine Learning Studio endpoint execution."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub outputs: Vec<AzureMachineLearningStudioOutputColumn>,
    #[doc = "Number between 1 and 10000 describing maximum number of rows for every Azure ML RRS execute request. Default is 1000."]
    #[serde(rename = "batchSize", default, skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
}
impl AzureMachineLearningStudioFunctionBindingProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The binding retrieval properties associated with an Azure Machine learning Studio."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureMachineLearningStudioFunctionBindingRetrievalProperties {
    #[doc = "The Request-Response execute endpoint of the Azure Machine Learning Studio. Find out more here: https://docs.microsoft.com/en-us/azure/machine-learning/machine-learning-consume-web-services#request-response-service-rrs"]
    #[serde(rename = "executeEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub execute_endpoint: Option<String>,
    #[doc = "The function type."]
    #[serde(rename = "udfType", default, skip_serializing_if = "Option::is_none")]
    pub udf_type: Option<UdfType>,
}
impl AzureMachineLearningStudioFunctionBindingRetrievalProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters needed to retrieve the default function definition for an Azure Machine Learning Studio function."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureMachineLearningStudioFunctionRetrieveDefaultDefinitionParameters {
    #[serde(flatten)]
    pub function_retrieve_default_definition_parameters: FunctionRetrieveDefaultDefinitionParameters,
    #[doc = "The binding retrieval properties associated with an Azure Machine learning Studio."]
    #[serde(rename = "bindingRetrievalProperties", default, skip_serializing_if = "Option::is_none")]
    pub binding_retrieval_properties: Option<AzureMachineLearningStudioFunctionBindingRetrievalProperties>,
}
impl AzureMachineLearningStudioFunctionRetrieveDefaultDefinitionParameters {
    pub fn new(function_retrieve_default_definition_parameters: FunctionRetrieveDefaultDefinitionParameters) -> Self {
        Self {
            function_retrieve_default_definition_parameters,
            binding_retrieval_properties: None,
        }
    }
}
#[doc = "Describes an input column for the Azure Machine Learning Studio endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureMachineLearningStudioInputColumn {
    #[doc = "The name of the input column."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The (Azure Machine Learning supported) data type of the input column. A list of valid  Azure Machine Learning data types are described at https://msdn.microsoft.com/en-us/library/azure/dn905923.aspx ."]
    #[serde(rename = "dataType", default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[doc = "The zero based index of the function parameter this input maps to."]
    #[serde(rename = "mapTo", default, skip_serializing_if = "Option::is_none")]
    pub map_to: Option<i32>,
}
impl AzureMachineLearningStudioInputColumn {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The inputs for the Azure Machine Learning Studio endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureMachineLearningStudioInputs {
    #[doc = "The name of the input. This is the name provided while authoring the endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A list of input columns for the Azure Machine Learning Studio endpoint."]
    #[serde(
        rename = "columnNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub column_names: Vec<AzureMachineLearningStudioInputColumn>,
}
impl AzureMachineLearningStudioInputs {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an output column for the Azure Machine Learning Studio endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureMachineLearningStudioOutputColumn {
    #[doc = "The name of the output column."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The (Azure Machine Learning supported) data type of the output column. A list of valid  Azure Machine Learning data types are described at https://msdn.microsoft.com/en-us/library/azure/dn905923.aspx ."]
    #[serde(rename = "dataType", default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
}
impl AzureMachineLearningStudioOutputColumn {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties that are associated with an Azure SQL database data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSqlDatabaseDataSourceProperties {
    #[doc = "The name of the SQL server containing the Azure SQL database. Required on PUT (CreateOrReplace) requests."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    #[doc = "The name of the Azure SQL database. Required on PUT (CreateOrReplace) requests."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[doc = "The user name that will be used to connect to the Azure SQL database. Required on PUT (CreateOrReplace) requests."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[doc = "The password that will be used to connect to the Azure SQL database. Required on PUT (CreateOrReplace) requests."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "The name of the table in the Azure SQL database. Required on PUT (CreateOrReplace) requests."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[doc = "Max Batch count for write to Sql database, the default value is 10,000. Optional on PUT requests."]
    #[serde(rename = "maxBatchCount", default, skip_serializing_if = "Option::is_none")]
    pub max_batch_count: Option<f64>,
    #[doc = "Max Write r count, currently only 1(single writer) and 0(based on query partition) are available. Optional on PUT requests."]
    #[serde(rename = "maxWriterCount", default, skip_serializing_if = "Option::is_none")]
    pub max_writer_count: Option<f64>,
    #[doc = "Authentication Mode. Valid modes are `ConnectionString`, `Msi` and 'UserToken'."]
    #[serde(rename = "authenticationMode", default, skip_serializing_if = "Option::is_none")]
    pub authentication_mode: Option<AuthenticationMode>,
}
impl AzureSqlDatabaseDataSourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an Azure SQL database output data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlDatabaseOutputDataSource {
    #[serde(flatten)]
    pub output_data_source: OutputDataSource,
    #[doc = "The properties that are associated with an Azure SQL database output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSqlDatabaseOutputDataSourceProperties>,
}
impl AzureSqlDatabaseOutputDataSource {
    pub fn new(output_data_source: OutputDataSource) -> Self {
        Self {
            output_data_source,
            properties: None,
        }
    }
}
#[doc = "The properties that are associated with an Azure SQL database output."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSqlDatabaseOutputDataSourceProperties {
    #[serde(flatten)]
    pub azure_sql_database_data_source_properties: AzureSqlDatabaseDataSourceProperties,
}
impl AzureSqlDatabaseOutputDataSourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an Azure SQL database reference input data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSqlReferenceInputDataSource {
    #[serde(flatten)]
    pub reference_input_data_source: ReferenceInputDataSource,
    #[doc = "Describes Azure SQL database reference input data source properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSqlReferenceInputDataSourceProperties>,
}
impl AzureSqlReferenceInputDataSource {
    pub fn new(reference_input_data_source: ReferenceInputDataSource) -> Self {
        Self {
            reference_input_data_source,
            properties: None,
        }
    }
}
#[doc = "Describes Azure SQL database reference input data source properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSqlReferenceInputDataSourceProperties {
    #[doc = "This element is associated with the datasource element. This is the name of the server that contains the database that will be written to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    #[doc = "This element is associated with the datasource element. This is the name of the database that output will be written to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[doc = "This element is associated with the datasource element. This is the user name that will be used to connect to the SQL Database instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[doc = "This element is associated with the datasource element. This is the password that will be used to connect to the SQL Database instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "This element is associated with the datasource element. The name of the table in the Azure SQL database.."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[doc = "This element is associated with the datasource element. This element is of enum type. It indicates what kind of data refresh option do we want to use:Static/RefreshPeriodicallyWithFull/RefreshPeriodicallyWithDelta"]
    #[serde(rename = "refreshType", default, skip_serializing_if = "Option::is_none")]
    pub refresh_type: Option<String>,
    #[doc = "This element is associated with the datasource element. This indicates how frequently the data will be fetched from the database. It is of DateTime format."]
    #[serde(rename = "refreshRate", default, skip_serializing_if = "Option::is_none")]
    pub refresh_rate: Option<String>,
    #[doc = "This element is associated with the datasource element. This query is used to fetch data from the sql database."]
    #[serde(rename = "fullSnapshotQuery", default, skip_serializing_if = "Option::is_none")]
    pub full_snapshot_query: Option<String>,
    #[doc = "This element is associated with the datasource element. This query is used to fetch incremental changes from the SQL database. To use this option, we recommend using temporal tables in Azure SQL Database."]
    #[serde(rename = "deltaSnapshotQuery", default, skip_serializing_if = "Option::is_none")]
    pub delta_snapshot_query: Option<String>,
}
impl AzureSqlReferenceInputDataSourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties that are associated with an Azure SQL database data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSynapseDataSourceProperties {
    #[doc = "The name of the SQL server containing the Azure SQL database. Required on PUT (CreateOrReplace) requests."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    #[doc = "The name of the Azure SQL database. Required on PUT (CreateOrReplace) requests."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[doc = "The name of the table in the Azure SQL database. Required on PUT (CreateOrReplace) requests."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[doc = "The user name that will be used to connect to the Azure SQL database. Required on PUT (CreateOrReplace) requests."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[doc = "The password that will be used to connect to the Azure SQL database. Required on PUT (CreateOrReplace) requests."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl AzureSynapseDataSourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an Azure Synapse output data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSynapseOutputDataSource {
    #[serde(flatten)]
    pub output_data_source: OutputDataSource,
    #[doc = "The properties that are associated with an Azure Synapse output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureSynapseOutputDataSourceProperties>,
}
impl AzureSynapseOutputDataSource {
    pub fn new(output_data_source: OutputDataSource) -> Self {
        Self {
            output_data_source,
            properties: None,
        }
    }
}
#[doc = "The properties that are associated with an Azure Synapse output."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureSynapseOutputDataSourceProperties {
    #[serde(flatten)]
    pub azure_synapse_data_source_properties: AzureSynapseDataSourceProperties,
}
impl AzureSynapseOutputDataSourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an Azure Table output data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureTableOutputDataSource {
    #[serde(flatten)]
    pub output_data_source: OutputDataSource,
    #[doc = "The properties that are associated with an Azure Table output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureTableOutputDataSourceProperties>,
}
impl AzureTableOutputDataSource {
    pub fn new(output_data_source: OutputDataSource) -> Self {
        Self {
            output_data_source,
            properties: None,
        }
    }
}
#[doc = "The properties that are associated with an Azure Table output."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureTableOutputDataSourceProperties {
    #[doc = "The name of the Azure Storage account. Required on PUT (CreateOrReplace) requests."]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[doc = "The account key for the Azure Storage account. Required on PUT (CreateOrReplace) requests."]
    #[serde(rename = "accountKey", default, skip_serializing_if = "Option::is_none")]
    pub account_key: Option<String>,
    #[doc = "The name of the Azure Table. Required on PUT (CreateOrReplace) requests."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[doc = "This element indicates the name of a column from the SELECT statement in the query that will be used as the partition key for the Azure Table. Required on PUT (CreateOrReplace) requests."]
    #[serde(rename = "partitionKey", default, skip_serializing_if = "Option::is_none")]
    pub partition_key: Option<String>,
    #[doc = "This element indicates the name of a column from the SELECT statement in the query that will be used as the row key for the Azure Table. Required on PUT (CreateOrReplace) requests."]
    #[serde(rename = "rowKey", default, skip_serializing_if = "Option::is_none")]
    pub row_key: Option<String>,
    #[doc = "If specified, each item in the array is the name of a column to remove (if present) from output event entities."]
    #[serde(
        rename = "columnsToRemove",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub columns_to_remove: Vec<String>,
    #[doc = "The number of rows to write to the Azure Table at a time."]
    #[serde(rename = "batchSize", default, skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
}
impl AzureTableOutputDataSourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties that are associated with a blob data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlobDataSourceProperties {
    #[doc = "A list of one or more Azure Storage accounts. Required on PUT (CreateOrReplace) requests."]
    #[serde(
        rename = "storageAccounts",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub storage_accounts: Vec<StorageAccount>,
    #[doc = "The name of a container within the associated Storage account. This container contains either the blob(s) to be read from or written to. Required on PUT (CreateOrReplace) requests."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[doc = "The blob path pattern. Not a regular expression. It represents a pattern against which blob names will be matched to determine whether or not they should be included as input or output to the job. See https://docs.microsoft.com/en-us/rest/api/streamanalytics/stream-analytics-input or https://docs.microsoft.com/en-us/rest/api/streamanalytics/stream-analytics-output for a more detailed explanation and example."]
    #[serde(rename = "pathPattern", default, skip_serializing_if = "Option::is_none")]
    pub path_pattern: Option<String>,
    #[doc = "The date format. Wherever {date} appears in pathPattern, the value of this property is used as the date format instead."]
    #[serde(rename = "dateFormat", default, skip_serializing_if = "Option::is_none")]
    pub date_format: Option<String>,
    #[doc = "The time format. Wherever {time} appears in pathPattern, the value of this property is used as the time format instead."]
    #[serde(rename = "timeFormat", default, skip_serializing_if = "Option::is_none")]
    pub time_format: Option<String>,
    #[doc = "Authentication Mode. Valid modes are `ConnectionString`, `Msi` and 'UserToken'."]
    #[serde(rename = "authenticationMode", default, skip_serializing_if = "Option::is_none")]
    pub authentication_mode: Option<AuthenticationMode>,
}
impl BlobDataSourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a blob output data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobOutputDataSource {
    #[serde(flatten)]
    pub output_data_source: OutputDataSource,
    #[doc = "The properties that are associated with a blob output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BlobOutputDataSourceProperties>,
}
impl BlobOutputDataSource {
    pub fn new(output_data_source: OutputDataSource) -> Self {
        Self {
            output_data_source,
            properties: None,
        }
    }
}
#[doc = "The properties that are associated with a blob output."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlobOutputDataSourceProperties {
    #[serde(flatten)]
    pub blob_data_source_properties: BlobDataSourceProperties,
}
impl BlobOutputDataSourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a blob input data source that contains reference data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobReferenceInputDataSource {
    #[serde(flatten)]
    pub reference_input_data_source: ReferenceInputDataSource,
    #[doc = "The properties that are associated with a blob input containing reference data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BlobReferenceInputDataSourceProperties>,
}
impl BlobReferenceInputDataSource {
    pub fn new(reference_input_data_source: ReferenceInputDataSource) -> Self {
        Self {
            reference_input_data_source,
            properties: None,
        }
    }
}
#[doc = "The properties that are associated with a blob input containing reference data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlobReferenceInputDataSourceProperties {
    #[serde(flatten)]
    pub blob_data_source_properties: BlobDataSourceProperties,
}
impl BlobReferenceInputDataSourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a blob input data source that contains stream data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobStreamInputDataSource {
    #[serde(flatten)]
    pub stream_input_data_source: StreamInputDataSource,
    #[doc = "The properties that are associated with a blob input containing stream data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BlobStreamInputDataSourceProperties>,
}
impl BlobStreamInputDataSource {
    pub fn new(stream_input_data_source: StreamInputDataSource) -> Self {
        Self {
            stream_input_data_source,
            properties: None,
        }
    }
}
#[doc = "The properties that are associated with a blob input containing stream data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlobStreamInputDataSourceProperties {
    #[serde(flatten)]
    pub blob_data_source_properties: BlobDataSourceProperties,
    #[doc = "The partition count of the blob input data source. Range 1 - 1024."]
    #[serde(rename = "sourcePartitionCount", default, skip_serializing_if = "Option::is_none")]
    pub source_partition_count: Option<i32>,
}
impl BlobStreamInputDataSourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The binding to a CSharp function."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CSharpFunctionBinding {
    #[serde(flatten)]
    pub function_binding: FunctionBinding,
    #[doc = "The binding properties associated with a CSharp function."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CSharpFunctionBindingProperties>,
}
impl CSharpFunctionBinding {
    pub fn new(function_binding: FunctionBinding) -> Self {
        Self {
            function_binding,
            properties: None,
        }
    }
}
#[doc = "The binding properties associated with a CSharp function."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CSharpFunctionBindingProperties {
    #[doc = "The Csharp code containing a single function definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    #[doc = "The Csharp code containing a single function definition."]
    #[serde(rename = "dllPath", default, skip_serializing_if = "Option::is_none")]
    pub dll_path: Option<String>,
    #[doc = "The Csharp code containing a single function definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[doc = "The Csharp code containing a single function definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
}
impl CSharpFunctionBindingProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The binding retrieval properties associated with a CSharp function."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CSharpFunctionBindingRetrievalProperties {
    #[doc = "The CSharp code containing a single function definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    #[doc = "The function type."]
    #[serde(rename = "udfType", default, skip_serializing_if = "Option::is_none")]
    pub udf_type: Option<UdfType>,
}
impl CSharpFunctionBindingRetrievalProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters needed to retrieve the default function definition for a CSharp function."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CSharpFunctionRetrieveDefaultDefinitionParameters {
    #[serde(flatten)]
    pub function_retrieve_default_definition_parameters: FunctionRetrieveDefaultDefinitionParameters,
    #[doc = "The binding retrieval properties associated with a CSharp function."]
    #[serde(rename = "bindingRetrievalProperties", default, skip_serializing_if = "Option::is_none")]
    pub binding_retrieval_properties: Option<CSharpFunctionBindingRetrievalProperties>,
}
impl CSharpFunctionRetrieveDefaultDefinitionParameters {
    pub fn new(function_retrieve_default_definition_parameters: FunctionRetrieveDefaultDefinitionParameters) -> Self {
        Self {
            function_retrieve_default_definition_parameters,
            binding_retrieval_properties: None,
        }
    }
}
#[doc = "The properties associated with a Stream Analytics cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterInfo {
    #[doc = "The resource id of cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl ClusterInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Controls certain runtime behaviors of the streaming job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CompatibilityLevel")]
pub enum CompatibilityLevel {
    #[serde(rename = "1.0")]
    N1_0,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CompatibilityLevel {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CompatibilityLevel {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CompatibilityLevel {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::N1_0 => serializer.serialize_unit_variant("CompatibilityLevel", 0u32, "1.0"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The query compilation object which defines the input, output, and transformation for the query compilation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CompileQuery {
    #[doc = "The query to compile."]
    pub query: String,
    #[doc = "The inputs for the query compilation."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub inputs: Vec<QueryInput>,
    #[doc = "The functions for the query compilation."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub functions: Vec<QueryFunction>,
    #[doc = "Describes the type of the job. Valid values are `Cloud` and 'Edge'."]
    #[serde(rename = "jobType")]
    pub job_type: compile_query::JobType,
    #[doc = "Controls certain runtime behaviors of the streaming job."]
    #[serde(rename = "compatibilityLevel", default, skip_serializing_if = "Option::is_none")]
    pub compatibility_level: Option<CompatibilityLevel>,
}
impl CompileQuery {
    pub fn new(query: String, job_type: compile_query::JobType) -> Self {
        Self {
            query,
            inputs: Vec::new(),
            functions: Vec::new(),
            job_type,
            compatibility_level: None,
        }
    }
}
pub mod compile_query {
    use super::*;
    #[doc = "Describes the type of the job. Valid values are `Cloud` and 'Edge'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "JobType")]
    pub enum JobType {
        Cloud,
        Edge,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for JobType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for JobType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for JobType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Cloud => serializer.serialize_unit_variant("JobType", 0u32, "Cloud"),
                Self::Edge => serializer.serialize_unit_variant("JobType", 1u32, "Edge"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes how input data is compressed"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Compression {
    #[serde(rename = "type")]
    pub type_: String,
}
impl Compression {
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}
#[doc = "Describes how data from an input is serialized or how data is serialized when written to an output in CSV format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CsvSerialization {
    #[serde(flatten)]
    pub serialization: Serialization,
    #[doc = "The properties that are associated with the CSV serialization type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CsvSerializationProperties>,
}
impl CsvSerialization {
    pub fn new(serialization: Serialization) -> Self {
        Self {
            serialization,
            properties: None,
        }
    }
}
#[doc = "The properties that are associated with the CSV serialization type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CsvSerializationProperties {
    #[doc = "Specifies the delimiter that will be used to separate comma-separated value (CSV) records. See https://docs.microsoft.com/en-us/rest/api/streamanalytics/stream-analytics-input or https://docs.microsoft.com/en-us/rest/api/streamanalytics/stream-analytics-output for a list of supported values. Required on PUT (CreateOrReplace) requests."]
    #[serde(rename = "fieldDelimiter", default, skip_serializing_if = "Option::is_none")]
    pub field_delimiter: Option<String>,
    #[doc = "Specifies the encoding of the incoming data in the case of input and the encoding of outgoing data in the case of output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<Encoding>,
}
impl CsvSerializationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes how data from an input is serialized or how data is serialized when written to an output in custom format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomClrSerialization {
    #[serde(flatten)]
    pub serialization: Serialization,
    #[doc = "The properties that are associated with the CustomClr serialization type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CustomClrSerializationProperties>,
}
impl CustomClrSerialization {
    pub fn new(serialization: Serialization) -> Self {
        Self {
            serialization,
            properties: None,
        }
    }
}
#[doc = "The properties that are associated with the CustomClr serialization type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomClrSerializationProperties {
    #[doc = "The serialization library path."]
    #[serde(rename = "serializationDllPath", default, skip_serializing_if = "Option::is_none")]
    pub serialization_dll_path: Option<String>,
    #[doc = "The serialization class name."]
    #[serde(rename = "serializationClassName", default, skip_serializing_if = "Option::is_none")]
    pub serialization_class_name: Option<String>,
}
impl CustomClrSerializationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Condition applicable to the resource, or to the job overall, that warrant customer attention."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticCondition {
    #[doc = "The UTC timestamp of when the condition started. Customers should be able to find a corresponding event in the ops log around this time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    #[doc = "The opaque diagnostic code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The human-readable message describing the condition in detail. Localized in the Accept-Language of the client request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl DiagnosticCondition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes conditions applicable to the Input, Output, or the job overall, that warrant customer attention."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Diagnostics {
    #[doc = "A collection of zero or more conditions applicable to the resource, or to the job overall, that warrant customer attention."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub conditions: Vec<DiagnosticCondition>,
}
impl Diagnostics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a DocumentDB output data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DocumentDbOutputDataSource {
    #[serde(flatten)]
    pub output_data_source: OutputDataSource,
    #[doc = "The properties that are associated with a DocumentDB output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DocumentDbOutputDataSourceProperties>,
}
impl DocumentDbOutputDataSource {
    pub fn new(output_data_source: OutputDataSource) -> Self {
        Self {
            output_data_source,
            properties: None,
        }
    }
}
#[doc = "The properties that are associated with a DocumentDB output."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DocumentDbOutputDataSourceProperties {
    #[doc = "The DocumentDB account name or ID. Required on PUT (CreateOrReplace) requests."]
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[doc = "The account key for the DocumentDB account. Required on PUT (CreateOrReplace) requests."]
    #[serde(rename = "accountKey", default, skip_serializing_if = "Option::is_none")]
    pub account_key: Option<String>,
    #[doc = "The name of the DocumentDB database. Required on PUT (CreateOrReplace) requests."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[doc = "The collection name pattern for the collections to be used. The collection name format can be constructed using the optional {partition} token, where partitions start from 0. See the DocumentDB section of https://docs.microsoft.com/en-us/rest/api/streamanalytics/stream-analytics-output for more information. Required on PUT (CreateOrReplace) requests."]
    #[serde(rename = "collectionNamePattern", default, skip_serializing_if = "Option::is_none")]
    pub collection_name_pattern: Option<String>,
    #[doc = "The name of the field in output events used to specify the key for partitioning output across collections. If 'collectionNamePattern' contains the {partition} token, this property is required to be specified."]
    #[serde(rename = "partitionKey", default, skip_serializing_if = "Option::is_none")]
    pub partition_key: Option<String>,
    #[doc = "The name of the field in output events used to specify the primary key which insert or update operations are based on."]
    #[serde(rename = "documentId", default, skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
}
impl DocumentDbOutputDataSourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the encoding of the incoming data in the case of input and the encoding of outgoing data in the case of output."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Encoding")]
pub enum Encoding {
    #[serde(rename = "UTF8")]
    Utf8,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Encoding {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Encoding {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Encoding {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Utf8 => serializer.serialize_unit_variant("Encoding", 0u32, "UTF8"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Common error representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[doc = "Error definition properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<error::Error>,
}
impl azure_core::Continuable for Error {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod error {
    use super::*;
    #[doc = "Error definition properties."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Error {
        #[doc = "Error code."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[doc = "Error message."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[doc = "Error target."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub target: Option<String>,
        #[doc = "Error details."]
        #[serde(
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub details: Vec<ErrorDetails>,
    }
    impl Error {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Common error details representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetails {
    #[doc = "Error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the error that occurred."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error code associated with the error that occurred."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Describes the error in detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The common properties that are associated with Event Hub data sources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventHubDataSourceProperties {
    #[serde(flatten)]
    pub service_bus_data_source_properties: ServiceBusDataSourceProperties,
    #[doc = "The name of the Event Hub. Required on PUT (CreateOrReplace) requests."]
    #[serde(rename = "eventHubName", default, skip_serializing_if = "Option::is_none")]
    pub event_hub_name: Option<String>,
}
impl EventHubDataSourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an Event Hub output data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubOutputDataSource {
    #[serde(flatten)]
    pub output_data_source: OutputDataSource,
    #[doc = "The properties that are associated with an Event Hub output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EventHubOutputDataSourceProperties>,
}
impl EventHubOutputDataSource {
    pub fn new(output_data_source: OutputDataSource) -> Self {
        Self {
            output_data_source,
            properties: None,
        }
    }
}
#[doc = "The properties that are associated with an Event Hub output."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventHubOutputDataSourceProperties {
    #[serde(flatten)]
    pub event_hub_data_source_properties: EventHubDataSourceProperties,
    #[doc = "The key/column that is used to determine to which partition to send event data."]
    #[serde(rename = "partitionKey", default, skip_serializing_if = "Option::is_none")]
    pub partition_key: Option<String>,
    #[serde(
        rename = "propertyColumns",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub property_columns: Vec<String>,
}
impl EventHubOutputDataSourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an Event Hub input data source that contains stream data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubStreamInputDataSource {
    #[serde(flatten)]
    pub stream_input_data_source: StreamInputDataSource,
    #[doc = "The properties that are associated with a Event Hub input containing stream data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EventHubStreamInputDataSourceProperties>,
}
impl EventHubStreamInputDataSource {
    pub fn new(stream_input_data_source: StreamInputDataSource) -> Self {
        Self {
            stream_input_data_source,
            properties: None,
        }
    }
}
#[doc = "The properties that are associated with a Event Hub input containing stream data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventHubStreamInputDataSourceProperties {
    #[serde(flatten)]
    pub event_hub_data_source_properties: EventHubDataSourceProperties,
    #[doc = "The name of an Event Hub Consumer Group that should be used to read events from the Event Hub. Specifying distinct consumer group names for multiple inputs allows each of those inputs to receive the same events from the Event Hub. If not specified, the input uses the Event Hub’s default consumer group."]
    #[serde(rename = "consumerGroupName", default, skip_serializing_if = "Option::is_none")]
    pub consumer_group_name: Option<String>,
}
impl EventHubStreamInputDataSourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an Event Hub output data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubV2OutputDataSource {
    #[serde(flatten)]
    pub output_data_source: OutputDataSource,
    #[doc = "The properties that are associated with an Event Hub output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EventHubOutputDataSourceProperties>,
}
impl EventHubV2OutputDataSource {
    pub fn new(output_data_source: OutputDataSource) -> Self {
        Self {
            output_data_source,
            properties: None,
        }
    }
}
#[doc = "Describes an Event Hub input data source that contains stream data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubV2StreamInputDataSource {
    #[serde(flatten)]
    pub stream_input_data_source: StreamInputDataSource,
    #[doc = "The properties that are associated with a Event Hub input containing stream data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EventHubStreamInputDataSourceProperties>,
}
impl EventHubV2StreamInputDataSource {
    pub fn new(stream_input_data_source: StreamInputDataSource) -> Self {
        Self {
            stream_input_data_source,
            properties: None,
        }
    }
}
#[doc = "Indicates the type of serialization that the input or output uses. Required on PUT (CreateOrReplace) requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EventSerializationType")]
pub enum EventSerializationType {
    Csv,
    Avro,
    Json,
    CustomClr,
    Parquet,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EventSerializationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EventSerializationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EventSerializationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Csv => serializer.serialize_unit_variant("EventSerializationType", 0u32, "Csv"),
            Self::Avro => serializer.serialize_unit_variant("EventSerializationType", 1u32, "Avro"),
            Self::Json => serializer.serialize_unit_variant("EventSerializationType", 2u32, "Json"),
            Self::CustomClr => serializer.serialize_unit_variant("EventSerializationType", 3u32, "CustomClr"),
            Self::Parquet => serializer.serialize_unit_variant("EventSerializationType", 4u32, "Parquet"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Indicates the policy to apply to events that arrive out of order in the input event stream."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EventsOutOfOrderPolicy")]
pub enum EventsOutOfOrderPolicy {
    Adjust,
    Drop,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EventsOutOfOrderPolicy {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EventsOutOfOrderPolicy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EventsOutOfOrderPolicy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Adjust => serializer.serialize_unit_variant("EventsOutOfOrderPolicy", 0u32, "Adjust"),
            Self::Drop => serializer.serialize_unit_variant("EventsOutOfOrderPolicy", 1u32, "Drop"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The storage account where the custom code artifacts are located."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct External {
    #[doc = "The properties that are associated with an Azure Storage account"]
    #[serde(rename = "storageAccount", default, skip_serializing_if = "Option::is_none")]
    pub storage_account: Option<StorageAccount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl External {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A function object, containing all information associated with the named function. All functions are contained under a streaming job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Function {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "The properties that are associated with a function."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FunctionProperties>,
}
impl Function {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The physical binding of the function. For example, in the Azure Machine Learning web service’s case, this describes the endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionBinding {
    #[doc = "Indicates the function binding type."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl FunctionBinding {
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FunctionConfiguration {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub inputs: Vec<FunctionInput>,
    #[doc = "Describes the output of a function."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<FunctionOutput>,
    #[doc = "The physical binding of the function. For example, in the Azure Machine Learning web service’s case, this describes the endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binding: Option<FunctionBinding>,
}
impl FunctionConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes one input parameter of a function."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FunctionInput {
    #[doc = "The (Azure Stream Analytics supported) data type of the function input parameter. A list of valid Azure Stream Analytics data types are described at https://msdn.microsoft.com/en-us/library/azure/dn835065.aspx"]
    #[serde(rename = "dataType", default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[doc = "A flag indicating if the parameter is a configuration parameter. True if this input parameter is expected to be a constant. Default is false."]
    #[serde(rename = "isConfigurationParameter", default, skip_serializing_if = "Option::is_none")]
    pub is_configuration_parameter: Option<bool>,
}
impl FunctionInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object containing a list of functions under a streaming job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FunctionListResult {
    #[doc = "A list of functions under a streaming job. Populated by a 'List' operation."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Function>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for FunctionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl FunctionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the output of a function."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FunctionOutput {
    #[doc = "The (Azure Stream Analytics supported) data type of the function output. A list of valid Azure Stream Analytics data types are described at https://msdn.microsoft.com/en-us/library/azure/dn835065.aspx"]
    #[serde(rename = "dataType", default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
}
impl FunctionOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties that are associated with a function."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionProperties {
    #[doc = "Indicates the type of function."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "The current entity tag for the function. This is an opaque string. You can use it to detect whether the resource has changed between requests. You can also use it in the If-Match or If-None-Match headers for write operations for optimistic concurrency."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FunctionConfiguration>,
}
impl FunctionProperties {
    pub fn new(type_: String) -> Self {
        Self {
            type_,
            etag: None,
            properties: None,
        }
    }
}
#[doc = "Parameters used to specify the type of function to retrieve the default definition for."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionRetrieveDefaultDefinitionParameters {
    #[doc = "Indicates the function binding type."]
    #[serde(rename = "bindingType")]
    pub binding_type: String,
}
impl FunctionRetrieveDefaultDefinitionParameters {
    pub fn new(binding_type: String) -> Self {
        Self { binding_type }
    }
}
#[doc = "Describes how identity is verified"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Identity {
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Identity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An input object, containing all information associated with the named input. All inputs are contained under a streaming job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Input {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "The properties that are associated with an input."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<InputProperties>,
}
impl Input {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object containing a list of inputs under a streaming job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InputListResult {
    #[doc = "A list of inputs under a streaming job. Populated by a 'List' operation."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Input>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for InputListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl InputListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties that are associated with an input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputProperties {
    #[doc = "Indicates whether the input is a source of reference data or stream data. Required on PUT (CreateOrReplace) requests."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "Describes how data from an input is serialized or how data is serialized when written to an output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serialization: Option<Serialization>,
    #[doc = "Describes conditions applicable to the Input, Output, or the job overall, that warrant customer attention."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<Diagnostics>,
    #[doc = "The current entity tag for the input. This is an opaque string. You can use it to detect whether the resource has changed between requests. You can also use it in the If-Match or If-None-Match headers for write operations for optimistic concurrency."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Describes how input data is compressed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression: Option<Compression>,
    #[doc = "partitionKey Describes a key in the input data which is used for partitioning the input data"]
    #[serde(rename = "partitionKey", default, skip_serializing_if = "Option::is_none")]
    pub partition_key: Option<String>,
}
impl InputProperties {
    pub fn new(type_: String) -> Self {
        Self {
            type_,
            serialization: None,
            diagnostics: None,
            etag: None,
            compression: None,
            partition_key: None,
        }
    }
}
#[doc = "Describes an IoT Hub input data source that contains stream data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTHubStreamInputDataSource {
    #[serde(flatten)]
    pub stream_input_data_source: StreamInputDataSource,
    #[doc = "The properties that are associated with a IoT Hub input containing stream data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IoTHubStreamInputDataSourceProperties>,
}
impl IoTHubStreamInputDataSource {
    pub fn new(stream_input_data_source: StreamInputDataSource) -> Self {
        Self {
            stream_input_data_source,
            properties: None,
        }
    }
}
#[doc = "The properties that are associated with a IoT Hub input containing stream data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IoTHubStreamInputDataSourceProperties {
    #[doc = "The name or the URI of the IoT Hub. Required on PUT (CreateOrReplace) requests."]
    #[serde(rename = "iotHubNamespace", default, skip_serializing_if = "Option::is_none")]
    pub iot_hub_namespace: Option<String>,
    #[doc = "The shared access policy name for the IoT Hub. This policy must contain at least the Service connect permission. Required on PUT (CreateOrReplace) requests."]
    #[serde(rename = "sharedAccessPolicyName", default, skip_serializing_if = "Option::is_none")]
    pub shared_access_policy_name: Option<String>,
    #[doc = "The shared access policy key for the specified shared access policy. Required on PUT (CreateOrReplace) requests."]
    #[serde(rename = "sharedAccessPolicyKey", default, skip_serializing_if = "Option::is_none")]
    pub shared_access_policy_key: Option<String>,
    #[doc = "The name of an IoT Hub Consumer Group that should be used to read events from the IoT Hub. If not specified, the input uses the Iot Hub’s default consumer group."]
    #[serde(rename = "consumerGroupName", default, skip_serializing_if = "Option::is_none")]
    pub consumer_group_name: Option<String>,
    #[doc = "The IoT Hub endpoint to connect to (ie. messages/events, messages/operationsMonitoringEvents, etc.)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}
impl IoTHubStreamInputDataSourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The binding to a JavaScript function."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JavaScriptFunctionBinding {
    #[serde(flatten)]
    pub function_binding: FunctionBinding,
    #[doc = "The binding properties associated with a JavaScript function."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JavaScriptFunctionBindingProperties>,
}
impl JavaScriptFunctionBinding {
    pub fn new(function_binding: FunctionBinding) -> Self {
        Self {
            function_binding,
            properties: None,
        }
    }
}
#[doc = "The binding properties associated with a JavaScript function."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JavaScriptFunctionBindingProperties {
    #[doc = "The JavaScript code containing a single function definition. For example: 'function (x, y) { return x + y; }'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
}
impl JavaScriptFunctionBindingProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The binding retrieval properties associated with a JavaScript function."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JavaScriptFunctionBindingRetrievalProperties {
    #[doc = "The JavaScript code containing a single function definition. For example: 'function (x, y) { return x + y; }'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    #[doc = "The function type."]
    #[serde(rename = "udfType", default, skip_serializing_if = "Option::is_none")]
    pub udf_type: Option<UdfType>,
}
impl JavaScriptFunctionBindingRetrievalProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters needed to retrieve the default function definition for a JavaScript function."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JavaScriptFunctionRetrieveDefaultDefinitionParameters {
    #[serde(flatten)]
    pub function_retrieve_default_definition_parameters: FunctionRetrieveDefaultDefinitionParameters,
    #[doc = "The binding retrieval properties associated with a JavaScript function."]
    #[serde(rename = "bindingRetrievalProperties", default, skip_serializing_if = "Option::is_none")]
    pub binding_retrieval_properties: Option<JavaScriptFunctionBindingRetrievalProperties>,
}
impl JavaScriptFunctionRetrieveDefaultDefinitionParameters {
    pub fn new(function_retrieve_default_definition_parameters: FunctionRetrieveDefaultDefinitionParameters) -> Self {
        Self {
            function_retrieve_default_definition_parameters,
            binding_retrieval_properties: None,
        }
    }
}
#[doc = "The properties that are associated with an Azure Storage account with MSI"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobStorageAccount {
    #[serde(flatten)]
    pub storage_account: StorageAccount,
    #[doc = "Authentication Mode. Valid modes are `ConnectionString`, `Msi` and 'UserToken'."]
    #[serde(rename = "authenticationMode", default, skip_serializing_if = "Option::is_none")]
    pub authentication_mode: Option<AuthenticationMode>,
}
impl JobStorageAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the format of the JSON the output will be written in. The currently supported values are 'lineSeparated' indicating the output will be formatted by having each JSON object separated by a new line and 'array' indicating the output will be formatted as an array of JSON objects."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "JsonOutputSerializationFormat")]
pub enum JsonOutputSerializationFormat {
    LineSeparated,
    Array,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for JsonOutputSerializationFormat {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for JsonOutputSerializationFormat {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for JsonOutputSerializationFormat {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::LineSeparated => serializer.serialize_unit_variant("JsonOutputSerializationFormat", 0u32, "LineSeparated"),
            Self::Array => serializer.serialize_unit_variant("JsonOutputSerializationFormat", 1u32, "Array"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes how data from an input is serialized or how data is serialized when written to an output in JSON format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonSerialization {
    #[serde(flatten)]
    pub serialization: Serialization,
    #[doc = "The properties that are associated with the JSON serialization type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JsonSerializationProperties>,
}
impl JsonSerialization {
    pub fn new(serialization: Serialization) -> Self {
        Self {
            serialization,
            properties: None,
        }
    }
}
#[doc = "The properties that are associated with the JSON serialization type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JsonSerializationProperties {
    #[doc = "Specifies the encoding of the incoming data in the case of input and the encoding of outgoing data in the case of output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<Encoding>,
    #[doc = "Specifies the format of the JSON the output will be written in. The currently supported values are 'lineSeparated' indicating the output will be formatted by having each JSON object separated by a new line and 'array' indicating the output will be formatted as an array of JSON objects."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<JsonOutputSerializationFormat>,
}
impl JsonSerializationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties that are associated with data sources that use OAuth as their authentication model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OAuthBasedDataSourceProperties {
    #[doc = "A refresh token that can be used to obtain a valid access token that can then be used to authenticate with the data source. A valid refresh token is currently only obtainable via the Azure Portal. It is recommended to put a dummy string value here when creating the data source and then going to the Azure Portal to authenticate the data source which will update this property with a valid refresh token. Required on PUT (CreateOrReplace) requests."]
    #[serde(rename = "refreshToken", default, skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[doc = "The user principal name (UPN) of the user that was used to obtain the refresh token. Use this property to help remember which user was used to obtain the refresh token."]
    #[serde(rename = "tokenUserPrincipalName", default, skip_serializing_if = "Option::is_none")]
    pub token_user_principal_name: Option<String>,
    #[doc = "The user display name of the user that was used to obtain the refresh token. Use this property to help remember which user was used to obtain the refresh token."]
    #[serde(rename = "tokenUserDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub token_user_display_name: Option<String>,
}
impl OAuthBasedDataSourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Stream Analytics REST API operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "The name of the operation being performed on this particular object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Contains the localized display information for this particular operation / action."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Contains the localized display information for this particular operation / action."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "The localized friendly form of the resource provider name."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "The localized friendly form of the resource type related to this action/operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "The localized friendly name for the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "The localized friendly description for the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list Stream Analytics operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of Stream Analytics operations supported by the Microsoft.StreamAnalytics resource provider."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An output object, containing all information associated with the named output. All outputs are contained under a streaming job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Output {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "The properties that are associated with an output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OutputProperties>,
}
impl Output {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the data source that output will be written to."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputDataSource {
    #[doc = "Indicates the type of data source output will be written to. Required on PUT (CreateOrReplace) requests."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl OutputDataSource {
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}
#[doc = "Indicates the policy to apply to events that arrive at the output and cannot be written to the external storage due to being malformed (missing column values, column values of wrong type or size)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OutputErrorPolicy")]
pub enum OutputErrorPolicy {
    Stop,
    Drop,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OutputErrorPolicy {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OutputErrorPolicy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OutputErrorPolicy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Stop => serializer.serialize_unit_variant("OutputErrorPolicy", 0u32, "Stop"),
            Self::Drop => serializer.serialize_unit_variant("OutputErrorPolicy", 1u32, "Drop"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Object containing a list of outputs under a streaming job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OutputListResult {
    #[doc = "A list of outputs under a streaming job. Populated by a 'List' operation."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Output>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OutputListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OutputListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties that are associated with an output."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OutputProperties {
    #[doc = "Describes the data source that output will be written to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datasource: Option<OutputDataSource>,
    #[serde(rename = "timeWindow", default, skip_serializing_if = "Option::is_none")]
    pub time_window: Option<String>,
    #[doc = "The size window to constrain a Stream Analytics output to."]
    #[serde(rename = "sizeWindow", default, skip_serializing_if = "Option::is_none")]
    pub size_window: Option<i32>,
    #[doc = "Describes how data from an input is serialized or how data is serialized when written to an output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serialization: Option<Serialization>,
    #[doc = "Describes conditions applicable to the Input, Output, or the job overall, that warrant customer attention."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<Diagnostics>,
    #[doc = "The current entity tag for the output. This is an opaque string. You can use it to detect whether the resource has changed between requests. You can also use it in the If-Match or If-None-Match headers for write operations for optimistic concurrency."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl OutputProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Value may be JobStartTime, CustomTime, or LastOutputEventTime to indicate whether the starting point of the output event stream should start whenever the job is started, start at a custom user time stamp specified via the outputStartTime property, or start from the last event output time."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OutputStartMode")]
pub enum OutputStartMode {
    JobStartTime,
    CustomTime,
    LastOutputEventTime,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OutputStartMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OutputStartMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OutputStartMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::JobStartTime => serializer.serialize_unit_variant("OutputStartMode", 0u32, "JobStartTime"),
            Self::CustomTime => serializer.serialize_unit_variant("OutputStartMode", 1u32, "CustomTime"),
            Self::LastOutputEventTime => serializer.serialize_unit_variant("OutputStartMode", 2u32, "LastOutputEventTime"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes how data from an input is serialized or how data is serialized when written to an output in Parquet format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParquetSerialization {
    #[serde(flatten)]
    pub serialization: Serialization,
    #[doc = "The properties that are associated with the Parquet serialization type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ParquetSerializationProperties>,
}
impl ParquetSerialization {
    pub fn new(serialization: Serialization) -> Self {
        Self {
            serialization,
            properties: None,
        }
    }
}
#[doc = "The properties that are associated with the Parquet serialization type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ParquetSerializationProperties {}
impl ParquetSerializationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a Power BI output data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerBiOutputDataSource {
    #[serde(flatten)]
    pub output_data_source: OutputDataSource,
    #[doc = "The properties that are associated with a Power BI output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PowerBiOutputDataSourceProperties>,
}
impl PowerBiOutputDataSource {
    pub fn new(output_data_source: OutputDataSource) -> Self {
        Self {
            output_data_source,
            properties: None,
        }
    }
}
#[doc = "The properties that are associated with a Power BI output."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PowerBiOutputDataSourceProperties {
    #[serde(flatten)]
    pub o_auth_based_data_source_properties: OAuthBasedDataSourceProperties,
    #[doc = "The name of the Power BI dataset. Required on PUT (CreateOrReplace) requests."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dataset: Option<String>,
    #[doc = "The name of the Power BI table under the specified dataset. Required on PUT (CreateOrReplace) requests."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[doc = "The ID of the Power BI group."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The name of the Power BI group. Use this property to help remember which specific Power BI group id was used."]
    #[serde(rename = "groupName", default, skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[doc = "Authentication Mode. Valid modes are `ConnectionString`, `Msi` and 'UserToken'."]
    #[serde(rename = "authenticationMode", default, skip_serializing_if = "Option::is_none")]
    pub authentication_mode: Option<AuthenticationMode>,
}
impl PowerBiOutputDataSourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error produced by the compiler."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryCompilationError {
    #[doc = "The content of the error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Describes the error location in the original query. Not set if isGlobal is true."]
    #[serde(rename = "startLine", default, skip_serializing_if = "Option::is_none")]
    pub start_line: Option<i32>,
    #[doc = "Describes the error location in the original query. Not set if isGlobal is true."]
    #[serde(rename = "startColumn", default, skip_serializing_if = "Option::is_none")]
    pub start_column: Option<i32>,
    #[doc = "Describes the error location in the original query. Not set if isGlobal is true."]
    #[serde(rename = "endLine", default, skip_serializing_if = "Option::is_none")]
    pub end_line: Option<i32>,
    #[doc = "Describes the error location in the original query. Not set if isGlobal is true."]
    #[serde(rename = "endColumn", default, skip_serializing_if = "Option::is_none")]
    pub end_column: Option<i32>,
    #[doc = "Whether the error is not for a specific part but for the entire query."]
    #[serde(rename = "isGlobal", default, skip_serializing_if = "Option::is_none")]
    pub is_global: Option<bool>,
}
impl QueryCompilationError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of the query compilation request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryCompilationResult {
    #[doc = "Error messages produced by the compiler."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<QueryCompilationError>,
    #[doc = "Warning messages produced by the compiler."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub warnings: Vec<String>,
    #[doc = "All input names used by the query."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub inputs: Vec<String>,
    #[doc = "All output names used by the query."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub outputs: Vec<String>,
    #[doc = "All function names used by the query."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub functions: Vec<String>,
}
impl QueryCompilationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A function for the query compilation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryFunction {
    #[doc = "The name of the function."]
    pub name: String,
    #[doc = "The type of the function."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "The type of the function binding."]
    #[serde(rename = "bindingType")]
    pub binding_type: String,
    #[doc = "The inputs for the function."]
    pub inputs: Vec<FunctionInput>,
    #[doc = "Describes the output of a function."]
    pub output: FunctionOutput,
}
impl QueryFunction {
    pub fn new(name: String, type_: String, binding_type: String, inputs: Vec<FunctionInput>, output: FunctionOutput) -> Self {
        Self {
            name,
            type_,
            binding_type,
            inputs,
            output,
        }
    }
}
#[doc = "An input for the query compilation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryInput {
    #[doc = "The name of the input."]
    pub name: String,
    #[doc = "The type of the input, can be Stream or Reference."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl QueryInput {
    pub fn new(name: String, type_: String) -> Self {
        Self { name, type_ }
    }
}
#[doc = "The result of the query testing request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryTestingResult {
    #[serde(flatten)]
    pub error: Error,
    #[doc = "The status of the query testing request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<QueryTestingResultStatus>,
    #[doc = "The SAS URL to the outputs payload."]
    #[serde(rename = "outputUri", default, skip_serializing_if = "Option::is_none")]
    pub output_uri: Option<String>,
}
impl QueryTestingResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The status of the query testing request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "QueryTestingResultStatus")]
pub enum QueryTestingResultStatus {
    Started,
    Success,
    CompilerError,
    RuntimeError,
    Timeout,
    UnknownError,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for QueryTestingResultStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for QueryTestingResultStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for QueryTestingResultStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Started => serializer.serialize_unit_variant("QueryTestingResultStatus", 0u32, "Started"),
            Self::Success => serializer.serialize_unit_variant("QueryTestingResultStatus", 1u32, "Success"),
            Self::CompilerError => serializer.serialize_unit_variant("QueryTestingResultStatus", 2u32, "CompilerError"),
            Self::RuntimeError => serializer.serialize_unit_variant("QueryTestingResultStatus", 3u32, "RuntimeError"),
            Self::Timeout => serializer.serialize_unit_variant("QueryTestingResultStatus", 4u32, "Timeout"),
            Self::UnknownError => serializer.serialize_unit_variant("QueryTestingResultStatus", 5u32, "UnknownError"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The properties that are associated with a raw input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RawInputDatasourceProperties {
    #[doc = "The JSON serialized content of the input data. Either payload or payloadUri must be set, but not both. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    #[doc = "The SAS URL to a blob containing the JSON serialized content of the input data. Either payload or payloadUri must be set, but not both."]
    #[serde(rename = "payloadUri", default, skip_serializing_if = "Option::is_none")]
    pub payload_uri: Option<String>,
}
impl RawInputDatasourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a raw output data source. This data source type is only applicable/usable when using the query testing API. You cannot create a job with this data source type or add an output of this data source type to an existing job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RawOutputDatasource {
    #[serde(flatten)]
    pub output_data_source: OutputDataSource,
    #[doc = "The properties that are associated with a raw output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RawOutputDatasourceProperties>,
}
impl RawOutputDatasource {
    pub fn new(output_data_source: OutputDataSource) -> Self {
        Self {
            output_data_source,
            properties: None,
        }
    }
}
#[doc = "The properties that are associated with a raw output."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RawOutputDatasourceProperties {
    #[doc = "The SAS URL to a blob where the output should be written. If this property is not set, output data will be written into a temporary storage, and a SAS URL to that temporary storage will be included in the result."]
    #[serde(rename = "payloadUri", default, skip_serializing_if = "Option::is_none")]
    pub payload_uri: Option<String>,
}
impl RawOutputDatasourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a raw input data source that contains reference data. This data source type is only applicable/usable when using the query testing API. You cannot create a job with this data source type or add an input of this data source type to an existing job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RawReferenceInputDataSource {
    #[serde(flatten)]
    pub reference_input_data_source: ReferenceInputDataSource,
    #[doc = "The properties that are associated with a raw input."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RawInputDatasourceProperties>,
}
impl RawReferenceInputDataSource {
    pub fn new(reference_input_data_source: ReferenceInputDataSource) -> Self {
        Self {
            reference_input_data_source,
            properties: None,
        }
    }
}
#[doc = "Describes a raw input data source that contains stream data. This data source type is only applicable/usable when using the query testing API. You cannot create a job with this data source type or add an input of this data source type to an existing job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RawStreamInputDataSource {
    #[serde(flatten)]
    pub stream_input_data_source: StreamInputDataSource,
    #[doc = "The properties that are associated with a raw input."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RawInputDatasourceProperties>,
}
impl RawStreamInputDataSource {
    pub fn new(stream_input_data_source: StreamInputDataSource) -> Self {
        Self {
            stream_input_data_source,
            properties: None,
        }
    }
}
#[doc = "Describes an input data source that contains reference data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceInputDataSource {
    #[doc = "Indicates the type of input data source containing reference data. Required on PUT (CreateOrReplace) requests."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl ReferenceInputDataSource {
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}
#[doc = "The properties that are associated with an input containing reference data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceInputProperties {
    #[serde(flatten)]
    pub input_properties: InputProperties,
    #[doc = "Describes an input data source that contains reference data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datasource: Option<ReferenceInputDataSource>,
}
impl ReferenceInputProperties {
    pub fn new(input_properties: InputProperties) -> Self {
        Self {
            input_properties,
            datasource: None,
        }
    }
}
#[doc = "The base resource definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource Id for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. Ex- Microsoft.Compute/virtualMachines or Microsoft.Storage/storageAccounts."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the status of the test operation along with error information, if applicable."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceTestStatus {
    #[doc = "The status of the test operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Describes the error that occurred."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
impl ResourceTestStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The stream analytics input to sample."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SampleInput {
    #[doc = "An input object, containing all information associated with the named input. All inputs are contained under a streaming job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<Input>,
    #[doc = "Defaults to the default ASA job compatibility level. Today it is 1.2"]
    #[serde(rename = "compatibilityLevel", default, skip_serializing_if = "Option::is_none")]
    pub compatibility_level: Option<String>,
    #[doc = "The SAS URI of the storage blob for service to write the sampled events to. If this parameter is not provided, service will write events to he system account and share a temporary SAS URI to it."]
    #[serde(rename = "eventsUri", default, skip_serializing_if = "Option::is_none")]
    pub events_uri: Option<String>,
    #[doc = "Defaults to en-US."]
    #[serde(rename = "dataLocale", default, skip_serializing_if = "Option::is_none")]
    pub data_locale: Option<String>,
}
impl SampleInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of the sample input request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SampleInputResult {
    #[serde(flatten)]
    pub error: Error,
    #[doc = "The status of the sample input request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SampleInputResultStatus>,
    #[doc = "Diagnostics messages. E.g. message indicating some partitions from the input have no data."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub diagnostics: Vec<String>,
    #[doc = "A SAS URL to download the sampled input data."]
    #[serde(rename = "eventsDownloadUrl", default, skip_serializing_if = "Option::is_none")]
    pub events_download_url: Option<String>,
    #[doc = "The timestamp for the last event in the data. It is in DateTime format."]
    #[serde(rename = "lastArrivalTime", default, skip_serializing_if = "Option::is_none")]
    pub last_arrival_time: Option<String>,
}
impl SampleInputResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The status of the sample input request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SampleInputResultStatus")]
pub enum SampleInputResultStatus {
    ReadAllEventsInRange,
    NoEventsFoundInRange,
    ErrorConnectingToInput,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SampleInputResultStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SampleInputResultStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SampleInputResultStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ReadAllEventsInRange => serializer.serialize_unit_variant("SampleInputResultStatus", 0u32, "ReadAllEventsInRange"),
            Self::NoEventsFoundInRange => serializer.serialize_unit_variant("SampleInputResultStatus", 1u32, "NoEventsFoundInRange"),
            Self::ErrorConnectingToInput => serializer.serialize_unit_variant("SampleInputResultStatus", 2u32, "ErrorConnectingToInput"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The properties that are associated with a scalar function."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScalarFunctionProperties {
    #[serde(flatten)]
    pub function_properties: FunctionProperties,
}
impl ScalarFunctionProperties {
    pub fn new(function_properties: FunctionProperties) -> Self {
        Self { function_properties }
    }
}
#[doc = "Describes how data from an input is serialized or how data is serialized when written to an output."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Serialization {
    #[doc = "Indicates the type of serialization that the input or output uses. Required on PUT (CreateOrReplace) requests."]
    #[serde(rename = "type")]
    pub type_: EventSerializationType,
}
impl Serialization {
    pub fn new(type_: EventSerializationType) -> Self {
        Self { type_ }
    }
}
#[doc = "The common properties that are associated with Service Bus data sources (Queues, Topics, Event Hubs, etc.)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceBusDataSourceProperties {
    #[doc = "The namespace that is associated with the desired Event Hub, Service Bus Queue, Service Bus Topic, etc. Required on PUT (CreateOrReplace) requests."]
    #[serde(rename = "serviceBusNamespace", default, skip_serializing_if = "Option::is_none")]
    pub service_bus_namespace: Option<String>,
    #[doc = "The shared access policy name for the Event Hub, Service Bus Queue, Service Bus Topic, etc. Required on PUT (CreateOrReplace) requests."]
    #[serde(rename = "sharedAccessPolicyName", default, skip_serializing_if = "Option::is_none")]
    pub shared_access_policy_name: Option<String>,
    #[doc = "The shared access policy key for the specified shared access policy. Required on PUT (CreateOrReplace) requests."]
    #[serde(rename = "sharedAccessPolicyKey", default, skip_serializing_if = "Option::is_none")]
    pub shared_access_policy_key: Option<String>,
    #[doc = "Authentication Mode. Valid modes are `ConnectionString`, `Msi` and 'UserToken'."]
    #[serde(rename = "authenticationMode", default, skip_serializing_if = "Option::is_none")]
    pub authentication_mode: Option<AuthenticationMode>,
}
impl ServiceBusDataSourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a Service Bus Queue output data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBusQueueOutputDataSource {
    #[serde(flatten)]
    pub output_data_source: OutputDataSource,
    #[doc = "The properties that are associated with a Service Bus Queue output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServiceBusQueueOutputDataSourceProperties>,
}
impl ServiceBusQueueOutputDataSource {
    pub fn new(output_data_source: OutputDataSource) -> Self {
        Self {
            output_data_source,
            properties: None,
        }
    }
}
#[doc = "The properties that are associated with a Service Bus Queue output."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceBusQueueOutputDataSourceProperties {
    #[serde(flatten)]
    pub service_bus_data_source_properties: ServiceBusDataSourceProperties,
    #[doc = "The name of the Service Bus Queue. Required on PUT (CreateOrReplace) requests."]
    #[serde(rename = "queueName", default, skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,
    #[doc = "A string array of the names of output columns to be attached to Service Bus messages as custom properties."]
    #[serde(
        rename = "propertyColumns",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub property_columns: Vec<String>,
    #[serde(rename = "systemPropertyColumns", default, skip_serializing_if = "Option::is_none")]
    pub system_property_columns: Option<serde_json::Value>,
}
impl ServiceBusQueueOutputDataSourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a Service Bus Topic output data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBusTopicOutputDataSource {
    #[serde(flatten)]
    pub output_data_source: OutputDataSource,
    #[doc = "The properties that are associated with a Service Bus Topic output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServiceBusTopicOutputDataSourceProperties>,
}
impl ServiceBusTopicOutputDataSource {
    pub fn new(output_data_source: OutputDataSource) -> Self {
        Self {
            output_data_source,
            properties: None,
        }
    }
}
#[doc = "The properties that are associated with a Service Bus Topic output."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceBusTopicOutputDataSourceProperties {
    #[serde(flatten)]
    pub service_bus_data_source_properties: ServiceBusDataSourceProperties,
    #[doc = "The name of the Service Bus Topic. Required on PUT (CreateOrReplace) requests."]
    #[serde(rename = "topicName", default, skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    #[doc = "A string array of the names of output columns to be attached to Service Bus messages as custom properties."]
    #[serde(
        rename = "propertyColumns",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub property_columns: Vec<String>,
    #[serde(rename = "systemPropertyColumns", default, skip_serializing_if = "Option::is_none")]
    pub system_property_columns: Option<serde_json::Value>,
}
impl ServiceBusTopicOutputDataSourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the Start Streaming Job operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StartStreamingJobParameters {
    #[doc = "Value may be JobStartTime, CustomTime, or LastOutputEventTime to indicate whether the starting point of the output event stream should start whenever the job is started, start at a custom user time stamp specified via the outputStartTime property, or start from the last event output time."]
    #[serde(rename = "outputStartMode", default, skip_serializing_if = "Option::is_none")]
    pub output_start_mode: Option<OutputStartMode>,
    #[doc = "Value is either an ISO-8601 formatted time stamp that indicates the starting point of the output event stream, or null to indicate that the output event stream will start whenever the streaming job is started. This property must have a value if outputStartMode is set to CustomTime."]
    #[serde(rename = "outputStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub output_start_time: Option<time::OffsetDateTime>,
}
impl StartStreamingJobParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties that are associated with an Azure Storage account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccount {
    #[doc = "The name of the Azure Storage account. Required on PUT (CreateOrReplace) requests."]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[doc = "The account key for the Azure Storage account. Required on PUT (CreateOrReplace) requests."]
    #[serde(rename = "accountKey", default, skip_serializing_if = "Option::is_none")]
    pub account_key: Option<String>,
}
impl StorageAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an input data source that contains stream data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StreamInputDataSource {
    #[doc = "Indicates the type of input data source containing stream data. Required on PUT (CreateOrReplace) requests."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl StreamInputDataSource {
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}
#[doc = "The properties that are associated with an input containing stream data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StreamInputProperties {
    #[serde(flatten)]
    pub input_properties: InputProperties,
    #[doc = "Describes an input data source that contains stream data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datasource: Option<StreamInputDataSource>,
}
impl StreamInputProperties {
    pub fn new(input_properties: InputProperties) -> Self {
        Self {
            input_properties,
            datasource: None,
        }
    }
}
#[doc = "A streaming job object, containing all information associated with the named streaming job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StreamingJob {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The properties that are associated with a streaming job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StreamingJobProperties>,
    #[doc = "Describes how identity is verified"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
}
impl StreamingJob {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object containing a list of streaming jobs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StreamingJobListResult {
    #[doc = "A list of streaming jobs. Populated by a 'List' operation."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<StreamingJob>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StreamingJobListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl StreamingJobListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties that are associated with a streaming job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StreamingJobProperties {
    #[doc = "The properties that are associated with a SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<StreamingJobSku>,
    #[doc = "A GUID uniquely identifying the streaming job. This GUID is generated upon creation of the streaming job."]
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[doc = "Describes the provisioning status of the streaming job."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Describes the state of the streaming job."]
    #[serde(rename = "jobState", default, skip_serializing_if = "Option::is_none")]
    pub job_state: Option<String>,
    #[doc = "Describes the type of the job. Valid modes are `Cloud` and 'Edge'."]
    #[serde(rename = "jobType", default, skip_serializing_if = "Option::is_none")]
    pub job_type: Option<streaming_job_properties::JobType>,
    #[doc = "Value may be JobStartTime, CustomTime, or LastOutputEventTime to indicate whether the starting point of the output event stream should start whenever the job is started, start at a custom user time stamp specified via the outputStartTime property, or start from the last event output time."]
    #[serde(rename = "outputStartMode", default, skip_serializing_if = "Option::is_none")]
    pub output_start_mode: Option<OutputStartMode>,
    #[doc = "Value is either an ISO-8601 formatted time stamp that indicates the starting point of the output event stream, or null to indicate that the output event stream will start whenever the streaming job is started. This property must have a value if outputStartMode is set to CustomTime."]
    #[serde(rename = "outputStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub output_start_time: Option<time::OffsetDateTime>,
    #[doc = "Value is either an ISO-8601 formatted timestamp indicating the last output event time of the streaming job or null indicating that output has not yet been produced. In case of multiple outputs or multiple streams, this shows the latest value in that set."]
    #[serde(rename = "lastOutputEventTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_output_event_time: Option<time::OffsetDateTime>,
    #[doc = "Indicates the policy to apply to events that arrive out of order in the input event stream."]
    #[serde(rename = "eventsOutOfOrderPolicy", default, skip_serializing_if = "Option::is_none")]
    pub events_out_of_order_policy: Option<EventsOutOfOrderPolicy>,
    #[doc = "Indicates the policy to apply to events that arrive at the output and cannot be written to the external storage due to being malformed (missing column values, column values of wrong type or size)."]
    #[serde(rename = "outputErrorPolicy", default, skip_serializing_if = "Option::is_none")]
    pub output_error_policy: Option<OutputErrorPolicy>,
    #[doc = "The maximum tolerable delay in seconds where out-of-order events can be adjusted to be back in order."]
    #[serde(rename = "eventsOutOfOrderMaxDelayInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub events_out_of_order_max_delay_in_seconds: Option<i32>,
    #[doc = "The maximum tolerable delay in seconds where events arriving late could be included.  Supported range is -1 to 1814399 (20.23:59:59 days) and -1 is used to specify wait indefinitely. If the property is absent, it is interpreted to have a value of -1."]
    #[serde(rename = "eventsLateArrivalMaxDelayInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub events_late_arrival_max_delay_in_seconds: Option<i32>,
    #[doc = "The data locale of the stream analytics job. Value should be the name of a supported .NET Culture from the set https://msdn.microsoft.com/en-us/library/system.globalization.culturetypes(v=vs.110).aspx. Defaults to 'en-US' if none specified."]
    #[serde(rename = "dataLocale", default, skip_serializing_if = "Option::is_none")]
    pub data_locale: Option<String>,
    #[doc = "Controls certain runtime behaviors of the streaming job."]
    #[serde(rename = "compatibilityLevel", default, skip_serializing_if = "Option::is_none")]
    pub compatibility_level: Option<CompatibilityLevel>,
    #[doc = "Value is an ISO-8601 formatted UTC timestamp indicating when the streaming job was created."]
    #[serde(rename = "createdDate", default, with = "azure_core::date::rfc3339::option")]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "A list of one or more inputs to the streaming job. The name property for each input is required when specifying this property in a PUT request. This property cannot be modify via a PATCH operation. You must use the PATCH API available for the individual input."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub inputs: Vec<Input>,
    #[doc = "A transformation object, containing all information associated with the named transformation. All transformations are contained under a streaming job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transformation: Option<Transformation>,
    #[doc = "A list of one or more outputs for the streaming job. The name property for each output is required when specifying this property in a PUT request. This property cannot be modify via a PATCH operation. You must use the PATCH API available for the individual output."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub outputs: Vec<Output>,
    #[doc = "A list of one or more functions for the streaming job. The name property for each function is required when specifying this property in a PUT request. This property cannot be modify via a PATCH operation. You must use the PATCH API available for the individual transformation."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub functions: Vec<Function>,
    #[doc = "The current entity tag for the streaming job. This is an opaque string. You can use it to detect whether the resource has changed between requests. You can also use it in the If-Match or If-None-Match headers for write operations for optimistic concurrency."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "The properties that are associated with an Azure Storage account with MSI"]
    #[serde(rename = "jobStorageAccount", default, skip_serializing_if = "Option::is_none")]
    pub job_storage_account: Option<JobStorageAccount>,
    #[doc = "Valid values are JobStorageAccount and SystemAccount. If set to JobStorageAccount, this requires the user to also specify jobStorageAccount property. ."]
    #[serde(rename = "contentStoragePolicy", default, skip_serializing_if = "Option::is_none")]
    pub content_storage_policy: Option<streaming_job_properties::ContentStoragePolicy>,
    #[doc = "The storage account where the custom code artifacts are located."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub externals: Option<External>,
    #[doc = "The properties associated with a Stream Analytics cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster: Option<ClusterInfo>,
}
impl StreamingJobProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod streaming_job_properties {
    use super::*;
    #[doc = "Describes the type of the job. Valid modes are `Cloud` and 'Edge'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "JobType")]
    pub enum JobType {
        Cloud,
        Edge,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for JobType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for JobType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for JobType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Cloud => serializer.serialize_unit_variant("JobType", 0u32, "Cloud"),
                Self::Edge => serializer.serialize_unit_variant("JobType", 1u32, "Edge"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Valid values are JobStorageAccount and SystemAccount. If set to JobStorageAccount, this requires the user to also specify jobStorageAccount property. ."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ContentStoragePolicy")]
    pub enum ContentStoragePolicy {
        SystemAccount,
        JobStorageAccount,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ContentStoragePolicy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ContentStoragePolicy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ContentStoragePolicy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SystemAccount => serializer.serialize_unit_variant("ContentStoragePolicy", 0u32, "SystemAccount"),
                Self::JobStorageAccount => serializer.serialize_unit_variant("ContentStoragePolicy", 1u32, "JobStorageAccount"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties that are associated with a SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StreamingJobSku {
    #[doc = "The name of the SKU. Required on PUT (CreateOrReplace) requests."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<streaming_job_sku::Name>,
}
impl StreamingJobSku {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod streaming_job_sku {
    use super::*;
    #[doc = "The name of the SKU. Required on PUT (CreateOrReplace) requests."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        Standard,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Name {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Name {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Name {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Standard => serializer.serialize_unit_variant("Name", 0u32, "Standard"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The base sub-resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubResource {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl SubResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the current quota for the subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionQuota {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Describes the properties of the quota."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<subscription_quota::Properties>,
}
impl SubscriptionQuota {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod subscription_quota {
    use super::*;
    #[doc = "Describes the properties of the quota."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The max permitted usage of this resource."]
        #[serde(rename = "maxCount", default, skip_serializing_if = "Option::is_none")]
        pub max_count: Option<i32>,
        #[doc = "The current usage of this resource."]
        #[serde(rename = "currentCount", default, skip_serializing_if = "Option::is_none")]
        pub current_count: Option<i32>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the GetQuotas operation. It contains a list of quotas for the subscription in a particular region."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionQuotasListResult {
    #[doc = "List of quotas for the subscription in a particular region."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SubscriptionQuota>,
}
impl SubscriptionQuotasListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of the test input or output request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestDatasourceResult {
    #[serde(flatten)]
    pub error: Error,
    #[doc = "The status of the test input or output request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<TestDatasourceResultStatus>,
}
impl TestDatasourceResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The status of the test input or output request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TestDatasourceResultStatus")]
pub enum TestDatasourceResultStatus {
    TestSucceeded,
    TestFailed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TestDatasourceResultStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TestDatasourceResultStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TestDatasourceResultStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::TestSucceeded => serializer.serialize_unit_variant("TestDatasourceResultStatus", 0u32, "TestSucceeded"),
            Self::TestFailed => serializer.serialize_unit_variant("TestDatasourceResultStatus", 1u32, "TestFailed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A stream analytics input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestInput {
    #[doc = "An input object, containing all information associated with the named input. All inputs are contained under a streaming job."]
    pub input: Input,
}
impl TestInput {
    pub fn new(input: Input) -> Self {
        Self { input }
    }
}
#[doc = "A stream analytics output."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestOutput {
    #[doc = "An output object, containing all information associated with the named output. All outputs are contained under a streaming job."]
    pub output: Output,
}
impl TestOutput {
    pub fn new(output: Output) -> Self {
        Self { output }
    }
}
#[doc = "The request object for query testing."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestQuery {
    #[doc = "Diagnostics information related to query testing."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<test_query::Diagnostics>,
    #[doc = "A streaming job object, containing all information associated with the named streaming job."]
    #[serde(rename = "streamingJob")]
    pub streaming_job: StreamingJob,
}
impl TestQuery {
    pub fn new(streaming_job: StreamingJob) -> Self {
        Self {
            diagnostics: None,
            streaming_job,
        }
    }
}
pub mod test_query {
    use super::*;
    #[doc = "Diagnostics information related to query testing."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Diagnostics {
        #[doc = "The SAS URI to the container or directory."]
        #[serde(rename = "writeUri")]
        pub write_uri: String,
        #[doc = "The path to the subdirectory."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub path: Option<String>,
    }
    impl Diagnostics {
        pub fn new(write_uri: String) -> Self {
            Self { write_uri, path: None }
        }
    }
}
#[doc = "The resource model definition for a ARM tracked top level resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The geo-location where the resource lives"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl TrackedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A transformation object, containing all information associated with the named transformation. All transformations are contained under a streaming job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Transformation {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "The properties that are associated with a transformation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TransformationProperties>,
}
impl Transformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties that are associated with a transformation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransformationProperties {
    #[doc = "Specifies the number of streaming units that the streaming job uses."]
    #[serde(rename = "streamingUnits", default, skip_serializing_if = "Option::is_none")]
    pub streaming_units: Option<i32>,
    #[doc = "Specifies the query that will be run in the streaming job. You can learn more about the Stream Analytics Query Language (SAQL) here: https://msdn.microsoft.com/library/azure/dn834998 . Required on PUT (CreateOrReplace) requests."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[doc = "The current entity tag for the transformation. This is an opaque string. You can use it to detect whether the resource has changed between requests. You can also use it in the If-Match or If-None-Match headers for write operations for optimistic concurrency."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl TransformationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The function type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum UdfType {
    Scalar,
}
