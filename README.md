# GCF Resource [![Build Status](https://travis-ci.org/frodenas/gcs-resource.png)](https://travis-ci.org/frodenas/gcs-resource)

Supports checking, fetching, and pushing of Cloud Funtions to Google Cloud.

## Source Configuration

* ``
* `json_key` (*Required.*): the contents of your Google Account JSON Key file to use when accessing the Cloud Functions. Example:
  ```
  json_key: |
    {
      "private_key_id": "...",
      "private_key": "...",
      "client_email": "...",
      "client_id": "...",
      "type": "service_account"
    }
  ```

## Behavior

### `check`: Extract cloud function version info.

WIP

### `in`: Fetch a cloud function from the Google Cloud.

WIP

#### Parameters

### `out`: Upload a cloud function to Google Cloud.

WIP

#### Parameters

## Example Configuration

### Resource Type

### Resource

### Plan

## Development

## Contributing

Refer to the [contributing guidelines][contributing].

## License

Apache License 2.0, see [LICENSE][license].

[contributing]: https://github.com/xtremerui/gcf-resource/blob/master/CONTRIBUTING.md
[gcf]: https://cloud.google.com/functions/
[license]: https://github.com/xtremerui/gcf-resource/blob/master/LICENSE
