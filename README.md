https://carlosmv.hashnode.dev/adding-logging-and-tracing-to-an-axum-app-rust
https://www.zupzup.org/rust-job-runner/
https://github.com/JoeyMckenzie/realworld-rust-axum-sqlx/tree/main
https://fasterthanli.me/articles/remote-development-with-rust-on-fly-io#what-the-heck-is-fly-io-for-even
https://dpc.pw/data-oriented-cleanandhexagonal-architecture-software-in-rust-through-an-example
https://users.rust-lang.org/t/reborrowing-via-a-trait/80039/5



Create
---------------------------------------------------------------------
Success - 201 Created - Return created object
Failure - 400 Invalid request - Return details about the failure
Async fire and forget operation - 202 Accepted - Optionally return url for polling status

Update
---------------------------------------------------------------------
Success - 200 Ok - Return the updated object
Success - 204 NoContent
Failure - 404 NotFound - The targeted entity identifier does not exist
Failure - 400 Invalid request - Return details about the failure
Async fire and forget operation - 202 Accepted - Optionally return url for polling status

Patch
---------------------------------------------------------------------
Success - 200 Ok - Return the patched object
Success - 204 NoContent
Failure - 404 NotFound - The targeted entity identifier does not exist
Failure - 400 Invalid request - Return details about the failure
Async fire and forget operation - 202 Accepted - Optionally return url for polling status

Delete
---------------------------------------------------------------------
Success - 200 Ok - No content
Success - 200 Ok - When element attempting to be deleted does not exist
Async fire and forget operation - 202 Accepted - Optionally return url for polling status

Get
---------------------------------------------------------------------
Success - 200 Ok - With the list of resulting entities matching the search criteria
Success - 200 Ok - With an empty array

Get specific
---------------------------------------------------------------------
Success - 200 Ok - The entity matching the identifier specified is returned as content
Failure - 404 NotFound - No content

Action
---------------------------------------------------------------------
Success - 200 Ok - Return content where appropriate
Success - 204 NoContent
Failure - 400 - Return details about the failure
Async fire and forget operation - 202 Accepted - Optionally return url for polling status

Generic results
---------------------------------------------------------------------
Authorization error 401 Unauthorized
Authentication error 403 Forbidden
For methods not supported 405
Generic server error 500