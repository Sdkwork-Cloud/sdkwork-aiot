use sdkwork_aiot_contract::{AiotActorRef, AiotRequestContext};
use sdkwork_web_core::{WebRequestContext, WebSubjectType};

/// Maps a framework-resolved [`WebRequestContext`] into the AIoT domain context.
pub fn aiot_context_from_web_request(context: &WebRequestContext) -> Option<AiotRequestContext> {
    let principal = context.principal.as_ref()?;
    let organization_id = principal
        .organization_id()
        .filter(|value| !value.is_empty())
        .unwrap_or("0")
        .to_owned();

    let actor = match principal.subject.subject_type {
        WebSubjectType::User => AiotActorRef::iam_user(principal.user_id()),
        WebSubjectType::Service => AiotActorRef::iam_service(principal.user_id()),
        WebSubjectType::System => AiotActorRef::system(),
        WebSubjectType::ApiKey => AiotActorRef::iam_service(
            principal
                .auth
                .api_key_id
                .as_deref()
                .unwrap_or(principal.user_id()),
        ),
    };

    let mut ctx = AiotRequestContext::new(principal.tenant_id(), organization_id)
        .with_user(principal.user_id())
        .with_actor(actor)
        .with_trace_id(context.request_id.0.clone());

    for permission in &principal.scopes.permission_scope {
        ctx = ctx.with_permission(permission.clone());
    }
    for scope in &principal.scopes.data_scope {
        ctx = ctx.with_data_scope(scope.clone());
    }

    Some(ctx)
}
