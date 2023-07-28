# Workflow config file
By default, Tudo will look for default `tudo.yaml` file in the project root directory, or you can specify workflow file path with flag `--config`.

| Key              | Required | Type     | Default Value | Description                                  |
|------------------|----------|----------|---------------|----------------------------------------------|
| version          | Yes      | String   | '1'           | Workflow config file version                 |
| config           | No       | Map      | None          | Workflow customized config                   |
| jobs             | No       | Map      | None          | Each job consists of the job’s name as a key and a map as a value. A name should be case insensitive unique within a current jobs list    |
| workflows        | Yes      | Map      | None          | Each job consists of the workflow’s name as a key and a map as a value. A name should be case insensitive unique within a current workflows list    |

## Version
**Required**: Yes
**Type**: String
**Default value**: '1'

For now version is simple with single number like `1`, `2`, ..etc. Semantic version is considered to use if needed.

## Config
**Required**: No
**Type**: Map
**Default value**: None

Customize configuration for the workflow.

## Jobs
**Required**: No
**Type**: Map
**Default value**: None

List of defined jobs 

## Workflows
**Required**: Yes
**Type**: Map
**Default value**: None

List of workflows