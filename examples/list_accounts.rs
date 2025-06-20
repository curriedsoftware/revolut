/***
 * Copyright (c) 2025 Rafael Fernández López <ereslibre@curried.software>
 *
 * Permission is hereby granted, free of charge, to any person
 * obtaining a copy of this software and associated documentation
 * files (the "Software"), to deal in the Software without
 * restriction, including without limitation the rights to use, copy,
 * modify, merge, publish, distribute, sublicense, and/or sell copies
 * of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be
 * included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 ***/

use revolut::{
    business::client::{BusinessAuthenticationBuilder, business_client},
    errors::ApiResult,
};

#[tokio::main]
async fn main() -> ApiResult<()> {
    let client = business_client()
        .with_sandbox_environment()
        .with_authentication(
            BusinessAuthenticationBuilder::default()
                .with_environment_inherited_client_assertion("REVOLUT_CLIENT_ASSERTION")?
                .with_environment_inherited_refresh_token("REVOLUT_REFRESH_TOKEN")?
                .build(),
        )
        .build()?;

    println!("{}", serde_json::to_string(&client.accounts().await?)?);

    Ok(())
}
