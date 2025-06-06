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

use clap::Parser;

use revolut::{
    errors::ApiResult,
    merchant::client::{MerchantAuthenticationBuilder, merchant_client},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Order ID
    #[arg(long)]
    order_id: String,
}

#[tokio::main]
async fn main() -> ApiResult<()> {
    let args = Args::parse();

    let client = merchant_client()
        .with_sandbox_environment()
        .with_authentication(
            MerchantAuthenticationBuilder::default()
                .with_environment_inherited_secret_key("REVOLUT_SECRET_KEY")?
                .build(),
        )
        .build()?;

    println!(
        "{}",
        serde_json::to_string(&client.cancel_order(&args.order_id).await?)?
    );

    Ok(())
}
