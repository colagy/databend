---
title: Databend Query Configuration
sidebar_label: Databend Query Configuration
description:
  Databend Query Configuration
---

A `databend-query` server is configured with a `toml` file or flags.

You can explore more flags with `./databend-query -h`.

## 1. Logging Config

### log.file

  * on: Enables or disables `file` logging. Defaults to `true`.
  * dir: Path to store log files.
  * level: Log level (DEBUG | INFO | ERROR). Defaults to `INFO`.
  * format: Log format. Defaults to `json`.
    - `json`: Databend outputs logs in JSON format.
    - `text`: Databend outputs plain text logs.

### log.stderr

  * on: Enables or disables `stderr` logging. Defaults to `false`.
  * level: Log level (DEBUG | INFO | ERROR). Defaults to `DEBUG`.
  * format: Log format. Defaults to `text`.
    - `text`: Databend outputs plain text logs.
    - `json`: Databend outputs logs in JSON format.

## 2. Meta Service Config

### address

* Meta service endpoint, which lets databend-query connect to get meta data, e.g., `192.168.0.1:9101`.
* Default: `""`
* Env variable: `META_ADDRESS`
* Required.

### username

* Meta service username when connecting to it.
* Default: `"root"`
* Env variable: `META_USERNAME`

### password

* Meta service password when connecting to it, `password` is not recommended here, please use Env variable instead.
* Default: `"root"`
* Env variable: `META_PASSWORD`

### endpoints

* A list of meta server endpoints that databend-query can connect to(as seeds), e.g., `["192.168.0.1:9101", "192.168.0.2:9101"]`.
* Default: `[""]`
* Env variable: `META_ENDPOINTS`

:::tip
If `endpoints` is configured, the `address` configuration will no longer take effect, you only need to configure one of them.
:::

## 3. Query config

### admin_api_address

* The IP address and port to listen on for admin the databend-query, e.g., `0.0.0.0::8080`.
* Default: `"127.0.0.1:8080"`
* Env variable: `QUERY_ADMIN_API_ADDRESS`

### metric_api_address

* The IP address and port to listen on that can be scraped by Prometheus, e.g., `0.0.0.0::7070`.
* Default: `"127.0.0.1:7070"`
* Env variable: `QUERY_METRIC_API_ADDRESS`

### flight_api_address

* The IP address and port to listen on for databend-query cluster shuffle data, e.g., `0.0.0.0::9090`.
* Default: `"127.0.0.1:9090"`
* Env variable: `QUERY_FLIGHT_API_ADDRESS`

### mysql_handler_host

* The IP address to listen on for MySQL handler, e.g., `0.0.0.0`.
* Default: `"127.0.0.1"`
* Env variable: `QUERY_MYSQL_HANDLER_HOST`

### mysql_handler_port

* The port to listen on for MySQL handler, e.g., `3307`.
* Default: `3307`
* Env variable: `QUERY_MYSQL_HANDLER_PORT`

### clickhouse_handler_host

* The IP address to listen on for ClickHouse handler, e.g., `0.0.0.0`.
* Default: `"127.0.0.1"`
* Env variable: `QUERY_CLICKHOUSE_HANDLER_HOST`

### clickhouse_http_handler_host

* The IP address to listen on for ClickHouse HTTP handler, e.g., `0.0.0.0`.
* Default: `"127.0.0.1"`
* Env variable: `QUERY_CLICKHOUSE_HTTP_HANDLER_HOST`

### clickhouse_http_handler_port

* The port to listen on for ClickHouse HTTP handler, e.g., `8124`.
* Default: `8124`
* Env variable: `QUERY_CLICKHOUSE_HTTP_HANDLER_PORT`

### tenant_id

* The ID for the databend-query server to store metadata to the Meta Service.
* Default: `"admin"`
* Env variable: `QUERY_TENANT_ID`

### cluster_id

