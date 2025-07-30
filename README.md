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

### Setting up Business API

Follow the instructions at the [Revolut API documentation
site](https://developer.revolut.com/docs/business/business-api).

In order to issue requests in the general case, two settings are
necessary:

1. Refresh token
    1. [How to obtain the refresh token](https://developer.revolut.com/docs/guides/manage-accounts/get-started/make-your-first-api-request#4-exchange-authorization-code-for-access-token)
1. Client assertion
    1. [How to obtain the client assertion](https://developer.revolut.com/docs/guides/manage-accounts/get-started/make-your-first-api-request#4-exchange-authorization-code-for-access-token)

The library will automatically request new access tokens when the
cached one expires, or when it performs the first request in cold
state.

### Some examples

#### List accounts

```shell-session
$ REVOLUT_CLIENT_ASSERTION='<CLIENT_ASSERTION>' REVOLUT_REFRESH_TOKEN='<REFRESH_TOKEN>' just list-accounts
```

### Misc

#### Generate a new access token

Generating a new access token requires the authorization code that was
granted in step [Setting up Business API](#setting-up-business-api)
along with the client assertion.

```shell-session
$ REVOLUT_CLIENT_ASSERTION='<CLIENT_ASSERTION>' REVOLUT_AUTHORIZATION_CODE='<AUTHORIZATION_CODE>' just retrieve-access-token
```

#### Refresh access token

Refreshing the access token requires the refresh token that was
granted in step [Setting up Business API](#setting-up-business-api)
along with the client assertion.

```shell-session
$ REVOLUT_CLIENT_ASSERTION='<CLIENT_ASSERTION>' REVOLUT_AUTHORIZATION_CODE='<AUTHORIZATION_CODE>' just refresh-access-token
```

## Merchant API

### Setting up Merchant API

Follow the instructions at the [Revolut API documentation
site](https://developer.revolut.com/docs/merchant/merchant-api).

In order to initiate requests, you need to provide:

1. Secret key
    1. [How to obtain the secret key](https://developer.revolut.com/docs/merchant/merchant-api#authorization)

### Some examples

In order to communicate with the Revolut Merchant API, you will need
to have [set up the Merchant API](#setting-up-merchant-api).

#### List orders

```shell-session
$ REVOLUT_SECRET_KEY='<SECRET_KEY>' just list-orders
```
