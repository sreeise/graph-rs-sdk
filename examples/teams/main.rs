mod create_team;
mod get_teams;

#[tokio::main]
async fn main() {
    get_teams::list_teams().await;
    get_teams::get_teams().await;
    create_team::create_team().await;
}
