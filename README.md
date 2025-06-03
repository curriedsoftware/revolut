# Revolut

Revolut API integration for Rust.

> [!NOTE]
> **This crate has no relationship with Revolut Ltd,** as such, it is
> an unofficial implementation that uses the documented public API
> endpoints.

> [!CAUTION]
> This crate is in its very early stages and is expected to be
> incomplete and might contain critical bugs. Do not use it in a
> production environment in its current state.

## Business API

### Setting up

Follow the instructions at the [Revolut API documentation
site](https://developer.revolut.com/docs/business/business-api).

In order to issue requests in the general case, two settings are
necessary:

1. Refresh token
    1. [How to obtain the refresh token](https://developer.revolut.com/docs/guides/manage-accounts/get-started/make-your-first-api-request#4-exchange-authorization-code-for-access-token)
2. Client assertion
    1. [How to obtain the client assertion](https://developer.revolut.com/docs/guides/manage-accounts/get-started/make-your-first-api-request#4-exchange-authorization-code-for-access-token)

The library will automatically request new access tokens when the
cached one expires, or when it performs the first request in cold
state.

#### List accounts

```shell-session
$ REVOLUT_CLIENT_ASSERTION='<CLIENT_ASSERTION>' REVOLUT_REFRESH_TOKEN='<REFRESH_TOKEN>' just list-accounts
```

### Misc

#### Generate a new access token

```shell-session
$ REVOLUT_CLIENT_ASSERTION='<CLIENT_ASSERTION>' REVOLUT_AUTHORIZATION_CODE='<AUTHORIZATION_CODE>' just retrieve-access-token
```
