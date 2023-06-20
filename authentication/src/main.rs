// Banco Core (sqlx Sqlite) ->
//      Organization { id, name, active,  }
//      Admin   -> { id, organization_id, email, password, is_default (true or false) } (Has permission to add or remove other admin but other admin cannot remove the default one)
//      Seller  -> { id, organization_id, email, password, created_at, active }
//      Product -> { id, name, amount, description, price, created_at, updated_at }
//      Sales   -> { id, product_id, amount, total_price, salesman_id, created_at, updated_at }
// Banco Authentication (mongodb) ->
//      token -> { account_id, api_key, permissions, created_at, updated_at, expires_at }

fn main() {
    println!("Hello, world!");
}
