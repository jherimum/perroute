use perroute_commons::types::actor::Actor;
use perroute_cqrs::command_bus::bus::CommandBusContext;
use sqlx::PgPool;
use tap::TapFallible;

pub async fn start_context<'ctx, 'a>(
    pool: PgPool,
    actor: &'a Actor,
) -> CommandBusContext<'ctx, 'a> {
    CommandBusContext::begin(pool, actor)
        .await
        .tap_err(|e| tracing::error!("Failed to tsart context: {e}"))
        .unwrap()
}
