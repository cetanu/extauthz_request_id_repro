Prerequisites
-------------

* Docker
* Docker-compose
* A http client


Steps
-----

The following will stand up a stack that reproduces the issue.
The request made against envoy will cause envoy to check the request
using the configured extauthz service.
The service will print the id, extracted from the httpRequest, and then
allow the request.
Envoy will receive the OK, proxy the request to the httpbin backend, which
will show the request headers that Envoy received (if you used the `/headers` path).

1. `docker-compose up --build extauthz envoy`
2. curl http://localhost/headers