* The ID for the databend-query server to construct a cluster.
* Default: `""`
* Env variable: `QUERY_CLUSTER_ID`


## 4. Storage config

### type

* Which storage type(Must one of `"fs"` | `"s3"` | `"azblob"` | `"obs"`) should use for the databend-query, e.g., `"s3"`.
* Default: `""`
* Env variable: `STORAGE_TYPE`
* Required.

### storage.s3

#### bucket

* AWS S3 bucket name.
* Default: `""`
* Env variable: `STORAGE_S3_BUCKET`
* Required.

#### endpoint_url

* AWS S3(or MinIO S3-like) endpoint URL, e.g., `"https://s3.amazonaws.com"`.
* Default: `"https://s3.amazonaws.com"`
* Env variable: `STORAGE_S3_ENDPOINT_URL`

#### access_key_id

* AWS S3 access_key_id.
* Default: `""`
* Env variable: `STORAGE_S3_ACCESS_KEY_ID`
* Required.

#### secret_access_key

* AWS S3 secret_access_key.
* Default: `""`
* Env variable: `STORAGE_S3SECRET_ACCESS_KEY`
* Required.

### storage.azblob

#### endpoint_url

* Azure Blob Storage endpoint URL, e.g., `"https://<your-storage-account-name>.blob.core.windows.net"`.
* Default: `""`
* Env variable: `STORAGE_AZBLOB_ENDPOINT_URL`
* Required.

#### container

* Azure Blob Storage container name.
* Default: `""`
* Env variable: `STORAGE_AZBLOB_CONTAINER`
* Required.

#### account_name

* Azure Blob Storage account name.
* Default: `""`
* Env variable: `STORAGE_AZBLOB_ACCOUNT_NAME`
* Required.

#### account_key

* Azure Blob Storage account key.
* Default: `""`
* Env variable: `STORAGE_AZBLOB_ACCOUNT_KEY`
* Required.

### storage.obs

#### bucket

* OBS bucket name.
* Default: `""`
* Env variable: `STORAGE_OBS_BUCKET`
* Required.

#### endpoint_url

* OBS endpoint URL, e.g., `"https://obs.cn-north-4.myhuaweicloud.com"`.
* Default: `""`
* Env variable: `STORAGE_OBS_ENDPOINT_URL`
* Required.

#### access_key_id

* OBS access_key_id.
* Default: `""`
* Env variable: `STORAGE_OBS_ACCESS_KEY_ID`
* Required.

#### secret_access_key

* OBS secret_access_key.
* Default: `""`
* Env variable: `STORAGE_OBS_SECRET_ACCESS_KEY`
* Required.

## A Toml File Demo

For ease of experience, set all hosts to 0.0.0.0. Exercise caution when setting host if the application is in production.

```toml title="databend-query.toml"
# Logging
[log.file]
on = true
dir = "./.datanend/logs"
level = "INFO"
format = "json"

[log.stderr]
on = false
level = "DEBUG"
format = "text"

# Meta Service
[meta]
address = "0.0.0.0:9101"
username = "root"
password = "root"

[query]
# For admin RESET API.
admin_api_address = "0.0.0.0:8001"

# Metrics.
metric_api_address = "0.0.0.0:7071"

# Cluster flight RPC.
flight_api_address = "0.0.0.0:9091"

# Query MySQL Handler.
mysql_handler_host = "0.0.0.0"
mysql_handler_port = 3307

# Query ClickHouse Handler.
clickhouse_handler_host = "0.0.0.0"
clickhouse_handler_port = 9001

# Query HTTP Handler.
http_handler_host = "0.0.0.0"
http_handler_port = 8081

tenant_id = "tenant1"
cluster_id = "cluster1"

[storage]
# s3
type = "s3"

[storage.s3]
bucket = "databend"
endpoint_url = "https://s3.amazonaws.com"
access_key_id = "<your-key-id>"
secret_access_key = "<your-access-key>"
```
