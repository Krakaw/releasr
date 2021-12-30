<h1 align="center">Welcome to releasr 👋</h1>
<p>
  <a href="https://twitter.com/krakaw\_1" target="_blank">
    <img alt="Twitter: krakaw\_1" src="https://img.shields.io/twitter/follow/krakaw\_1.svg?style=social" />
  </a>
</p>

> Release not tracking on a per environment per semver release.
> 
> Every release note is attached to an environment. 
> Once a note has been completed it won't be shown again. 
> 
> Notes cannot be added to versions that have been completed.  

## Install

```sh
cargo build --release
./target/release/releasr
```

## Usage

```sh
cp .env.sample .env
./releasr
```

### Create Environments
```sh
# Create a dev environment
curl http://localhost:8080/environments \
  -X POST \
  -H 'Content-Type: application/json' \
  -d '{"name": "backend_dev", "version_url": "https://example.com/version.json", "last_deployed_version": 0}'

# Create a prod environment
curl http://localhost:8080/environments \
  -X POST \
  -H 'Content-Type: application/json' \
  -d '{"name": "backend_prod", "version_url": "https://example.com/version.json", "last_deployed_version": 0}'
```

### Attach notes for a version
```sh
# Attach a note to both environments
curl http://localhost:8080/notes \
  -X POST \
  -H 'Content-Type: application/json' \
  -d '{"environment": "backend_*", "version": "1.0.0", "note": "Run initial migrations using `cargo run -- migrations`"}'
```
```json
[
  {
    "id": 1,
    "version": "1.0.0",
    "version_int": 1000000,
    "note": "Run initial migrations using `cargo run -- migrations`",
    "environment": "backend_dev",
    "completed_at": null,
    "created_at": "2021-12-30T11:41:37.866539073Z",
    "modified_at": "2021-12-30T11:41:37.866539073Z"
  },
  {
    "id": 2,
    "version": "1.0.0",
    "version_int": 1000000,
    "note": "Run initial migrations using `cargo run -- migrations`",
    "environment": "backend_prod",
    "completed_at": null,
    "created_at": "2021-12-30T11:41:37.882175630Z",
    "modified_at": "2021-12-30T11:41:37.882175630Z"
  }
]
```
```sh
# Attach a note to a specific environment for another version
curl http://localhost:8080/notes \
  -X POST \
  -H 'Content-Type: application/json' \
  -d '{"environment": "backend_prod", "version": "1.0.1", "note": "Manually delete records from `table_x`"}'
```
```json
[
  {
    "id": 3,
    "version": "1.0.1",
    "version_int": 1000001,
    "note": "Manually delete records from `table_x`",
    "environment": "backend_prod",
    "completed_at": null,
    "created_at": "2021-12-30T11:44:40.673926719Z",
    "modified_at": "2021-12-30T11:44:40.673926719Z"
  }
]
```

### List pending notes
```sh
# To fetch all pending notes
curl http://localhost:8080/notes \
  -X GET \
  -H 'Content-Type: application/json'
```
```json
[
  {
    "id": 1,
    "version": "1.0.0",
    "version_int": 1000000,
    "note": "Run initial migrations using `cargo run -- migrations`",
    "environment": "backend_dev",
    "completed_at": null,
    "created_at": "2021-12-30T11:41:37.866539073Z",
    "modified_at": "2021-12-30T11:41:37.866539073Z"
  },
  {
    "id": 2,
    "version": "1.0.0",
    "version_int": 1000000,
    "note": "Run initial migrations using `cargo run -- migrations`",
    "environment": "backend_prod",
    "completed_at": null,
    "created_at": "2021-12-30T11:41:37.882175630Z",
    "modified_at": "2021-12-30T11:41:37.882175630Z"
  },
  {
    "id": 3,
    "version": "1.0.1",
    "version_int": 1000001,
    "note": "Manually delete records from `table_x`",
    "environment": "backend_prod",
    "completed_at": null,
    "created_at": "2021-12-30T11:44:40.673926719Z",
    "modified_at": "2021-12-30T11:44:40.673926719Z"
  }
]
```
```sh
# To fetch specific pending notes for an environment and maximum version
curl http://localhost:8080/notes?environment=backend_prod&version=1.0.0 \
  -X GET \
  -H 'Content-Type: application/json'
```
```json
[
  {
    "id": 2,
    "version": "1.0.0",
    "version_int": 1000000,
    "note": "Run initial migrations using `cargo run -- migrations`",
    "environment": "backend_prod",
    "completed_at": null,
    "created_at": "2021-12-30T11:41:37.882175630Z",
    "modified_at": "2021-12-30T11:41:37.882175630Z"
  }
]
```

### Complete notes for an environment and version
```sh
# Once a deploy is complete and the notes have been executed, you can complete them.
curl http://localhost:8080/notes \
  -X PATCH \
  -H 'Content-Type: application/json' \
  -d '{"environment": "backend_dev", "version": "1.0.1"}'
```
```json
{
  "completed_count": 1,
  "environment": {
    "last_deployed_version": 1000001,
    "name": "backend_dev",
    "version_url": "https://example.com/version.json"
  }
}
```

### Delete a note permanently
```sh
# Delete a note using its `id`
curl http://localhost:8080/notes/2 \
  -X DELETE \
  -H 'Content-Type: application/json'
```
```json
{
  "id": 2,
  "version": "1.0.0",
  "version_int": 1000000,
  "note": "Run initial migrations using `cargo run -- migrations`",
  "environment": "backend_prod",
  "completed_at": null,
  "created_at": "2021-12-30T11:41:37.882175630Z",
  "modified_at": "2021-12-30T11:41:37.882175630Z"
}
```
## Test

```sh
cargo test
```

## Author

👤 **Krakaw**

* Website: https://krakaw.com
* Twitter: [@krakaw\_1](https://twitter.com/krakaw\_1)
* Github: [@Krakaw](https://github.com/Krakaw)

## Show your support

Give a ⭐️ if this project helped you!

***
_This README was generated with ❤️ by [readme-md-generator](https://github.com/kefranabg/readme-md-generator)_
